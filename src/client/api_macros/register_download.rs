#[macro_use]
macro_rules! register_download {
    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: 0 } ) => {
      pub fn $name(&'a self) -> IntoResponse<'a, $T, BlockingHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: 1 } ) => {
      pub fn $name<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, $T, BlockingHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr } ) => {
      pub fn $name(&'a self) -> IntoResponse<'a, $T, BlockingHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
      #[doc = $doc]
      pub fn $name(&'a self) -> IntoResponse<'a, $T, BlockingHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: true } ) => {
      #[doc = $doc]
      pub fn $name<B: serde::Serialize>(&'a self, body: &B) -> IntoResponse<'a, $T, BlockingHttpClient> {
        let client = self.client.request();
        client.set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ] } ) => {
      pub fn $name<S: AsRef<str>>(&'a self, $p: S) -> IntoResponse<'a, $T, BlockingHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ] } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>>(&'a self, $p: S) -> IntoResponse<'a, $T, BlockingHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>>(&'a self, $p: S) -> IntoResponse<'a, $T, BlockingHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, $p: S, body: &B) -> IntoResponse<'a, $T, BlockingHttpClient> {
          let client = self.client.request();
          client.set_request(vec![
              graph_http::RequestAttribute::Method(Method::GET),
              graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
          ]).unwrap();

          if let Err(err) = client.set_body_with_serialize(body) {
              return IntoResponse::new_error(self.client.request(), err);
          }

          render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
          );
          IntoResponse::new(&self.client.request)
      }
    };
}

#[macro_use]
macro_rules! register_async_download {
    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: 0 } ) => {
      pub async fn $name(&'a self) -> IntoResponse<'a, $T, AsyncHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: 1 } ) => {
      pub async fn $name<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, $T, AsyncHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr } ) => {
      pub async fn $name(&'a self) -> IntoResponse<'a, $T, AsyncHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
      #[doc = $doc]
      pub async fn $name(&'a self) -> IntoResponse<'a, $T, AsyncHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ] } ) => {
      pub async fn $name<S: AsRef<str>>(&'a self, $p: S) -> IntoResponse<'a, $T, AsyncHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ] } ) => {
      #[doc = $doc]
      pub async fn $name<S: AsRef<str>>(&'a self, $p: S) -> IntoResponse<'a, $T, AsyncHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
      #[doc = $doc]
      pub async fn $name<S: AsRef<str>>(&'a self, $p: S) -> IntoResponse<'a, $T, AsyncHttpClient> {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
      #[doc = $doc]
      pub async fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, $p: S, body: &B) -> IntoResponse<'a, $T, AsyncHttpClient> {
          let client = self.client.request();
          client.set_request(vec![
              graph_http::RequestAttribute::Method(Method::GET),
              graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect)
          ]).unwrap();

          if let Err(err) = client.set_body_with_serialize(body) {
              return IntoResponse::new_error(self.client.request(), err);
          }

          render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
          );
          IntoResponse::new(&self.client.request)
      }
    };
}

#[macro_use]
macro_rules! download {
    ( { name: $name:ident, response: $response:ty, path: $template:expr, params: 0 } ) => {
        register_download!( { name: $name, response: $response, path: $template, params: 0  }  );
    };

    ( { name: $name:ident, response: $response:ty, path: $template:expr, params: 1 } ) => {
        register_download!( { name: $name, response: $response, path: $template, params: 1  }  );
    };

    ( { name: $name:ident, response: $response:ty, path: $template:expr } ) => {
        register_download!( { name: $name, response: $response, path: $template  }  );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $response:ty, path: $template:expr, has_body: false } ) => {
        register_download!( { doc: $doc, name: $name, response: $response, path: $template, has_body: false  }  );
    };

    ( { name: $name:ident, response: $response:ty, path: $template:expr, params: [ $p:ident ] } ) => {
        register_download!( { name: $name, response: $response, path: $template, params: [ $p ]  }  );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $response:ty, path: $template:expr, params: [ $p:ident ] } ) => {
        register_download!( { doc: $doc, name: $name, response: $response, path: $template, params: [ $p ]  }  );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $response:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        register_download!( { doc: $doc, name: $name, response: $response, path: $template, params: [ $p ] , has_body: false }  );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $response:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        register_download!( { doc: $doc, name: $name, response: $response, path: $template, params: [ $p ], has_body: true  }  );
    };
}

#[macro_use]
macro_rules! async_download {
    ( { name: $name:ident, response: $response:ty, path: $template:expr, params: 0 } ) => {
        register_async_download!( { name: $name, response: $response, path: $template, params: 0  } );
    };

    ( { name: $name:ident, response: $response:ty, path: $template:expr, params: 1 } ) => {
        register_async_download!( { name: $name, response: $response, path: $template, params: 1  } );
    };

    ( { name: $name:ident, response: $response:ty, path: $template:expr } ) => {
        register_async_download!( { name: $name, response: $response, path: $template  } );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $response:ty, path: $template:expr, has_body: false } ) => {
        register_async_download!( { doc: $doc, name: $name, response: $response, path: $template, has_body: false  }  );
    };

    ( { name: $name:ident, response: $response:ty, path: $template:expr, params: [ $p:ident ] } ) => {
        register_async_download!( { name: $name, response: $response, path: $template, params: [ $p ]  } );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $response:ty, path: $template:expr, params: [ $p:ident ] } ) => {
        register_async_download!( { doc: $doc, name: $name, response: $response, path: $template, params: [ $p ]  }  );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $response:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        register_async_download!( { doc: $doc, name: $name, response: $response, path: $template, params: [ $p ] , has_body: false }  );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $response:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        register_async_download!( { doc: $doc, name: $name, response: $response, path: $template, params: [ $p ], has_body: true  }  );
    };
}
