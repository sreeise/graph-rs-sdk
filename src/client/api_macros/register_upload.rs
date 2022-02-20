macro_rules! register_upload {
    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 0  } ) => {
      pub fn $name<P: AsRef<Path>>(&'a self, file: P) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client
            .set_body_with_file(file.as_ref().to_path_buf())
        {
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

    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 1  } ) => {
      pub fn $name<S: AsRef<str>, P: AsRef<Path>>(&'a self, id: S, file: P) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client
            .set_body_with_file(file.as_ref().to_path_buf())
        {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 0  } ) => {
      #[doc = $doc]
      pub fn $name<P: AsRef<Path>>(&'a self, file: P) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client
            .set_body_with_file(file.as_ref().to_path_buf())
        {
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

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 1 } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>, P: AsRef<Path>>(&'a self, id: S, file: P) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client
            .set_body_with_file(file.as_ref().to_path_buf())
        {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, path: $template:expr, method: $m:expr, params: 0, has_body: false, upload_session: true} ) => {
      pub fn $name<P: AsRef<Path> + Send + Sync>(&'a self, file: P) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_upload_session(file.as_ref().to_path_buf());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, path: $template:expr, method: $m:expr, params: 0, has_body: true, upload_session: true } ) => {
      pub fn $name<P: AsRef<Path> + Send + Sync, B: serde::Serialize>(&'a self, file: P, body: &B) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        let body = serde_json::to_string(body);

        if let Ok(body) = body {
            client.set_method($m);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, path: $template:expr, method: $m:expr, has_body: false, upload_session: true} ) => {
      pub fn $name<P: AsRef<Path> + Send + Sync>(&'a self, file: P) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_upload_session(file.as_ref().to_path_buf());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, path: $template:expr, method: $m:expr, has_body: true, upload_session: true } ) => {
      pub fn $name<P: AsRef<Path> + Send + Sync, B: serde::Serialize>(&'a self, file: P, body: &B) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        let body = serde_json::to_string(body);

        if let Ok(body) = body {
            client.set_method($m);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, path: $template:expr, method: $m:expr, params: 1, has_body: false, upload_session: true } ) => {
      pub fn $name<S: AsRef<str>, P: AsRef<Path> + Send + Sync>(&'a self, id: S, file: P) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_upload_session(file.as_ref().to_path_buf());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, path: $template:expr, method: $m:expr, params: 1, has_body: true, upload_session: true } ) => {
      pub fn $name<S: AsRef<str>, P: AsRef<Path> + Send + Sync, B: serde::Serialize>(&'a self, id: S, file: P, body: &B) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        let body = serde_json::to_string(body);

        if let Ok(body) = body {
            client.set_method($m);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, path: $template:expr, method: $m:expr, params: 0, has_body: false, upload_session: true } ) => {
      #[doc = $doc]
      pub fn $name<P: AsRef<Path> + Send + Sync>(&'a self, file: P) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_upload_session(file.as_ref().to_path_buf());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, path: $template:expr, method: $m:expr, params: 0, has_body: true, upload_session: true } ) => {
      #[doc = $doc]
      pub fn $name<P: AsRef<Path> + Send + Sync, B: serde::Serialize>(&'a self, file: P, body: &B) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        let body = serde_json::to_string(body);

        if let Ok(body) = body {
            client.set_method($m);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, path: $template:expr, method: $m:expr, has_body: false, upload_session: true } ) => {
      #[doc = $doc]
      pub fn $name<P: AsRef<Path> + Send + Sync>(&'a self, file: P) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_upload_session(file.as_ref().to_path_buf());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, path: $template:expr, method: $m:expr, has_body: true, upload_session: true } ) => {
      #[doc = $doc]
      pub fn $name<P: AsRef<Path> + Send + Sync, B: serde::Serialize>(&'a self, file: P, body: &B) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        let body = serde_json::to_string(body);

        if let Ok(body) = body {
            client.set_method($m);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, path: $template:expr, method: $m:expr, params: 1, has_body: false, upload_session: true } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>, P: AsRef<Path> + Send + Sync>(&'a self, id: S, file: P) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_upload_session(file.as_ref().to_path_buf());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, path: $template:expr, method: $m:expr, params: 1, has_body: true, upload_session: true } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>, P: AsRef<Path> + Send + Sync, B: serde::Serialize>(&'a self, id: S, file: P, body: &B) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        let body = serde_json::to_string(body);

        if let Ok(body) = body {
            client.set_method($m);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    // Upload methods with named parameters

    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr  } ) => {
      pub fn $name<P: AsRef<Path>>(&'a self, file: P) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client
            .set_body_with_file(file.as_ref().to_path_buf())
        {
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

    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident ]  } ) => {
      pub fn $name<S: AsRef<str>, P: AsRef<Path>>(&'a self, $p: S, file: P) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client
            .set_body_with_file(file.as_ref().to_path_buf())
        {
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

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr  } ) => {
      #[doc = $doc]
      pub fn $name<P: AsRef<Path>>(&'a self, file: P) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client
            .set_body_with_file(file.as_ref().to_path_buf())
        {
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

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident ] } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>, P: AsRef<Path>>(&'a self, $p: S, file: P) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client
            .set_body_with_file(file.as_ref().to_path_buf())
        {
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

    ( { name: $name:ident, path: $template:expr, method: $m:expr, has_body: false, upload_session: true } ) => {
      pub fn $name<P: AsRef<Path> + Send + Sync>(&'a self, file: P) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_upload_session(file.as_ref().to_path_buf());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, path: $template:expr, method: $m:expr, has_body: true, upload_session: true } ) => {
      pub fn $name<P: AsRef<Path> + Send + Sync, B: serde::Serialize>(&'a self, file: P, body: &B) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        let body = serde_json::to_string(body);

        if let Ok(body) = body {
            client.set_method($m);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, path: $template:expr, method: $m:expr, params: [ $p:ident ], has_body: false, upload_session: true } ) => {
      pub fn $name<S: AsRef<str>, P: AsRef<Path> + Send + Sync>(&'a self, $p: S, file: P) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_upload_session(file.as_ref().to_path_buf());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { name: $name:ident, path: $template:expr, method: $m:expr, params: [ $p:ident ], has_body: true, upload_session: true } ) => {
      pub fn $name<S: AsRef<str>, P: AsRef<Path> + Send + Sync, B: serde::Serialize>(&'a self, $p: S, file: P, body: &B) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        let body = serde_json::to_string(body);

        if let Ok(body) = body {
            client.set_method($m);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, path: $template:expr, method: $m:expr, has_body: false, upload_session: true } ) => {
      #[doc = $doc]
      pub fn $name<P: AsRef<Path> + Send + Sync>(&'a self, file: P) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_upload_session(file.as_ref().to_path_buf());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, path: $template:expr, method: $m:expr, has_body: true, upload_session: true } ) => {
      #[doc = $doc]
      pub fn $name<P: AsRef<Path> + Send + Sync, B: serde::Serialize>(&'a self, file: P, body: &B) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        let body = serde_json::to_string(body);

        if let Ok(body) = body {
            client.set_method($m);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({})
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, path: $template:expr, method: $m:expr, params: [ $p:ident ], has_body: false, upload_session: true } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>, P: AsRef<Path> + Send + Sync>(&'a self, $p: S, file: P) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        client.set_upload_session(file.as_ref().to_path_buf());

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": $p.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, path: $template:expr, method: $m:expr, params: [ $p:ident ], has_body: true, upload_session: true } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>, P: AsRef<Path> + Send + Sync, B: serde::Serialize>(&'a self, $p: S, file: P, body: &B) -> IntoResponse<'a, UploadSessionClient<Client>, Client>
      {
        let client = self.client.request();
        let body = serde_json::to_string(body);

        if let Ok(body) = body {
            client.set_method($m);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
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
