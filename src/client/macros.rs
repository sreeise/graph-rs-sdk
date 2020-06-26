#[macro_use]
macro_rules! register_client {
    ( $name:ident, $($helper:ident => $value:expr,)* ) => {
        $( register_helper!($helper, $value); )*

        pub struct $name<'a, Client> where Client: crate::http::RequestClient {
            client: &'a Graph<Client>,
        }

        impl<'a, Client> $name<'a, Client> where Client: crate::http::RequestClient {
            pub fn new(client: &'a Graph<Client>) -> $name<'a, Client> {

                $(
                    client.registry()
                        .register_helper(stringify!($helper), Box::new($helper));
                )*

                $name {
                    client,
                }
            }
        }
    };

    ( $name:ident, $($helper:ident => $value:expr, $value2:expr, $identity:expr,)* ) => {
        pub struct $name<'a, Client> where Client: crate::http::RequestClient {
            client: &'a Graph<Client>,
        }

        impl<'a, Client> $name<'a, Client> where Client: crate::http::RequestClient {
            pub fn new(client: &'a Graph<Client>) -> $name<'a, Client> {
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
                }
            }
        }
    };
}

#[macro_use]
macro_rules! register_ident_client {
    ( $name:ident, $($helper:ident => $value:expr,)* ()) => {
        $( register_helper!($helper, $value); )*

        pub struct $name<'a, Client> where Client: crate::http::RequestClient {
            client: &'a Graph<Client>,
        }

        impl<'a, Client> $name<'a, Client,> where Client: crate::http::RequestClient  {
            pub fn new(id: &str, client: &'a Graph<Client>) -> $name<'a, Client> {
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
                }
            }
        }
    };

    ( $name:ident, $($helper:ident => $value:expr,)* ) => {
        $( register_helper!($helper, $value); )*

        pub struct $name<'a, Client> where Client: crate::http::RequestClient {
            client: &'a Graph<Client>,
            id: String,
        }

        impl<'a, Client> $name<'a, Client> where Client: crate::http::RequestClient {
            pub fn new(id: &str, client: &'a Graph<Client>) -> $name<'a, Client> {
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
                    id: id.into(),
                }
            }

            fn set_path(&self) {
                let ident = self.client.ident();
                if self.client.ident().eq(&Ident::Me) {
                    self.client
                        .request()
                        .as_mut()
                        .extend_path(&[ident.as_ref()]);
                } else {
                    self.client
                        .request()
                        .as_mut()
                        .extend_path(&[ident.as_ref(), self.id.as_str()]);
                }
            }

            pub fn drive(&'a self) -> DriveRequest<'a, Client> {
                self.set_path();
                DriveRequest::new(self.client)
            }

            pub fn mail(&'a self) -> MailRequest<'a, Client> {
                self.set_path();
                MailRequest::new(self.client)
            }

            pub fn calendar(&'a self) -> CalendarRequest<'a, Client> {
                self.set_path();
                CalendarRequest::new(self.client)
            }

            pub fn onenote(&'a self) -> OnenoteRequest<'a, Client> {
                self.set_path();
                OnenoteRequest::new(self.client)
            }

            pub fn contacts(&'a self) -> ContactsRequest<'a, Client> {
                self.set_path();
                ContactsRequest::new(self.client)
            }

            pub fn attachments(&'a self) -> AttachmentRequest<'a, Client> {
                self.set_path();
                AttachmentRequest::new(self.client)
            }
        }
    };
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
        $client.request().as_mut().extend_path(&vec);
    };

    ($client:expr, $template:expr, $json:expr) => {
        let path = $client
            .registry()
            .render_template($template, $json)
            .unwrap();
        let mut vec: Vec<&str> = path.split("/").collect();
        vec.retain(|s| !s.is_empty());
        $client.request().as_mut().extend_path(&vec);
    };

    ($client:expr, $template:expr, $json:expr, $last:expr ) => {
        let path = $client
            .registry()
            .render_template($template, $json)
            .unwrap();
        let mut vec: Vec<&str> = path.split("/").collect();
        vec.retain(|s| !s.is_empty());
        vec.extend($last);
        $client.request().as_mut().extend_path(&vec);
    };
}

#[macro_use]
macro_rules! register_method {
    ( $name:ident, $T:ty => $template:expr, $m:expr ) => {
      pub fn $name(&'a self) -> IntoResponse<'a, $T, Client>
        where Client: crate::http::RequestClient
      {
        self.client.request()
            .set_method($m);

        render_path!(
            self.client,
            $template
        );
        IntoResponse::new(self.client)
      }
    };

    ( | $name:ident, $T:ty => $template:expr, $m:expr ) => {
      pub fn $name<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, $T, Client>
        where Client: crate::http::RequestClient
      {
        self.client.request()
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
      pub fn $name<S: AsRef<str>>(&'a self, id: S, id2: S) -> IntoResponse<'a, $T, Client>
        where Client: crate::http::RequestClient
      {
        self.client.request()
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
      pub fn $name<S: AsRef<str>>(&'a self, id: S, id2: S, id3: S) -> IntoResponse<'a, $T, Client>
        where Client: crate::http::RequestClient
      {
        self.client.request()
            .set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref(), "id2": id2.as_ref(), "id3": id3.as_ref() })
        );
        IntoResponse::new(self.client)
      }
    };

    ( |||| $name:ident, $T:ty => $template:expr, $m:expr ) => {
      pub fn $name<S: AsRef<str>>(&'a self, id: S, id2: S, id3: S, id4: S) -> IntoResponse<'a, $T, Client>
        where Client: crate::http::RequestClient
      {
        self.client.request()
            .set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({
                "id": id.as_ref(),
                "id2": id2.as_ref(),
                "id3": id3.as_ref(),
                "id4": id4.as_ref()
            })
        );
        IntoResponse::new(self.client)
      }
    };

    ( [ $name:ident, $T:ty => $template:expr, $m:expr ] ) => {
      pub fn $name<B: serde::Serialize>(&'a self, body: &B) -> IntoResponse<'a, $T, Client>
        where Client: crate::http::RequestClient
      {
        self.client.request()
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
      pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, id: S, body: &B) -> IntoResponse<'a, $T, Client>
        where Client: crate::http::RequestClient
      {
        self.client.request()
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
      pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, id: S, id2: S, body: &B) -> IntoResponse<'a, $T, Client>
        where Client: crate::http::RequestClient
      {
        self.client.request()
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

    ( [ ||| $name:ident, $T:ty => $template:expr, $m:expr ] ) => {
      pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, id: S, id2: S, id3: S, body: &B) -> IntoResponse<'a, $T, Client>
        where Client: crate::http::RequestClient
      {
        self.client.request()
            .set_method($m)
            .set_body(serde_json::to_string_pretty(body).unwrap());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref(), "id2": id2.as_ref(), "id3": id3.as_ref() })
        );
        IntoResponse::new(self.client)
      }
    };
}

#[macro_use]
macro_rules! register_download {
    ( | $name:ident, $T:ty => $template:expr ) => {
      pub fn $name<S: AsRef<str>, P: AsRef<Path>>(&'a self, id: S, directory: P) -> $T {
        let mut request = self.client.request();
        request.set_method(reqwest::Method::GET)
            .set_download_dir(directory.as_ref())
            .set_request_type(GraphRequestType::Redirect);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        request.download()
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

    ( |||| $name:ident, $T:ty => $template:expr ) => {
        register_method!( |||| $name, $T => $template, Method::GET);
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

    ( [ ||| $name:ident, $T:ty, $template:expr ] ) => {
        register_method!( [ ||| $name, $T => $template, Method::GET ]);
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

    ( |||| $name:ident, $T:ty => $template:expr ) => {
        register_method!( |||| $name, $T => $template, Method::POST);
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

    ( [ ||| $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ ||| $name, $T => $template, Method::POST ]);
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

    ( |||| $name:ident, $T:ty => $template:expr ) => {
        register_method!( |||| $name, $T => $template, Method::PATCH);
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

    ( [ ||| $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ ||| $name, $T => $template, Method::PATCH ]);
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

    ( |||| $name:ident, $T:ty => $template:expr ) => {
        register_method!( |||| $name, $T => $template, Method::PUT);
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

    ( [ ||| $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ ||| $name, $T => $template, Method::PUT ]);
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

    ( |||| $name:ident, $T:ty => $template:expr ) => {
        register_method!( |||| $name, $T => $template, Method::DELETE);
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

    ( [ ||| $name:ident, $T:ty => $template:expr ] ) => {
        register_method!( [ ||| $name, $T => $template, Method::DELETE ]);
    };
}

#[macro_use]
macro_rules! download {
    ( | $name:ident, $T:ty => $template:expr ) => {
        register_download!( | $name, $T => $template );
    };
}
