pub enum Req {
    Get,
    Redirect,
    NotFound,
}

impl Req {
    pub fn on_request<'a, T>(req: T) -> Option<(&'a str, &'a str)> {
        Req::send(req)
    }

    pub fn send<'a, T>(req: T) -> Option<(&'a str, &'a str)> {
        let get = "GET / HTTP/1.1\r\n";
        let redirect = "GET /redirect";

        match req {
            get => Req::Get.location(),
            redirect => Req::Redirect.location(),
            _ => Req::NotFound.location(),
        }
    }

    fn location(&self) -> Option<(&str, &str)> {
        let req_ok = "HTTP/1.1 200 OK\r\n\r\n";
        let req_not_found = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        match *self {
            Req::Get => Some((req_ok, "html/return.html")),
            Req::Redirect => Some((req_ok, "html/return.html")),
            Req::NotFound => Some((req_not_found, "html/404.html")),
        }
    }
}
