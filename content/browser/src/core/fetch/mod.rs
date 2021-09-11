use super::public::{
    request::{RequestInfo, RequestInit},
    response::Response,
};

/// fetch function
/// https://fetch.spec.whatwg.org/#fetch-method
/// Needs a considerate for WindowOrWorkerGlobalScope
pub async fn fetch(input: RequestInfo, init: Option<RequestInit>) -> Response {
    todo!()
}
