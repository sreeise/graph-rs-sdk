#[macro_use]
macro_rules! register_download {
    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: 0 } ) => {
      pub fn $name<P: AsRef<Path>>(&'a self, directory: P) -> $T {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::Download(directory.as_ref().to_path_buf()),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect),
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        self.client.request().download()
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: 1 } ) => {
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

    ( { name: $name:ident, response: $T:ty, path: $template:expr } ) => {
      pub fn $name<P: AsRef<Path>>(&'a self, directory: P) -> $T {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::Download(directory.as_ref().to_path_buf()),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect),
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        self.client.request().download()
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ] } ) => {
      pub fn $name<S: AsRef<str>, P: AsRef<Path>>(&'a self, $p: S, directory: P) -> $T {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::Download(directory.as_ref().to_path_buf()),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect),
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
        );
        self.client.request().download()
      }
    };
}

#[macro_use]
macro_rules! register_async_download {
    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: 0 } ) => {
      pub async fn $name<P: AsRef<Path>>(&'a self, directory: P) -> $T {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::Download(directory.as_ref().to_path_buf()),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect),
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        self.client.request().download().await
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: 1 } ) => {
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

    ( { name: $name:ident, response: $T:ty, path: $template:expr } ) => {
      pub async fn $name<P: AsRef<Path>>(&'a self, directory: P) -> $T {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::Download(directory.as_ref().to_path_buf()),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect),
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        self.client.request().download().await
      }
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ] } ) => {
      pub async fn $name<S: AsRef<str>, P: AsRef<Path>>(&'a self, $p: S, directory: P) -> $T {
        self.client.request()
            .set_request(vec![
                graph_http::RequestAttribute::Method(Method::GET),
                graph_http::RequestAttribute::Download(directory.as_ref().to_path_buf()),
                graph_http::RequestAttribute::RequestType(graph_http::RequestType::Redirect),
            ]).unwrap();

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
        );
        self.client.request().download().await
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

    ( { name: $name:ident, response: $response:ty, path: $template:expr, params: [ $p:ident ] } ) => {
        register_download!( { name: $name, response: $response, path: $template, params: [ $p ]  }  );
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

    ( { name: $name:ident, response: $response:ty, path: $template:expr, params: [ $p:ident ] } ) => {
        register_async_download!( { name: $name, response: $response, path: $template, params: [ $p ]  } );
    };
}
