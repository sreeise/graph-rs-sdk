macro_rules! api_method {
    ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, has_body: false } ) => {
    #[doc = $doc]
    pub fn $name(&'a self)-> IntoResponse<'a, $T, Client>
    {
        self.client.request()
            .set_method($m);

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ }));
        IntoResponse::new(&self.client.request)
    }
};

( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, has_body: true } ) => {
    #[doc = $doc]
    pub fn $name<B: serde::Serialize>(&'a self, body: &B)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ }));
        IntoResponse::new(&self.client.request)
    }
};

( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, has_body: true } ) => {
    pub fn $name<B: serde::Serialize>(&'a self, body: &B)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ }));
        IntoResponse::new(&self.client.request)
    }
};

( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, has_body: false } ) => {
    pub fn $name(&'a self)-> IntoResponse<'a, $T, Client>
    {
        self.client.request()
            .set_method($m);

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ }));
        IntoResponse::new(&self.client.request)
    }
};

( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident ], has_body: false } ) => {
    #[doc = $doc]
    pub fn $name<S: AsRef<str>>(&'a self, $p: S)-> IntoResponse<'a, $T, Client>
    {
        self.client.request()
            .set_method($m);

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident ], has_body: true } ) => {
    #[doc = $doc]
    pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, $p: S, body: &B)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident ], has_body: true } ) => {
    pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, $p: S, body: &B)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident ], has_body: false } ) => {
    pub fn $name<S: AsRef<str>>(&'a self, $p: S)-> IntoResponse<'a, $T, Client>
    {
        self.client.request()
            .set_method($m);

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
    #[doc = $doc]
    pub fn $name<S: AsRef<str>>(&'a self, $p: S, $p1: S)-> IntoResponse<'a, $T, Client>
    {
        self.client.request()
            .set_method($m);

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
    #[doc = $doc]
    pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, $p: S, $p1: S, body: &B)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident ], has_body: true } ) => {
    pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, $p: S, $p1: S, body: &B)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
    pub fn $name<S: AsRef<str>>(&'a self, $p: S, $p1: S)-> IntoResponse<'a, $T, Client>
    {
        self.client.request()
            .set_method($m);

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
    #[doc = $doc]
    pub fn $name<S: AsRef<str>>(&'a self, $p: S, $p1: S, $p2: S)-> IntoResponse<'a, $T, Client>
    {
        self.client.request()
            .set_method($m);


        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref(), "id2": $p2.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
    #[doc = $doc]
    pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, $p: S, $p1: S, $p2: S, body: &B)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref(), "id2": $p2.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: true } ) => {
    pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, $p: S, $p1: S, $p2: S, body: &B)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref(), "id2": $p2.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident $p2:ident ], has_body: false } ) => {
    pub fn $name<S: AsRef<str>>(&'a self, $p: S, $p1: S, $p2: S)-> IntoResponse<'a, $T, Client>
    {
        self.client.request()
            .set_method($m);

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref(), "id2": $p2.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
    #[doc = $doc]
    pub fn $name<S: AsRef<str>>(&'a self, $p: S, $p1: S, $p2: S, $p3: S)-> IntoResponse<'a, $T, Client>
    {
        self.client.request()
            .set_method($m);

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref(), "id2": $p2.as_ref(), "id3": $p3.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
    #[doc = $doc]
    pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, $p: S, $p1: S, $p2: S, $p3: S, body: &B)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref(), "id2": $p2.as_ref(), "id3": $p3.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: true } ) => {
    pub fn $name<S: AsRef<str>, B: serde::Serialize>(&'a self, $p: S, $p1: S, $p2: S, $p3: S, body: &B)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);
        if let Err(err) = client.set_body_with_serialize(body) {
            return IntoResponse::new_error(self.client.request(), err);
        }

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref(), "id2": $p2.as_ref(), "id3": $p3.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};

( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident $p2:ident $p3:ident ], has_body: false } ) => {
    pub fn $name<S: AsRef<str>>(&'a self, $p: S, $p1: S, $p2: S, $p3: S)-> IntoResponse<'a, $T, Client>
    {
        self.client.request()
            .set_method($m);

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref(), "id2": $p2.as_ref(), "id3": $p3.as_ref() }));
        IntoResponse::new(&self.client.request)
    }
};


    // Methods with a query parameter as part of the request method. Example is methods that accept
    // a deltaToken query.
    //
    // This is to conform to other graph api sdk's where you can provide an optional delta token
    // to start from or none at all and usually done through some form of method overloading.
    //
    // In other words its syntax sugar and does the exact same thing as calling the IntoResponse
    // delta_token function which just adds the deltaToken query parameter with a provided value.
    // The only difference here is that any resource that has a delta method will now also have
    // a delta token method that sets the deltaToken query parameter.

( { name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, query: { key: $key:expr, value: $p:ident } } ) => {
    pub fn $name<S: AsRef<str> + Sync>(&'a self, $p: S)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);

        client.url_mut(|url| {
            url.append_query_pair($key, $p.as_ref());
        });

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ }));
        IntoResponse::new(&self.client.request)
    }
};

( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, query: [ key: $key:expr, value: $p:ident ] } ) => {
    #[doc = $doc]
    pub fn $name<S: AsRef<str> + Sync>(&'a self, $p: S)-> IntoResponse<'a, $T, Client>
    {
        let client = self.client.request();
        client.set_method($m);

        client.url_mut(|url| {
            url.append_query_pair($key, $p.as_ref());
        });

        render_path!(
                self.client,
                $template,
                &serde_json::json!({ }));
        IntoResponse::new(&self.client.request)
    }
};
}
