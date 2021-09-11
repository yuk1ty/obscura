/// https://fetch.spec.whatwg.org/#response
#[derive(Clone)]
struct Response {
    // alternative to `type` on WHATWG spec because `type` is reserved word in Rust
    response_type: ResponseType,
    url: String,
    redirected: bool,
    status: u16,
    ok: bool,
    status_text: String,
    headers: String, // TODO
}

/// https://fetch.spec.whatwg.org/#responsetype
#[allow(dead_code)]
#[derive(Clone)]
enum ResponseType {
    Basic,
    Cors,
    Default,
    Error,
    Opaque,
    OpaqueRedirect,
}

pub mod builder {
    use super::Response;

    struct ResponseBuilder {
        status: u16,
        status_text: String,
        headers: Option<String>, // TODO
    }

    impl ResponseBuilder {
        pub fn start() -> ResponseBuilder {
            Default::default()
        }

        pub fn status<'a>(&'a mut self, status: u16) -> &'a ResponseBuilder {
            self.status = status;
            self
        }

        pub fn status_text<'a, S: Into<String>>(
            &'a mut self,
            status_text: S,
        ) -> &'a ResponseBuilder {
            self.status_text = status_text.into();
            self
        }

        // TODO: need to replace `Headers`
        pub fn headers<'a, S: Into<String>>(&'a mut self, headers: S) -> &'a ResponseBuilder {
            self.headers = Some(headers.into());
            self
        }

        pub fn build(self) -> Response {
            Response {
                response_type: super::ResponseType::Basic, // TODO
                url: "".to_string(),                       // TODO
                redirected: true,                          // TODO
                status: self.status,
                ok: true, // TODO
                status_text: self.status_text,
                headers: self.headers.unwrap(),
            }
        }
    }

    impl Default for ResponseBuilder {
        fn default() -> Self {
            Self {
                status: 202,
                status_text: String::from(""),
                headers: None,
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::ResponseBuilder;

        #[test]
        fn create_response() {
            let mut builder = ResponseBuilder::start();
            builder.status(404);
            builder.status_text("Not Found");
            builder.headers("testing");
            let response = builder.build();

            assert!(response.status == 404);
            assert!(response.status_text == "Not Found".to_string());
        }
    }
}
