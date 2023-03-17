macro_rules! into_handler {
	($inner:expr, $method:expr, $template:expr $(, params: $($arg_name:ident),*)? $(,)?) => {
        let params = vec![$($( $arg_name.as_ref(), )*)?];
        let json = map_parameters(&params);
        let url_result = $inner.build_url($template, &json);

        match RequestComponents::try_from(($inner.resource_config.resource_identity, $method, url_result)) {
            Ok(rc) => return ResponseHandler::new($inner.client.clone(), rc, None),
            Err(err) => return ResponseHandler::new(
                $inner.client.clone(),
                RequestComponents::new($inner.resource_config.resource_identity, $inner.resource_config.url.clone(), $method, None),
                Some(err)
            )
        }
	};

	($inner:expr, $method:expr, $template:expr, $body:expr $(, params: $($arg_name:ident),*)? $(,)?) => {
        let params = vec![$($( $arg_name.as_ref(), )*)?];
        let json = map_parameters(&params);
        let url_result = $inner.build_url($template, &json);
        let body_result = serde_json::to_string(&$body).map_err(GraphFailure::from);

        let rc_result = RequestComponents::try_from(($inner.resource_config.resource_identity, $method, url_result, body_result));
        if let Ok(rc) = rc_result {
            return ResponseHandler::new($inner.client.clone(), rc, None);
        }
        let rc = RequestComponents::new($inner.resource_config.resource_identity, $inner.resource_config.url.clone(), $method, None);
        return ResponseHandler::new($inner.client.clone(), rc, rc_result.err());
	};
}

macro_rules! resource_api_method {
	($(doc: $doc:expr,)? name: $name:ident, path: $template:expr, method: $method:expr $(, params: $($arg_name:ident),*)?) => {
        $( #[doc = $doc] )?
        pub fn $name(&self $(, $( $arg_name : impl AsRef<str> ),* )? ) -> ResponseHandler {
            into_handler!(&self, $method, $template $(, params: $( $arg_name ),* )?);
        }
	};

	($(doc: $doc:expr,)? name: $name:ident, path: $template:expr, body: $body:expr, method: $method:expr $(, params: $($arg_name:ident),*)?) => {
        $( #[doc = $doc] )?
        pub fn $name<B: serde::Serialize>(&self $(, $( $arg_name : impl AsRef<str> ),* )?, body: &B) -> ResponseHandler {
            into_handler!(&self, $method, $template, body $(, params: $( $arg_name ),* )?);
        }
	};

	($(doc: $doc:expr,)? name: $name:ident, path: $template:expr, method: $method:expr, body: $body:expr $(, params: $($arg_name:ident,)*)?) => {
        $( #[doc = $doc] )?
        pub fn $name<B: serde::Serialize>(&self $(, $( $arg_name : impl AsRef<str> ),* )?, body: &B) -> ResponseHandler {
            into_handler!(&self, $method, $template, body $(, params: $( $arg_name ),* )?);
        }
	};
}

macro_rules! get {
	($(doc: $doc:expr,)? name: $name:ident, path: $path:expr $(, params: $($arg_name:ident),*)?) => {
        resource_api_method!(
            $( doc: $doc, )?
            name: $name,
            path: $path,
            method: Method::GET
            $(, params: $( $arg_name ),* )?
        );
	};

	($(doc: $doc:expr,)? name: $name:ident, path: $path:expr, body: $body:expr $(, params: $($arg_name:ident),*)?) => {
        resource_api_method!(
            $( doc: $doc, )?
            name: $name,
            path: $path,
            body: $body,
            method: Method::GET
            $(, params: $( $arg_name ),* )?
        );
	};
}

macro_rules! post {
	($(doc: $doc:expr,)? name: $name:ident, path: $path:expr $(, params: $($arg_name:ident),*)?) => {
        resource_api_method!(
            $( doc: $doc, )?
            name: $name,
            path: $path,
            method: Method::POST
            $(, params: $( $arg_name ),* )?
        );
	};

	($(doc: $doc:expr,)? name: $name:ident, path: $path:expr, body: $body:expr $(, params: $($arg_name:ident),*)?) => {
        resource_api_method!(
            $( doc: $doc, )?
            name: $name,
            path: $path,
            body: $body,
            method: Method::POST
            $(, params: $( $arg_name ),* )?
        );
	};
}

macro_rules! patch {
	($(doc: $doc:expr,)? name: $name:ident, path: $path:expr $(, params: $($arg_name:ident),*)?) => {
        resource_api_method!(
            $( doc: $doc, )?
            name: $name,
            path: $path,
            method: Method::PATCH
            $(, params: $( $arg_name ),* )?
        );
	};

	($(doc: $doc:expr,)? name: $name:ident, path: $path:expr, body: $body:expr $(, params: $($arg_name:ident),*)?) => {
        resource_api_method!(
            $( doc: $doc, )?
            name: $name,
            path: $path,
            body: $body,
            method: Method::PATCH
            $(, params: $( $arg_name ),* )?
        );
	};
}

macro_rules! put {
	($(doc: $doc:expr,)? name: $name:ident, path: $path:expr $(, params: $($arg_name:ident),*)?) => {
        resource_api_method!(
            $( doc: $doc, )?
            name: $name,
            path: $path,
            method: Method::PUT
            $(, params: $( $arg_name ),* )?
        );
	};

	($(doc: $doc:expr,)? name: $name:ident, path: $path:expr, body: $body:expr $(, params: $($arg_name:ident),*)?) => {
        resource_api_method!(
            $( doc: $doc, )?
            name: $name,
            path: $path,
            body: $body,
            method: Method::PUT
            $(, params: $( $arg_name ),* )?
        );
	};
}

macro_rules! delete {
	($(doc: $doc:expr,)? name: $name:ident, path: $path:expr $(, params: $($arg_name:ident),*)?) => {
        resource_api_method!(
            $( doc: $doc, )?
            name: $name,
            path: $path,
            method: Method::DELETE
            $(, params: $( $arg_name ),* )?
        );
	};

	($(doc: $doc:expr,)? name: $name:ident, path: $path:expr, body: $body:expr $(, params: $($arg_name:ident),*)?) => {
        resource_api_method!(
            $( doc: $doc, )?
            name: $name,
            path: $path,
            body: $body,
            method: Method::DELETE
            $(, params: $( $arg_name ),* )?
        );
	};
}
