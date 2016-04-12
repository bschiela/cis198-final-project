//! A general, Signature Version 4 Signable AWS Request

use hyper::client::RequestBuilder;

pub struct AWSRequest<'a, T> {
    request: RequestBuilder<'a>,
    body_params: T
}
