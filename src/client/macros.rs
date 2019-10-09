#[macro_use]
macro_rules! register_client {
    ( $name:ident, $($helper:ident => $value:expr,)* ) => {
        $( register_helper!($helper, $value); )*

        pub struct $name<'a, I> {
            client: &'a Graph,
            ident: PhantomData<I>,
        }

        impl<'a, I> $name<'a, I> {
            pub fn new(client: &'a Graph) -> $name<'a, I> {

                $(
                    client.registry()
                        .register_helper(stringify!($helper), Box::new($helper));
                )*

                $name {
                    client,
                    ident: PhantomData,
                }
            }
        }
    };

    ( $name:ident, $($helper:ident => $value:expr, $value2:expr, $identity:expr,)* ) => {
        pub struct $name<'a, I> {
            client: &'a Graph,
            ident: PhantomData<I>,
        }

        impl<'a, I> $name<'a, I> {
            pub fn new(client: &'a Graph) -> $name<'a, I> {
                let ident = client.ident();
                $(
                    client.registry().register_helper(
                        stringify!($helper),
                        Box::new(
                            move |_: &Helper,
                                _: &Handlebars,
                                _: &Context,
                                _: &mut RenderContext,
                                out: &mut dyn Output|
                            -> HelperResult {
                                if ident.ne(&$identity) {
                                    out.write($value)?;
                                } else {
                                    out.write($value2)?;
                                }
                                Ok(())
                            },
                        ),
                    );
                )*

                $name {
                    client,
                    ident: PhantomData,
                }
            }
        }
    };
}

#[macro_use]
macro_rules! register_ident_client {
    ( $name:ident, $($helper:ident => $value:expr,)* ) => {
        $( register_helper!($helper, $value); )*

        pub struct $name<'a, I> {
            client: &'a Graph,
            ident: PhantomData<I>,
            id: String,
        }

        impl<'a, I> $name<'a, I> {
            pub fn new(id: &str, client: &'a Graph) -> $name<'a, I> {
                $(
                    client.registry()
                        .register_helper(stringify!($helper), Box::new($helper));
                )*
                let id_string = id.to_string();
                client.registry()
                    .register_helper("RID",
                    Box::new(move |
                        _: &Helper,
                        _: &Handlebars,
                        _: &Context,
                        _: &mut RenderContext,
                        out: &mut dyn Output|
                        -> HelperResult {
                            out.write(&id_string)?;
                            Ok(())
                    }));

                $name {
                    client,
                    ident: PhantomData,
                    id: id.into(),
                }
            }

            fn set_path(&self) {
                let ident = self.client.ident();
                if self.client.ident().eq(&Ident::Me) {
                    self.client
                        .builder()
                        .as_mut()
                        .extend_path(&[ident.as_ref()]);
                } else {
                    self.client
                        .builder()
                        .as_mut()
                        .extend_path(&[ident.as_ref(), self.id.as_str()]);
                }
            }

            pub fn drive(&'a self) -> DriveRequest<'a, I> {
                self.set_path();
                DriveRequest::new(self.client)
            }

            pub fn mail(&'a self) -> MailRequest<'a, I> {
                self.set_path();
                MailRequest::new(self.client)
            }

            pub fn calendar(&'a self) -> CalendarRequest<'a, I> {
                self.set_path();
                CalendarRequest::new(self.client)
            }

            pub fn onenote(&'a self) -> OnenoteRequest<'a, I> {
                self.set_path();
                OnenoteRequest::new(self.client)
            }

            pub fn contacts(&'a self) -> ContactsRequest<'a, I> {
                self.set_path();
                ContactsRequest::new(self.client)
            }
        }
    }
}

#[macro_use]
macro_rules! register_helper {
    ( $name:ident, $value:expr ) => {
        fn $name(
            _: &Helper,
            _: &Handlebars,
            _: &Context,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> HelperResult {
            out.write($value)?;
            Ok(())
        }
    };
}

#[macro_use]
macro_rules! render_path {
    ($client:expr, $template:expr) => {
        let path = $client
            .registry()
            .render_template($template, &serde_json::json!({}))
            .unwrap();
        let mut vec: Vec<&str> = path.split("/").collect();
        vec.retain(|s| !s.is_empty());
        $client.builder().as_mut().extend_path(&vec);
    };

    ($client:expr, $template:expr, $json:expr) => {
        let path = $client
            .registry()
            .render_template($template, $json)
            .unwrap();
        let mut vec: Vec<&str> = path.split("/").collect();
        vec.retain(|s| !s.is_empty());
        $client.builder().as_mut().extend_path(&vec);
    };

    ($client:expr, $template:expr, $json:expr, $last:expr ) => {
        let path = $client
            .registry()
            .render_template($template, $json)
            .unwrap();
        let mut vec: Vec<&str> = path.split("/").collect();
        vec.retain(|s| !s.is_empty());
        vec.extend($last);
        $client.builder().as_mut().extend_path(&vec);
    };
}

#[macro_use]
macro_rules! register_method {
    ( $name:ident, $T:ty => $template:expr, $m:expr ) => {
      pub fn $name(&'a self) -> IntoResponse<'a, I, $T> {
        self.client.builder()
            .set_method($m);

        render_path!(
            self.client,
            $template
        );
        IntoResponse::new(self.client)
      }
    };

    ( | $name:ident, $T:ty => $template:expr, $m:expr ) => {
      pub fn $name<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, I, $T> {
        self.client.builder()
            .set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        IntoResponse::new(self.client)
      }
    };

    ( || $name:ident, $T:ty => $template:expr, $m:expr ) => {
      pub fn $name<S: AsRef<str>>(&'a self, id: S, id2: S) -> IntoResponse<'a, I, $T> {
        self.client.builder()
            .set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref(), "id2": id2.as_ref() })
        );
        IntoResponse::new(self.client)
      }
    };

    ( ||| $name:ident, $T:ty => $template:expr, $m:expr ) => {
      pub fn $name<S: AsRef<str>>(&'a self, id: S, id2: S, id3: S) -> IntoResponse<'a, I, $T> {
        self.client.builder()
            .set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref(), "id2": id2.as_ref(), "id3": id3.as_ref() })
        );
        IntoResponse::new(self.client)
      }
    };

    ( [ $name:ident, $T:ty => $template:expr, $m:expr ] ) => {
      pub fn $name<B: serde::Serialize>(&'a self, body: &B) -> IntoResponse<'a, I, $T> {
        self.client.builder()
            .set_method($m)
            .set_body(serde_json::to_string_pretty(body).unwrap());

        render_path!(
            self.client,
            $template
        );
        IntoResponse::new(self.client)
      }
    };

    ( [ | $name:ident, $T:ty => $template:expr, $m:expr ] ) => {
      pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, id: S, body: &B) -> IntoResponse<'a, I, $T> {
        self.client.builder()
            .set_method($m)
            .set_body(serde_json::to_string_pretty(body).unwrap());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        IntoResponse::new(self.client)
      }
    };

    ( [ || $name:ident, $T:ty => $template:expr, $m:expr ] ) => {
      pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, id: S, id2: S, body: &B) -> IntoResponse<'a, I, $T> {
        self.client.builder()
            .set_method($m)
            .set_body(serde_json::to_string_pretty(body).unwrap());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref(), "id2": id2.as_ref() })
        );
        IntoResponse::new(self.client)
      }
    };
}

#[macro_use]
macro_rules! get {
    ( $name:ident, $T:ty => $template:expr ) => {
        register_method!( $name, $T => $template, Method::GET );
    };

    ( [ $name:ident, $T:ty => $template:expr ] ) => {
       register_method!( [ $name, $T => $template, Method::GET ] );
    };

    ( | $name:ident, $T:ty => $template:expr ) => {
        register_method!( | $name, $T => $template, Method::GET);
    };

    ( || $name:ident, $T:ty => $template:expr ) => {
        register_method!( || $name, $T => $template, Method::GET);
    };

    ( ||| $name:ident, $T:ty => $template:expr ) => {
        register_method!( ||| $name, $T => $template, Method::GET);
    };

    ( [ $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ $name, $T => $template, Method::GET ] );
    };

    ( [ | $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ | $name, $T => $template, Method::GET ] );
    };

    ( [ || $name:ident, $T:ty, $template:expr ] ) => {
        register_method!( [ || $name, $T => $template, Method::GET ]);
    };
}

#[macro_use]
macro_rules! post {
    ( $name:ident, $T:ty => $template:expr ) => {
        register_method!( $name, $T => $template, Method::POST );
    };

    ( [ $name:ident, $T:ty => $template:expr ] ) => {
       register_method!( [ $name, $T => $template, Method::POST ] );
    };

    ( | $name:ident, $T:ty => $template:expr ) => {
        register_method!( | $name, $T => $template, Method::POST);
    };

    ( || $name:ident, $T:ty => $template:expr ) => {
        register_method!( || $name, $T => $template, Method::POST);
    };

    ( ||| $name:ident, $T:ty => $template:expr ) => {
        register_method!( ||| $name, $T => $template, Method::POST);
    };

    ( [ $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ $name, $T => $template, Method::POST ] );
    };

    ( [ | $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ | $name, $T => $template, Method::POST ] );
    };

    ( [ || $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ || $name, $T => $template, Method::POST ]);
    };
}

#[macro_use]
macro_rules! patch {
    ( $name:ident, $T:ty => $template:expr ) => {
        register_method!( $name, $T => $template, Method::PATCH );
    };

    ( [ $name:ident, $T:ty => $template:expr ] ) => {
       register_method!( [ $name, $T => $template, Method::PATCH ] );
    };

    ( | $name:ident, $T:ty => $template:expr ) => {
        register_method!( | $name, $T => $template, Method::PATCH);
    };

    ( || $name:ident, $T:ty => $template:expr ) => {
        register_method!( || $name, $T => $template, Method::PATCH);
    };

    ( ||| $name:ident, $T:ty => $template:expr ) => {
        register_method!( ||| $name, $T => $template, Method::PATCH);
    };

    ( [ $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ $name, $T => $template, Method::PATCH ] );
    };

    ( [ | $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ | $name, $T => $template, Method::PATCH ] );
    };

    ( [ || $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ || $name, $T => $template, Method::PATCH ]);
    };
}

#[allow(unused_macros)]
#[macro_use]
macro_rules! put {
    ( $name:ident, $T:ty => $template:expr ) => {
        register_method!( $name, $T => $template, Method::PUT );
    };

    ( [ $name:ident, $T:ty => $template:expr ] ) => {
       register_method!( [ $name, $T => $template, Method::PUT ] );
    };

    ( | $name:ident, $T:ty => $template:expr ) => {
        register_method!( | $name, $T => $template, Method::PUT);
    };

    ( || $name:ident, $T:ty => $template:expr ) => {
        register_method!( || $name, $T => $template, Method::PUT);
    };

    ( ||| $name:ident, $T:ty => $template:expr ) => {
        register_method!( ||| $name, $T => $template, Method::PUT);
    };

    ( [ $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ $name, $T => $template, Method::PUT ] );
    };

    ( [ | $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ | $name, $T => $template, Method::PUT ] );
    };

    ( [ || $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ || $name, $T => $template, Method::PUT ]);
    };
}

#[macro_use]
macro_rules! delete {
    ( $name:ident, $T:ty => $template:expr ) => {
        register_method!( $name, $T => $template, Method::DELETE );
    };

    ( [ $name:ident, $T:ty => $template:expr ] ) => {
       register_method!( [ $name, $T => $template, Method::DELETE ] );
    };

    ( | $name:ident, $T:ty => $template:expr ) => {
        register_method!( | $name, $T => $template, Method::DELETE);
    };

    ( || $name:ident, $T:ty => $template:expr ) => {
        register_method!( || $name, $T => $template, Method::DELETE);
    };

    ( ||| $name:ident, $T:ty => $template:expr ) => {
        register_method!( ||| $name, $T => $template, Method::DELETE);
    };

    ( [ $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ $name, $T => $template, Method::DELETE ] );
    };

    ( [ | $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ | $name, $T => $template, Method::DELETE ] );
    };

    ( [ || $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ || $name, $T => $template, Method::DELETE ]);
    };
}
