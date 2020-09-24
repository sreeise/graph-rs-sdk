#[macro_use]
macro_rules! register_client {
    ( $name:ident, $($helper:ident => $value:expr,)* ) => {
        $( register_helper!($helper, $value); )*

        pub struct $name<'a, Client> {
            client: &'a Graph<Client>,
        }

        impl<'a, Client> $name<'a, Client> where Client: graph_http::RequestClient {
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

        impl<'a, Client> $name<'a, Client> where Client: graph_http::RequestClient {
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

        impl<'a, Client> $name<'a, Client,> where Client: graph_http::RequestClient  {
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

        impl<'a, Client> $name<'a, Client> where Client: graph_http::RequestClient {
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

            pub fn drive(&'a self) -> DrivesRequest<'a, Client> {
                self.set_path();
                DrivesRequest::new(self.client)
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
        IntoResponse::new(&self.client.request)
      }
    };

   ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 0, has_body: false } ) => {
      pub fn $name(&'a self) -> IntoResponse<'a, $T, Client>
      {
        self.client.request()
            .set_method($m);

        render_path!(
            self.client,
            $template
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 0, has_body: false } ) => {
      #[doc = $doc]
      pub fn $name(&'a self) -> IntoResponse<'a, $T, Client>
      {
        self.client.request()
            .set_method($m);

        render_path!(
            self.client,
            $template
        );
        IntoResponse::new(&self.client.request)
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
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 1, has_body: false } ) => {
      pub fn $name<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, $T, Client>
      {
        self.client.request()
            .set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({
                "id": id.as_ref()
            })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 1, has_body: false } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, $T, Client>
      {
        self.client.request()
            .set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({
                "id": id.as_ref()
            })
        );
        IntoResponse::new(&self.client.request)
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
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 2, has_body: false } ) => {
      pub fn $name<S: AsRef<str>>(&'a self, id: S, id2: S) -> IntoResponse<'a, $T, Client>
      {
        self.client.request()
            .set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({
                "id": id.as_ref(),
                "id2": id2.as_ref()
            })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 2, has_body: false } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>>(&'a self, id: S, id2: S) -> IntoResponse<'a, $T, Client>
      {
        self.client.request()
            .set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({
                "id": id.as_ref(),
                "id2": id2.as_ref()
            })
        );
        IntoResponse::new(&self.client.request)
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
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 3, has_body: false } ) => {
      pub fn $name<S: AsRef<str>>(&'a self, id: S, id2: S, id3: S) -> IntoResponse<'a, $T, Client>
      {
        self.client.request()
            .set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({
                "id": id.as_ref(),
                "id2": id2.as_ref(),
                "id3": id3.as_ref()
            })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 3, has_body: false } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>>(&'a self, id: S, id2: S, id3: S) -> IntoResponse<'a, $T, Client>
      {
        self.client.request()
            .set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({
                "id": id.as_ref(),
                "id2": id2.as_ref(),
                "id3": id3.as_ref()
            })
        );
        IntoResponse::new(&self.client.request)
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
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 4, has_body: false } ) => {
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
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 4, has_body: false } ) => {
      #[doc = $doc]
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
        IntoResponse::new(&self.client.request)
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
        IntoResponse::new(&self.client.request)
      }
    };

     ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 0, has_body: true } ) => {
      pub fn $name<B: serde::Serialize>(&'a self, body: &B) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_body(serde_json::to_string_pretty(body).unwrap());

        render_path!(
            self.client,
            $template
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 0, has_body: true } ) => {
      #[doc = $doc]
      pub fn $name<B: serde::Serialize>(&'a self, body: &B) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_body(serde_json::to_string_pretty(body).unwrap());

        render_path!(
            self.client,
            $template
        );
        IntoResponse::new(&self.client.request)
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
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 1, has_body: true } ) => {
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
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 1, has_body: true } ) => {
      #[doc = $doc]
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
        IntoResponse::new(&self.client.request)
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
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 2, has_body: true } ) => {
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
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 2, has_body: true } ) => {
      #[doc = $doc]
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
        IntoResponse::new(&self.client.request)
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
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 3, has_body: true } ) => {
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
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 3, has_body: true } ) => {
      #[doc = $doc]
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
        IntoResponse::new(&self.client.request)
      }
    };
}

#[macro_use]
macro_rules! register_download {
    ( | $name:ident, $T:ty => $template:expr ) => {
      pub fn $name<S: AsRef<str>, P: AsRef<Path>>(&'a self, id: S, directory: P) -> $T {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::Download(directory.as_ref().to_path_buf()),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect),
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
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::Download(directory.as_ref().to_path_buf()),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect),
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

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::GET, params: 0, has_body: false }
        );
   };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::GET, params: 0, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::GET, params: 1, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::GET, params: 1, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::GET, params: 2, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::GET, params: 2, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::GET, params: 3, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::GET, params: 3, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::GET, params: 4, has_body: false }
        );
    };


    // Registrations with doc comments.
    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: 0, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: 0, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: 1, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: 1, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: 2, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: 2, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: 3, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: 3, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: 4, has_body: false }
        );
    };
    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: 4, has_body: true }
        );
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


    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: 0, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: 0, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: 1, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: 1, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: 2, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: 2, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: 3, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: 3, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: 4, has_body: true }
        );
    };


    // Registrations with doc comments.
    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 0, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 0, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 1, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 1, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 2, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 2, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 3, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 3, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 4, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 4, has_body: true }
        );
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


    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PATCH, params: 0, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PATCH, params: 0, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PATCH, params: 1, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PATCH, params: 1, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PATCH, params: 2, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PATCH, params: 2, has_body: true }
        );
    };
    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PATCH, params: 3, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PATCH, params: 3, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PATCH, params: 4, has_body: false }
        );
    };


    // Registrations with doc comments.
    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 0, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 0, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 1, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 1, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 2, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 2, has_body: true }
        );
    };
    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 3, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 3, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 4, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 4, has_body: true }
        );
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

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PUT, params: 0, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PUT, params: 0, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PUT, params: 1, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PUT, params: 1, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PUT, params: 2, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PUT, params: 2, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PUT, params: 3, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PUT, params: 3, has_body: true }
        );
    };

   ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::PUT, params: 4, has_body: false }
        );
    };


    // Registrations with doc comments.
    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 0, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 0, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 1, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 1, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 2, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 2, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 3, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 3, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 4, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 4, has_body: true }
        );
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


    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::DELETE, params: 0, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::DELETE, params: 0, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::DELETE, params: 1, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::DELETE, params: 1, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::DELETE, params: 2, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::DELETE, params: 2, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::DELETE, params: 3, has_body: false }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: true }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::DELETE, params: 3, has_body: true }
        );
    };

    ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: false }) => {
        register_method!(
            { name: $name, response: $T, path: $template, method: Method::DELETE, params: 4, has_body: false }
        );
    };


    // Registrations with doc comments.
    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: 0, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: 0, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: 1, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: 1, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: 2, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 2, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: 2, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: 3, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 3, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: 3, has_body: true }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: 4, has_body: false }
        );
    };

    ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 4, has_body: true }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: 4, has_body: true }
        );
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
