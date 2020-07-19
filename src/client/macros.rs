#[macro_use]
macro_rules! register_client {
    ( $name:ident, $($helper:ident => $value:expr,)* ) => {
        $( register_helper!($helper, $value); )*

        pub struct $name<'a, Client> {
            client: &'a Graph<Client>,
        }

        impl<'a, Client> $name<'a, Client> where Client: crate::http::RequestClient {
            pub fn new(client: &'a Graph<Client>) -> $name<'a, Client> {

                $(
                    client.request().registry(|r| {
                        r.register_helper(stringify!($helper), Box::new($helper));
                    });
                )*

                $name {
                    client,
                }
            }
        }
    };

    ( $name:ident, $($helper:ident => $value:expr, $value2:expr, $identity:expr,)* ) => {
        pub struct $name<'a, Client> {
            client: &'a Graph<Client>,
        }

        impl<'a, Client> $name<'a, Client> where Client: crate::http::RequestClient {
            pub fn new(client: &'a Graph<Client>) -> $name<'a, Client> {
                let ident = client.ident();
                $(
                    client.request().registry(|r| {
                        r.register_helper(
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
                    });
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

        pub struct $name<'a, Client> {
            client: &'a Graph<Client>,
        }

        impl<'a, Client> $name<'a, Client,> where Client: crate::http::RequestClient  {
            pub fn new(id: &str, client: &'a Graph<Client>) -> $name<'a, Client> {
                $(
                    client.request().registry(|r| {
                        r.register_helper(stringify!($helper), Box::new($helper));
                    });
                )*

                client.request().registry(|r| {
                    let id_string = id.to_string();
                    r.register_helper("RID",
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
                });

                $name {
                    client,
                }
            }
        }
    };

    ( $name:ident, $($helper:ident => $value:expr,)* ) => {
        $( register_helper!($helper, $value); )*

        pub struct $name<'a, Client> {
            client: &'a Graph<Client>,
            id: String,
        }

        impl<'a, Client> $name<'a, Client> where Client: crate::http::RequestClient {
            pub fn new(id: &str, client: &'a Graph<Client>) -> $name<'a, Client> {
                $(
                    client.request().registry()
                        .register_helper(stringify!($helper), Box::new($helper));
                )*
                client.request().registry(|r| {
                    let id_string = id.to_string();
                    r.register_helper("RID",
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
                });

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
                        .extend_path(&[ident.as_ref()]);

                } else {
                   self.client
                        .request()
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
            .request()
            .render_template($template, &serde_json::json!({}));
        let mut vec: Vec<&str> = path.split("/").collect();
        vec.retain(|s| !s.is_empty());
        $client.request().url_mut(|url| {
            url.extend_path(&vec);
        });
    };

    ($client:expr, $template:expr, $json:expr) => {
        let path = $client.request().render_template($template, $json);
        let mut vec: Vec<&str> = path.split("/").collect();
        vec.retain(|s| !s.is_empty());
        $client.request().url_mut(|url| {
            url.extend_path(&vec);
        });
    };

    ($client:expr, $template:expr, $json:expr, $last:expr ) => {
        let path = $client.request().render_template($template, $json);
        let mut vec: Vec<&str> = path.split("/").collect();
        vec.retain(|s| !s.is_empty());
        vec.extend($last);
        $client.request().url_mut(|url| {
            url.extend_path(&vec);
        });
    };
}

#[macro_use]
macro_rules! register_method {
    ( $name:ident, $T:ty => $template:expr, $m:expr ) => {
      pub fn $name(&'a self) -> IntoResponse<'a, $T, Client>
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
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_body(serde_json::to_string_pretty(body).unwrap());

        render_path!(
            self.client,
            $template
        );
        IntoResponse::new(self.client)
      }
    };

    ( [ | $name:ident, $T:ty => $template:expr, $m:expr ] ) => {
      pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, id: S, body: &B) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_body(serde_json::to_string_pretty(body).unwrap());

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
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_body(serde_json::to_string_pretty(body).unwrap());

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
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_body(serde_json::to_string_pretty(body).unwrap());

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
        self.client.request()
            .set_request(vec![
                crate::http::RequestAttribute::Method(Method::GET),
                crate::http::RequestAttribute::Download(directory.as_ref().to_path_buf()),
                crate::http::RequestAttribute::RequestType(GraphRequestType::Redirect),
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        self.client.request().download()
      }
    };
}

#[macro_use]
macro_rules! register_async_download {
    ( | $name:ident, $T:ty => $template:expr ) => {
      pub async fn $name<S: AsRef<str>, P: AsRef<Path>>(&'a self, id: S, directory: P) -> $T {
        self.client.request()
            .set_request(vec![
                crate::http::RequestAttribute::Method(Method::GET),
                crate::http::RequestAttribute::Download(directory.as_ref().to_path_buf()),
                crate::http::RequestAttribute::RequestType(GraphRequestType::Redirect),
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        self.client.request().download().await
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

#[macro_use]
macro_rules! async_download {
    ( | $name:ident, $T:ty => $template:expr ) => {
        register_async_download!( | $name, $T => $template );
    };
}

#[macro_use]
macro_rules! method_impl {
   ( $impl_name:ident,
    $( get!($name_get:ident, $T_get:ty => $template_get:expr); )*
    $( post!($name_post:ident, $T_post:ty => $template_post:expr); )*
    $( put!($name_put:ident, $T_put:ty => $template_put:expr); )*
    $( patch!($name_patch:ident, $T_patch:ty => $template_patch:expr); )*
    $( delete!($name_delete:ident, $T_delete:ty => $template_delete:expr); )*

    $( get!( | $name_get1:ident, $T_get1:ty => $template_get1:expr); )*
    $( post!( | $name_post1:ident, $T_post1:ty => $template_post1:expr); )*
    $( put!( | $name_put1:ident, $T_put1:ty => $template_put1:expr); )*
    $( patch!( | $name_patch1:ident, $T_patch1:ty => $template_patch1:expr); )*
    $( delete!( | $name_delete1:ident, $T_delete1:ty => $template_delete1:expr); )*

    $( get!( || $name_get2:ident, $T_get2:ty => $template_get2:expr); )*
    $( post!( || $name_post2:ident, $T_post2:ty => $template_post2:expr); )*
    $( put!( || $name_put2:ident, $T_put2:ty => $template_put2:expr); )*
    $( patch!( || $name_patch2:ident, $T_patch2:ty => $template_patch2:expr); )*
    $( delete!( || $name_delete2:ident, $T_delete2:ty => $template_delete2:expr); )*

    $( get!( ||| $name_get3:ident, $T_get3:ty => $template_get3:expr); )*
    $( post!( ||| $name_post3:ident, $T_post3:ty => $template_post3:expr); )*
    $( put!( ||| $name_put3:ident, $T_put3:ty => $template_put3:expr); )*
    $( patch!( ||| $name_patch3:ident, $T_patch3:ty => $template_patch3:expr); )*
    $( delete!( ||| $name_delete3:ident, $T_delete3:ty => $template_delete3:expr); )*

    $( get!( [ $name_get1b:ident, $T_get1b:ty => $template_get1b:expr ] ); )*
    $( post!( [ $name_post1b:ident, $T_post1b:ty => $template_post1b:expr ] ); )*
    $( put!( [ $name_put1b:ident, $T_put1b:ty => $template_put1b:expr ] ); )*
    $( patch!( [ $name_patch1b:ident, $T_patch1b:ty => $template_patch1b:expr ] ); )*
    $( delete!( [ $name_delete1b:ident, $T_delete1b:ty => $template_delete1b:expr ] ); )*

    $( get!( [ | $name_get2b:ident, $T_get2b:ty => $template_get2b:expr ] ); )*
    $( post!( [ | $name_post2b:ident, $T_post2b:ty => $template_post2b:expr ] ); )*
    $( put!( [ | $name_put2b:ident, $T_put2b:ty => $template_put2b:expr ] ); )*
    $( patch!( [ | $name_patch2b:ident, $T_patch2b:ty => $template_patch2b:expr ] ); )*
    $( delete!( [ | $name_delete2b:ident, $T_delete2b:ty => $template_delete2b:expr ] ); )*

    $( get!( [ || $name_get3b:ident, $T_get3b:ty => $template_get3b:expr ] ); )*
    $( post!( [ || $name_post3b:ident, $T_post3b:ty => $template_post3b:expr ] ); )*
    $( put!( [ || $name_put3b:ident, $T_put3b:ty => $template_put3b:expr ] ); )*
    $( patch!( [ || $name_patch3b:ident, $T_patch3b:ty => $template_patch3b:expr ] ); )*
    $( delete!( [ || $name_delete3b:ident, $T_delete3b:ty => $template_delete3b:expr ] ); )*

     $( get!( [ ||| $name_get4b:ident, $T_get4b:ty => $template_get4b:expr ] ); )*
     $( post!( [ ||| $name_post4b:ident, $T_post4b:ty => $template_post4b:expr ] ); )*
     $( put!( [ ||| $name_put4b:ident, $T_put4b:ty => $template_put4b:expr ] ); )*
     $( patch!( [ ||| $name_patch4b:ident, $T_patch4b:ty => $template_patch4b:expr ] ); )*
     $( delete!( [ ||| $name_delete4b:ident, $T_delete4b:ty => $template_delete4b:expr ] ); )*

    ) => {
        impl<'a, Client> $impl_name<'a, Client> where Client: crate::http::RequestClient {
            $( get!($name_get, $T_get => $template_get); )*
            $( post!($name_post, $T_post => $template_post); )*
            $( put!($name_put, $T_put => $template_put); )*
            $( patch!($name_patch, $T_patch => $template_patch); )*
            $( delete!($name_delete, $T_delete => $template_delete); )*

            $( get!( | $name_get1, $T_get1 => $template_get1); )*
            $( post!( | $name_post1, $T_post1 => $template_post1); )*
            $( put!( | $name_put1, $T_put1 => $template_put1); )*
            $( patch!( | $name_patch1, $T_patch1 => $template_patch1); )*
            $( delete!( | $name_delete1, $T_delete1 => $template_delete1); )*

            $( get!( || $name_get2, $T_get2 => $template_get2); )*
            $( post!( || $name_post2, $T_post2 => $template_post2); )*
            $( put!( || $name_put2, $T_put2 => $template_put2); )*
            $( patch!( || $name_patch2, $T_patch2 => $template_patch2); )*
            $( delete!( || $name_delete2, $T_delete2 => $template_delete2); )*

            $( get!( ||| $name_get3, $T_get3 => $template_get3); )*
            $( post!( ||| $name_post3, $T_post3 => $template_post3); )*
            $( put!( ||| $name_put3, $T_put3 => $template_put3); )*
            $( patch!( ||| $name_patch3, $T_patch3 => $template_patch3); )*
            $( delete!( ||| $name_delete3, $T_delete3 => $template_delete3); )*

            $( get!( [ $name_get1b, $T_get1b => $template_get1b ] ); )*
            $( post!( [ $name_post1b, $T_post1b => $template_post1b ] ); )*
            $( put!( [ $name_put1b, $T_put1b => $template_put1b ] ); )*
            $( patch!( [ $name_patch1b, $T_patch1b => $template_patch1b ] ); )*
            $( delete!( [ $name_delete1b, $T_delete1b => $template_delete1b ] ); )*

            $( get!( [ | $name_get2b, $T_get2b => $template_get2b ] ); )*
            $( post!( [ | $name_post2b, $T_post2b => $template_post2b ] ); )*
            $( put!( [ | $name_put2b, $T_put2b => $template_put2b ] ); )*
            $( patch!( [ | $name_patch2b, $T_patch2b => $template_patch2b ] ); )*
            $( delete!( [ | $name_delete2b, $T_delete2b => $template_delete2b ] ); )*

            $( get!( [ || $name_get3b, $T_get3b => $template_get3b ] ); )*
            $( post!( [ || $name_post3b, $T_post3b => $template_post3b ] ); )*
            $( put!( [ || $name_put3b, $T_put3b => $template_put3b ] ); )*
            $( patch!( [ || $name_patch3b, $T_patch3b => $template_patch3b ] ); )*
            $( delete!( [ || $name_delete3b, $T_delete3b => $template_delete3b ] ); )*

            $( get!( [ ||| $name_get4b, $T_get4b => $template_get4b ] ); )*
            $( post!( [ ||| $name_post4b, $T_post4b => $template_post4b ] ); )*
            $( put!( [ ||| $name_put4b, $T_put4b => $template_put4b ] ); )*
            $( patch!( [ ||| $name_patch4b, $T_patch4b => $template_patch4b ] ); )*
            $( delete!( [ ||| $name_delete4b, $T_delete4b => $template_delete4b ] ); )*
        }
    };
}
