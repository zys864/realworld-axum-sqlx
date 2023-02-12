use http::{HeaderValue, Request};

use tower_http::request_id::{MakeRequestId, RequestId};

// A `MakeRequestId` that generate uuid
#[derive(Clone, Default)]
pub struct TraceId;

impl MakeRequestId for TraceId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let request_id = uuid::Uuid::new_v4().to_string();

        Some(RequestId::new(
            HeaderValue::from_str(&request_id)
                .expect("convert uuid string to headervalue failed"),
        ))
    }
}
