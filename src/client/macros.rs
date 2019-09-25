#[macro_use]
macro_rules! request_method_fn {
  ( $name:ident, $I:ty, $ord:expr, $fnitem:expr, $m:expr, $fmt:expr ) => {
      pub fn $name(&'a self, arg: &str) -> ResponseClient<'a, I, $I> {
        self.client.request()
            .extend_ord_from($ord)
            .extend_ord_from($fnitem(arg.to_string()))
            .set_method($m)
            .sort_ord();
         if $fmt {
            self.client
                .request()
                .format_ord();
         }
        ResponseClient::new(self.client)
      }
    };

    ( $name:ident, $I:ty, $ord:expr, $fnitem:expr, $m:expr, $fmt:expr, $b:expr ) => {
      pub fn $name<B: serde::Serialize>(&'a self, arg: &str, body: &B) -> ResponseClient<'a, I, $I> {
        self.client.request()
            .extend_ord_from($ord)
            .extend_ord_from($fnitem(arg.to_string()))
            .set_method($m)
            .set_body(serde_json::to_string_pretty(&body).unwrap())
            .sort_ord();
            if $fmt {
               self.client
                .request()
                .format_ord();
            }
        ResponseClient::new(self.client)
      }
    };
}

#[macro_use]
macro_rules! request_method {
    ( $name:ident, $I:ty, $ord:expr, $m:expr, $fmt:expr ) => {
      pub fn $name(&'a self) -> ResponseClient<'a, I, $I> {
        self.client.request()
            .extend_ord_from($ord)
            .set_method($m)
            .sort_ord();
         if $fmt {
            self.client
                .request()
                .format_ord();
         }
        ResponseClient::new(self.client)
      }
    };

    ( $name:ident, $I:ty, $ord:expr, $m:expr, $fmt:expr, $b:expr ) => {
      pub fn $name<B: serde::Serialize>(&'a self, body: &B) -> ResponseClient<'a, I, $I> {
        self.client.request()
            .extend_ord_from($ord)
            .set_method($m)
            .set_body(serde_json::to_string_pretty(&body).unwrap())
            .sort_ord();
            if $fmt {
               self.client
                .request()
                .format_ord();
            }
        ResponseClient::new(self.client)
      }
    };

    ( $name:ident, $arg_type:ty, $n:expr, $I:ty, $ord:expr, $m:expr, $fmt:expr ) => {
      pub fn $name(&'a self, arg: $arg_type) -> ResponseClient<'a, I, $I> {
        self.client.request()
            .extend_ord_from($ord)
            .set_method($m)
            .set_body(serde_json::to_string_pretty(&json!({ $n: arg })).unwrap())
            .sort_ord();
         if $fmt {
            self.client
                .request()
                .format_ord();
         }
        ResponseClient::new(self.client)
      }
    };
}

#[macro_use]
macro_rules! request_method_ident {
    ( $name:ident, $T: ty, $I:ty, $ord:expr, $m:expr, $fmt:expr ) => {
      pub fn $name(&'a self) -> ResponseClient<'a, $T, $I> {
        self.client.request()
            .extend_ord_from($ord)
            .set_method($m)
            .sort_ord();
         if $fmt {
            self.client
                .request()
                .format_ord();
         }
        ResponseClient::new(self.client)
      }
    };

    ( $name:ident, $T: ty, $I:ty, $ord:expr, $m:expr, $fmt:expr, $b:expr ) => {
      pub fn $name<B: serde::Serialize>(&'a self, body: &B) -> ResponseClient<'a, $T, $I> {
        self.client.request()
            .extend_ord_from($ord)
            .set_method($m)
            .set_body(serde_json::to_string_pretty(&body).unwrap())
            .sort_ord();
            if $fmt {
               self.client
                .request()
                .format_ord();
            }
        ResponseClient::new(self.client)
      }
    };

    ( $name:ident, $arg_type:ty, $n:expr, $T: ty, $I:ty, $ord:expr, $m:expr, $fmt:expr ) => {
      pub fn $name(&'a self, arg: $arg_type) -> ResponseClient<'a, $T, $I> {
        self.client.request()
            .extend_ord_from($ord)
            .set_method($m)
            .set_body(serde_json::to_string_pretty(&json!({ $n: arg })).unwrap())
            .sort_ord();
         if $fmt {
            self.client
                .request()
                .format_ord();
         }
        ResponseClient::new(self.client)
      }
    };
}

#[macro_use]
macro_rules! get {
    ( $name:ident, $I:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $I, $ord, Method::GET, $fmt);
    };

    ( $name:ident, $I:ty, $ord:expr, $fmt:expr, $b:expr ) => {
        request_method!($name, $I, $ord, Method::GET, $fmt, $b);
    };

    ( $name:ident, $arg_type:ty, $n:expr, $I:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $arg_type, $n, $I, $ord, Method::GET, $fmt);
    };
}

#[macro_use]
macro_rules! post {
    ( $name:ident, $I:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $I, $ord, Method::POST, $fmt);
    };

    ( $name:ident, $I:ty, $ord:expr, $fmt:expr, $b:expr ) => {
        request_method!($name, $I, $ord, Method::POST, $fmt, $b);
    };

    ( $name:ident, $arg_type:ty, $n:expr, $I:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $arg_type, $n, $I, $ord, Method::POST, $fmt);
    };
}

#[macro_use]
macro_rules! patch {
    ( $name:ident, $I:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $I, $ord, Method::PATCH, $fmt);
    };

    ( $name:ident, $I:ty, $ord:expr, $fmt:expr, $b:expr ) => {
        request_method!($name, $I, $ord, Method::PATCH, $fmt, $b);
    };

    ( $name:ident, $arg_type:ty, $n:expr, $I:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $arg_type, $n, $I, $ord, Method::PATCH, $fmt);
    };
}

#[allow(unused_macros)]
#[macro_use]
macro_rules! put {
    ( $name:ident, $I:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $I, $ord, Method::PUT, $fmt);
    };

    ( $name:ident, $I:ty, $ord:expr, $fmt:expr, $b:expr ) => {
        request_method!($name, $I, $ord, Method::PUT, $fmt, $b);
    };

    ( $name:ident, $arg_type:ty, $n:expr, $I:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $arg_type, $n, $I, $ord, Method::PUT, $fmt);
    };
}

#[macro_use]
macro_rules! delete {
    ( $name:ident, $I:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $I, $ord, Method::DELETE, $fmt);
    };

    ( $name:ident, $I:ty, $ord:expr, $fmt:expr, $b:expr ) => {
        request_method!($name, $I, $ord, Method::DELETE, $fmt, $b);
    };

    ( $name:ident, $arg_type:ty, $n:expr, $I:ty, $ord:expr, $fmt:expr ) => {
        request_method!($name, $arg_type, $n, $I, $ord, Method::DELETE, $fmt);
    };
}
