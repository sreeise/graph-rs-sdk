macro_rules! resource_client {
    ( $name:ident ) => {
        pub struct $name {
            client: graph_http::api_impl::Client,
            resource_config: graph_http::api_impl::ResourceConfig,
            registry: handlebars::Handlebars,
        }

        impl $name {
            pub(crate) fn new(
                client: graph_http::api_impl::Client,
                resource_config: graph_http::api_impl::ResourceConfig,
                registry: handlebars::Handlebars,
            ) -> $name {
                $name {
                    client,
                    resource_config,
                    registry,
                }
            }
        }

        impl ApiClientImpl for $name {
            fn url(&self) -> GraphUrl {
                self.resource_config.url.clone()
            }

            fn render_template<S: AsRef<str>>(
                &self,
                path: S,
                path_params_map: &serde_json::Value,
            ) -> GraphResult<String> {
                self.registry
                    .render_template(path.as_ref(), path_params_map)
                    .map_err(GraphFailure::from)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("resource_config", &self.resource_config)
                    .finish()
            }
        }
    };
}

macro_rules! register_client {
    ( $name:ident, $($helper:ident => $value:expr,)* ) => {
        $( register_helper!($helper, $value); )*

        pub struct $name<'a, Client> {
            pub(crate) client: &'a Graph<Client>,
        }

        impl<'a, Client> $name<'a, Client> where Client: graph_http::RequestClient {
            pub(crate) fn new(client: &'a Graph<Client>) -> $name<'a, Client> {

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

    ( $name:ident, $($helper:ident => $value:expr,)* ()) => {
        $( register_helper!($helper, $value); )*

        #[allow(dead_code)]
        pub struct $name<'a, Client> {
            pub(crate) client: &'a Graph<Client>,
            pub(crate) id: String,
        }

        impl<'a, Client> $name<'a, Client,> where Client: graph_http::RequestClient  {
            pub(crate) fn new(id: &str, client: &'a Graph<Client>) -> $name<'a, Client> {
                let id_stored = id.to_string();
                $(
                    client.request().registry(|r| {
                        r.register_helper(stringify!($helper), Box::new($helper));
                    });
                )*

                if !id_stored.is_empty() {
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
                }

                $name {
                    client,
                    id: id_stored
                }
            }
        }
    };

    ( $name:ident, $($helper:ident => $value:expr, $value2:expr, $identity:expr,)* ) => {
        pub struct $name<'a, Client> {
            pub(crate) client: &'a Graph<Client>,
        }

        impl<'a, Client> $name<'a, Client> where Client: graph_http::RequestClient {
            pub(crate) fn new(client: &'a Graph<Client>) -> $name<'a, Client> {
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

    // Drive only macro.
    ( () $name:ident, $($helper:ident => $value:expr, $value2:expr, $identity:expr,)* ) => {
        #[allow(dead_code)]
        pub struct $name<'a, Client> {
            pub(crate) client: &'a Graph<Client>,
            pub(crate) id: String,
        }

        impl<'a, Client> $name<'a, Client> where Client: graph_http::RequestClient {
            pub(crate) fn new(id: &str, client: &'a Graph<Client>) -> $name<'a, Client> {
                let ident = client.ident();
                let id_stored = id.to_string();
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

               if !id_stored.is_empty() {
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
               }

               client.request.register_ident_helper(ident);

                $name {
                    client,
                    id: id_stored,
                }
            }
        }
    };
}
