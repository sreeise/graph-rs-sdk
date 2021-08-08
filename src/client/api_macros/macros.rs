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
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

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
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

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
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

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
        if let Err(err) = client.set_body_with_serialize(body) {
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

    ( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 1, has_body: true } ) => {
      pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, id: S, body: &B) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
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

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 1, has_body: true } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, id: S, body: &B) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
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

    ( [ || $name:ident, $T:ty => $template:expr, $m:expr ] ) => {
      pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, id: S, id2: S, body: &B) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

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
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

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
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

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
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

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
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

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
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref(), "id2": id2.as_ref(), "id3": id3.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 4, has_body: false } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>>(&'a self, id: S, id2: S, id3: S, id4: S) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref(), "id2": id2.as_ref(), "id3": id3.as_ref(), "id4": id4.as_ref() })
        );
        IntoResponse::new(&self.client.request)
      }
    };


    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: 4, has_body: true } ) => {
      #[doc = $doc]
      pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, id: S, id2: S, id3: S, id4: S, body: &B) -> IntoResponse<'a, $T, Client>
      {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
            self.client,
            $template,
            &serde_json::json!({ "id": id.as_ref(), "id2": id2.as_ref(), "id3": id3.as_ref(), "id4": id4.as_ref() })
        );
        IntoResponse::new(&self.client.request)
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

    // Macros with named parameters

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        api_method!(
                { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 ], has_body: false }
            );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 $p2 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 $p2 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 $p2 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 $p2 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 $p2 $p3 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 $p2 $p3 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 $p2 $p3 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 $p2 $p3 ], has_body: false }
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


    // Named macro parameters

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        api_method!(
                { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 ], has_body: false }
            );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 $p2 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 $p2 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 $p2 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 $p2 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 $p2 $p3 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 $p2 $p3 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 $p2 $p3 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p $p1 $p2 $p3 ], has_body: false }
        );
    };


    // Upload methods where the body is a file
   ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, upload: true }) => {
        register_upload!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: 0 }
        );
   };

   ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, upload: true }) => {
        register_upload!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: 1 }
        );
   };

   ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, upload: true }) => {
        register_upload!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 0 }
        );
   };

   ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, upload: true }) => {
        register_upload!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: 1 }
        );
   };

   // Upload session
  ({ name: $name:ident, path: $template:expr, params: 0, has_body: false, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::POST, params: 0, has_body: false, upload_session: true }
        );
   };

  ({ name: $name:ident, path: $template:expr, params: 0, has_body: true, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::POST, params: 0, has_body: true, upload_session: true }
        );
   };

  ({ name: $name:ident, path: $template:expr, params: 1, has_body: false, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::POST, params: 1, has_body: false, upload_session: true }
        );
  };

 ({ name: $name:ident, path: $template:expr, params: 1, has_body: true, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::POST, params: 1, has_body: true, upload_session: true }
        );
  };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 0, has_body: false, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::POST, params: 0, has_body: false, upload_session: true }
        );
   };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 0, has_body: true, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::POST, params: 0, has_body: true, upload_session: true }
        );
   };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 1, has_body: false, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::POST, params: 1, has_body: false, upload_session: true }
        );
  };


 ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 1, has_body: true, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::POST, params: 1, has_body: true, upload_session: true }
        );
  };


  // Upload and upload session with named parameters
        // Upload methods where the body is a file
   ({ name: $name:ident, response: $T:ty, path: $template:expr, upload: true }) => {
        register_upload!(
            { name: $name, response: $T, path: $template, method: Method::POST }
        );
   };

   ({ name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], upload: true }) => {
        register_upload!(
            { name: $name, response: $T, path: $template, method: Method::POST, params: [ $p ] }
        );
   };

   ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, upload: true }) => {
        register_upload!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST }
        );
   };

   ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], upload: true }) => {
        register_upload!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::POST, params: [ $p ] }
        );
   };

   // Upload session
  ({ name: $name:ident, path: $template:expr, has_body: false, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::POST, has_body: false, upload_session: true }
        );
   };

  ({ name: $name:ident, path: $template:expr, has_body: true, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::POST, has_body: true, upload_session: true }
        );
   };

  ({ name: $name:ident, path: $template:expr, params: [ $p:ident ], has_body: false, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::POST, params: [ $p ], has_body: false, upload_session: true }
        );
  };

 ({ name: $name:ident, path: $template:expr, params: [ $p:ident ], has_body: true, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::POST, params: [ $p ], has_body: true, upload_session: true }
        );
  };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, has_body: false, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::POST, has_body: false, upload_session: true }
        );
   };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, has_body: true, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::POST, has_body: true, upload_session: true }
        );
   };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: [ $p:ident ], has_body: false, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::POST, params: [ $p ], has_body: false, upload_session: true }
        );
  };


 ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: [ $p:ident ], has_body: true, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::POST, params: [ $p ], has_body: true, upload_session: true }
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


    // Named macro parameters

        ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        api_method!(
                { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 ], has_body: false }
            );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 $p2 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 $p2 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 $p2 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 $p2 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 $p2 $p3 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 $p2 $p3 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 $p2 $p3 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: [ $p $p1 $p2 $p3 ], has_body: false }
        );
    };


   // Upload methods where the body is a file
   ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, upload: true }) => {
        register_upload!(
            { name: $name, response: $T, path: $template, method: Method::PATCH, params: 0 }
        );
   };

   ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, upload: true }) => {
        register_upload!(
            { name: $name, response: $T, path: $template, method: Method::PATCH, params: 1 }
        );
   };

   ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, upload: true }) => {
        register_upload!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 0 }
        );
   };

   ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, upload: true }) => {
        register_upload!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PATCH, params: 1 }
        );
   };


  // Upload session
  ({ name: $name:ident, path: $template:expr, params: 0, has_body: false, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::PATCH, params: 0, has_body: false, upload_session: true }
        );
   };

  ({ name: $name:ident, path: $template:expr, params: 0, has_body: true, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::PATCH, params: 0, has_body: true, upload_session: true }
        );
   };

  ({ name: $name:ident, path: $template:expr, params: 1, has_body: false, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::PATCH, params: 1, has_body: false, upload_session: true }
        );
  };

 ({ name: $name:ident, path: $template:expr, params: 1, has_body: true, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::PATCH, params: 1, has_body: true, upload_session: true }
        );
  };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 0, has_body: false, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::PATCH, params: 0, has_body: false, upload_session: true }
        );
   };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 0, has_body: true, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::PATCH, params: 0, has_body: true, upload_session: true }
        );
   };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 1, has_body: false, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::PATCH, params: 1, has_body: false, upload_session: true }
        );
  };

 ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 1, has_body: true, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::PATCH, params: 1, has_body: true, upload_session: true }
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


   // Upload methods where the body is a file
   ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 0, upload: true }) => {
        register_upload!(
            { name: $name, response: $T, path: $template, method: Method::PUT, params: 0 }
        );
   };

   ({ name: $name:ident, response: $T:ty, path: $template:expr, params: 1, upload: true }) => {
        register_upload!(
            { name: $name, response: $T, path: $template, method: Method::PUT, params: 1 }
        );
   };

   ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 0, upload: true }) => {
        register_upload!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 0 }
        );
   };

   ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: 1, upload: true }) => {
        register_upload!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: 1 }
        );
   };


  ({ name: $name:ident, path: $template:expr, params: 0, has_body: false, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::PUT, params: 0, has_body: false, upload_session: true }
        );
   };

  ({ name: $name:ident, path: $template:expr, params: 0, has_body: true, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::PUT, params: 0, has_body: true, upload_session: true }
        );
   };

  ({ name: $name:ident, path: $template:expr, params: 1, has_body: false, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::PUT, params: 1, has_body: false, upload_session: true }
        );
  };

 ({ name: $name:ident, path: $template:expr, params: 1, has_body: true, upload_session: true }) => {
        register_upload!(
            { name: $name, path: $template, method: Method::PUT, params: 1, has_body: true, upload_session: true }
        );
  };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 0, has_body: false, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::PUT, params: 0, has_body: false, upload_session: true }
        );
   };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 0, has_body: true, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::PUT, params: 0, has_body: true, upload_session: true }
        );
   };

  ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 1, has_body: false, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::PUT, params: 1, has_body: false, upload_session: true }
        );
  };

 ({ doc: $doc:expr, name: $name:ident, path: $template:expr, params: 1, has_body: true, upload_session: true }) => {
        register_upload!(
            { doc: $doc, name: $name, path: $template, method: Method::PUT, params: 1, has_body: true, upload_session: true }
        );
  };


    // Named macro parameters

        ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        api_method!(
                { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 ], has_body: false }
            );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 $p2 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 $p2 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 $p2 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 $p2 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 $p2 $p3 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 $p2 $p3 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 $p2 $p3 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::PUT, params: [ $p $p1 $p2 $p3 ], has_body: false }
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


    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        api_method!(
                { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 ], has_body: false }
            );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 $p2 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 $p2 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 $p2 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 $p2 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 $p2 $p3 ], has_body: false }
        );
    };

    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 $p2 $p3 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 $p2 $p3 ], has_body: true }
        );
    };

    ( { name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
        api_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::DELETE, params: [ $p $p1 $p2 $p3 ], has_body: false }
        );
    };
}
