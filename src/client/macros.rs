#[macro_use]
macro_rules! client_struct {
    ( $name:ident ) => {
        pub struct $name<'a, I> {
            client: &'a Graph,
            ident: PhantomData<I>,
        }

        client_impl!($name);
    };
}

#[macro_use]
macro_rules! client_impl {
    ( $name:ident ) => {
        impl<'a, I> $name<'a, I> {
            pub fn new(client: &'a Graph) -> $name<'a, I> {
                $name {
                    client,
                    ident: PhantomData,
                }
            }
        }
    };
}

macro_rules! set_req {
    ( $self:expr, $ord:expr, $m:expr, $fmt:expr ) => {
        $self.client.request().extend_ord_from($ord).set_method($m);
        if $fmt {
            $self.client.request().format_ord();
        }
    };

    ( $self:expr, $ord:expr, $m:expr, $body:expr, $fmt:expr ) => {
        $self
            .client
            .request()
            .extend_ord_from($ord)
            .set_body(serde_json::to_string_pretty($body).unwrap())
            .set_method($m);
        if $fmt {
            $self.client.request().format_ord();
        }
    };
}

#[macro_use]
macro_rules! request_method_fn {
  ( $name:ident, $T:ty, $ord:expr, $fnitem:expr, $m:expr, $fmt:expr ) => {
      pub fn $name(&'a self, arg: &str) -> ResponseClient<'a, I, $T> {
        self.client.request().extend_ord_from($fnitem(arg.to_string()));
        set_req!(self, $ord, $m, $fmt);
        ResponseClient::new(self.client)
      }
    };

    ( $name:ident, $T:ty, $ord:expr, $fnitem:expr, $m:expr, $fmt:expr, $b:expr ) => {
      pub fn $name<B: serde::Serialize>(&'a self, arg: &str, body: &B) -> ResponseClient<'a, I, $T> {
       self.client.request().extend_ord_from($fnitem(arg.to_string()));
        set_req!(self, $ord, $m, &body, $fmt);
        ResponseClient::new(self.client)
      }
    };
}

#[macro_use]
macro_rules! request_method {
    ( $name:ident, $T:ty, $ord:expr, $m:expr, $fmt:expr ) => {
      pub fn $name(&'a self) -> ResponseClient<'a, I, $T> {
        set_req!(self, $ord, $m, $fmt);
        ResponseClient::new(self.client)
      }
    };

    ( $name:ident, $T:ty, $ord:expr, $m:expr, $fmt:expr, $b:expr ) => {
      pub fn $name<B: serde::Serialize>(&'a self, body: &B) -> ResponseClient<'a, I, $T> {
        set_req!(self, $ord, $m, &body, $fmt);
        ResponseClient::new(self.client)
      }
    };

    ( $name:ident, $arg_type:ty, $n:expr, $T:ty, $ord:expr, $m:expr, $fmt:expr ) => {
      pub fn $name(&'a self, arg: $arg_type) -> ResponseClient<'a, I, $T> {
        set_req!(self, $ord, $m, &json!({ $n: arg }), $fmt);
        ResponseClient::new(self.client)
      }
    };
}

#[macro_use]
macro_rules! request_method_ident {
    ( $name:ident, $I: ty, $T:ty, $ord:expr, $m:expr, $fmt:expr ) => {
      pub fn $name(&'a self) -> ResponseClient<'a, $I, $T> {
        set_req!(self, $ord, $m, $fmt);
        ResponseClient::new(self.client)
      }
    };

    ( $name:ident, $I: ty, $T:ty, $ord:expr, $m:expr, $fmt:expr, $b:expr ) => {
      pub fn $name<B: serde::Serialize>(&'a self, body: &B) -> ResponseClient<'a, $I, $T> {
        set_req!(self, $ord, $m, &body, $fmt);
        ResponseClient::new(self.client)
      }
    };

    ( $name:ident, $arg_type:ty, $n:expr, $I: ty, $T:ty, $ord:expr, $m:expr, $fmt:expr ) => {
      pub fn $name(&'a self, arg: $arg_type) -> ResponseClient<'a, $I, $T> {
        set_req!(self, $ord, $m, &json!({ $n: arg }), $fmt);
        ResponseClient::new(self.client)
      }
    };
}

#[macro_use]
macro_rules! get {
    ( $name:ident, $T:ty, $ord:expr ) => {
        request_method!($name, $T, $ord, Method::GET, true);
    };

    ( $name:ident, $T:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $T, $ord, Method::GET, $fmt);
    };

    ( $name:ident, $T:ty, $ord:expr, $fmt:expr, $b:expr ) => {
        request_method!($name, $T, $ord, Method::GET, $fmt, $b);
    };

    ( $name:ident, $arg_type:ty, $n:expr, $T:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $arg_type, $n, $T, $ord, Method::GET, $fmt);
    };
}

#[macro_use]
macro_rules! post {
    ( $name:ident, $T:ty, $ord:expr ) => {
        request_method!($name, $T, $ord, Method::POST, true);
    };

    ( $name:ident, $T:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $T, $ord, Method::POST, $fmt);
    };

    ( $name:ident, $T:ty, $ord:expr, $fmt:expr, $b:expr ) => {
        request_method!($name, $T, $ord, Method::POST, $fmt, $b);
    };

    ( $name:ident, $arg_type:ty, $n:expr, $T:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $arg_type, $n, $T, $ord, Method::POST, $fmt);
    };
}

#[macro_use]
macro_rules! patch {
    ( $name:ident, $T:ty, $ord:expr ) => {
        request_method!($name, $T, $ord, Method::PATCH, true);
    };

    ( $name:ident, $T:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $T, $ord, Method::PATCH, $fmt);
    };

    ( $name:ident, $T:ty, $ord:expr, $fmt:expr, $b:expr ) => {
        request_method!($name, $T, $ord, Method::PATCH, $fmt, $b);
    };

    ( $name:ident, $arg_type:ty, $n:expr, $T:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $arg_type, $n, $T, $ord, Method::PATCH, $fmt);
    };
}

#[allow(unused_macros)]
#[macro_use]
macro_rules! put {
    ( $name:ident, $T:ty, $ord:expr ) => {
        request_method!($name, $T, $ord, Method::PUT, true);
    };

    ( $name:ident, $T:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $T, $ord, Method::PUT, $fmt);
    };

    ( $name:ident, $T:ty, $ord:expr, $fmt:expr, $b:expr ) => {
        request_method!($name, $T, $ord, Method::PUT, $fmt, $b);
    };

    ( $name:ident, $arg_type:ty, $n:expr, $T:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $arg_type, $n, $T, $ord, Method::PUT, $fmt);
    };
}

#[macro_use]
macro_rules! delete {
    ( $name:ident, $T:ty, $ord:expr ) => {
        request_method!($name, $T, $ord, Method::DELETE, true);
    };

    ( $name:ident, $T:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $T, $ord, Method::DELETE, $fmt);
    };

    ( $name:ident, $T:ty, $ord:expr, $fmt:expr, $b:expr ) => {
        request_method!($name, $T, $ord, Method::DELETE, $fmt, $b);
    };

    ( $name:ident, $arg_type:ty, $n:expr, $T:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $arg_type, $n, $T, $ord, Method::DELETE, $fmt);
    };
}
