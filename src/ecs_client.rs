//! This module contains the ECSClient which can be used to interact with Amazon ECS's API.

use region::Region;
use action::ECSAction;
use request::*;
use custom_headers::{XAmzTarget, XAmzDate};

use hyper;
use hyper::client::RequestBuilder;
use hyper::header::{Headers, Header, HeaderFormat, Host, AcceptEncoding, Encoding, qitem, ContentType, ContentLength};
use hyper::mime::{Mime, TopLevel, SubLevel};

use serde_json;

use time;

/// The service abbreviation string for Amazon ECS.
const SERVICE_ABBREVIATION: &'static str = "ecs";
/// The MIME sublevel content type of an ECS HTTP request body.
const AMZ_SUBLEVEL_CONTENT_TYPE: &'static str = "x-amz-json-1.1";
/// The ECS API version this request is meant for.
const ECS_API_VERSION: &'static str = "AmazonEC2ContainerServiceV20141113";
/// The default algorithm used for calculating the authentication signature.
const SIGNING_ALGORITHM: &'static str = "AWS4-HMAC-SHA256";

pub struct ECSClient {
    region: Region,
    client: hyper::Client
}

// TODO provide code examples of how to use client
impl ECSClient {
    /// Creates a new ECSClient for the specified Region.
    pub fn new(region: Region) -> ECSClient {
        ECSClient {
            region: region,
            client: hyper::Client::new()
        }
    }

    /// Sets the Region to which the client sends requests.
    pub fn set_region(&mut self, region: Region) {
        self.region = region;
    }

    /// Lists all of your compute clusters on ECS.
    pub fn list_clusters(&self, request: list_clusters::ListClustersRequest) {
        let response = self.sign_and_send(ECSAction::ListClusters, request);
        // TODO: deserialize and return response
        unimplemented!()
    }

    /// Signs the request using Amazon's Signature Version 4 Signing Algorithm.
    /// Serializes the service request to json format and sets it as the payload in the HTTP body.
    /// Sends the request to ECS and returns the response.
    fn sign_and_send<T: ecs_request::ECSRequest>(&self,
                                                 action: ECSAction,
                                                 request: T) -> i32 {
        let body_json = serde_json::to_string(&request).unwrap();

        let mut req_builder = self.client.post(&self.compute_hostname());
        req_builder = self.set_headers(req_builder, action);

        // set the json-serialized request as the body of the HTTP request
        req_builder = self.set_body(req_builder, &body_json);

        // TODO get credentials from environment
        // TODO compute AuthV4 Signature -> set as Authorization header
        // TODO send and return response (change return value from i32)
        unimplemented!()
    }

    /// Sets the Host, Accept-Encoding, X-Amz-Target, X-Amz-Date, and Content-Type HTTP headers.
    fn set_headers<'a>(&self, req_builder: RequestBuilder<'a>, action: ECSAction) -> RequestBuilder<'a> {
        let mut headers: Headers = Headers::new();
        headers.set(Host {
            hostname: self.compute_hostname(),
            port: None,
        });
        headers.set(AcceptEncoding(vec![qitem(Encoding::Identity)]));
        headers.set(XAmzTarget(self.compute_x_amz_target(action)));
        headers.set(XAmzDate(time::strftime("%Y%m%dT%H%M%SZ", &time::now_utc()).unwrap()));
        headers.set(ContentType(
                Mime(
                    TopLevel::Application,
                    SubLevel::Ext(String::from(AMZ_SUBLEVEL_CONTENT_TYPE)),
                    vec![],
                )
            )
        );
        req_builder.headers(headers)
    }

    /// Sets the body of the HTTP request.
    fn set_body<'a>(&self, req_builder: RequestBuilder<'a>, body: &'a str) -> RequestBuilder<'a> {
        let content_length = body.len();
        req_builder.body(body).header(ContentLength(content_length as u64))
    }

    /// Builds and returns the hostname String used in the Host header.
    fn compute_hostname(&self) -> String {
        let mut hostname = String::from(SERVICE_ABBREVIATION);
        hostname.push_str(".");
        hostname.push_str(&self.region.to_string());
        hostname.push_str(".amazonaws.com");
        hostname
    }

    /// Builds and returns the target String used in the X-Amz-Target header.
    fn compute_x_amz_target(&self, action: ECSAction) -> String {
        let mut target = String::from(ECS_API_VERSION);
        target.push_str(".");
        target.push_str(&action.to_string());
        target
    }

    /// Builds and returns the canonical request String according to the guidelines at
    /// http://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html .
    /// The canonical request contains the HTTP headers with lowercase names followed by their
    /// value, with consecutive spaces converted to single spaces.  The headers must appear in
    /// order sorted by character code in lowercase, followed by a list of headers included in the
    /// signed request, followed by SHA256-hashed body.  The entire request is then hashed again
    /// and returned as a String.
    fn build_canonical_request(&self, headers: &Headers, body: &str) -> String {
        let mut canon_req = String::from("POST\n");
        let mut signed_headers = String::new();
        canon_req.push_str("/\n"); // canonical URI (empty)
        canon_req.push_str("\n"); // canonical query string (empty)

        // CANONICAL HEADERS
        // must be sorted lexicographically by lowercase header name
        let accept_encoding: &AcceptEncoding = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                AcceptEncoding::header_name(),
                &(accept_encoding as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&AcceptEncoding::header_name().to_lowercase());
        signed_headers.push_str(";");

        let content_length: &ContentLength = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                ContentLength::header_name(),
                &(content_length as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&ContentLength::header_name().to_lowercase());
        signed_headers.push_str(";");

        let content_type: &ContentType = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                ContentType::header_name(),
                &(content_type as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&ContentType::header_name().to_lowercase());
        signed_headers.push_str(";");

        let host: &Host = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                Host::header_name(),
                &(host as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&Host::header_name().to_lowercase());
        signed_headers.push_str(";");

        let x_amz_date: &XAmzDate = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                XAmzDate::header_name(),
                &(x_amz_date as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&XAmzDate::header_name().to_lowercase());
        signed_headers.push_str(";");

        let x_amz_target: &XAmzTarget = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                XAmzTarget::header_name(),
                &(x_amz_target as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&XAmzTarget::header_name().to_lowercase());
        signed_headers.push_str("\n");

        // add list of signed headers
        canon_req.push_str(&signed_headers);
        canon_req
    }

    /// Formats a single header according to the canonical format.  Header names must appear in
    /// lowercase, followed by a ':', followed by the header value, with consecutive spaces
    /// converted to single spaces.  A newline character terminates the String.
    fn fmt_canonical_header(&self, name: &str, value: &str) -> String {
        let mut header = String::from(name).to_lowercase();
        header.push_str(":");
        header.push_str(value);
        // convert sequential spaces to single spaces
        let mut canon_header = String::new();
        for token in header.split_whitespace() {
            canon_header.push_str(token);
            canon_header.push_str(" ");
        }
        // remove trailing space and append newline
        canon_header = String::from(canon_header.trim_right());
        canon_header.push_str("\n");
        canon_header
    }
}

#[cfg(test)]
mod test {
    use super::ECSClient;
    use hyper::header::{Headers, HeaderFormat, Header, Host, AcceptEncoding, Encoding, qitem, ContentType, ContentLength};
    use custom_headers::{XAmzTarget, XAmzDate};
    use time;
    use hyper::mime::{Mime, TopLevel, SubLevel};

    fn build_canonical_request(headers: &Headers, body: &str) -> String {
        let mut canon_req = String::from("POST\n");
        let mut signed_headers = String::new();
        canon_req.push_str("/\n"); // canonical URI (empty)
        canon_req.push_str("\n"); // canonical query string (empty)

        // CANONICAL HEADERS
        // must be sorted lexicographically by lowercase header name
        let accept_encoding: &AcceptEncoding = headers.get().unwrap();
        canon_req.push_str(&self::fmt_canonical_header(
                AcceptEncoding::header_name(),
                &(accept_encoding as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&AcceptEncoding::header_name().to_lowercase());
        signed_headers.push_str(";");

        let content_length: &ContentLength = headers.get().unwrap();
        canon_req.push_str(&self::fmt_canonical_header(
                ContentLength::header_name(),
                &(content_length as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&ContentLength::header_name().to_lowercase());
        signed_headers.push_str(";");

        let content_type: &ContentType = headers.get().unwrap();
        canon_req.push_str(&self::fmt_canonical_header(
                ContentType::header_name(),
                &(content_type as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&ContentType::header_name().to_lowercase());
        signed_headers.push_str(";");

        let host: &Host = headers.get().unwrap();
        canon_req.push_str(&self::fmt_canonical_header(
                Host::header_name(),
                &(host as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&Host::header_name().to_lowercase());
        signed_headers.push_str(";");

        let x_amz_date: &XAmzDate = headers.get().unwrap();
        canon_req.push_str(&self::fmt_canonical_header(
                XAmzDate::header_name(),
                &(x_amz_date as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&XAmzDate::header_name().to_lowercase());
        signed_headers.push_str(";");

        let x_amz_target: &XAmzTarget = headers.get().unwrap();
        canon_req.push_str(&self::fmt_canonical_header(
                XAmzTarget::header_name(),
                &(x_amz_target as &(HeaderFormat + Send + Sync)).to_string()
        ));
        signed_headers.push_str(&XAmzTarget::header_name().to_lowercase());
        signed_headers.push_str("\n");

        // add list of signed headers
        canon_req.push_str(&signed_headers);
        canon_req
    }

    fn fmt_canonical_header(name: &str, value: &str) -> String {
        let mut header = String::from(name).to_lowercase();
        header.push_str(":");
        header.push_str(value);
        // convert sequential spaces to single spaces
        let mut canon_header = String::new();
        for token in header.split_whitespace() {
            canon_header.push_str(token);
            canon_header.push_str(" ");
        }
        // remove trailing space and append newline
        canon_header = String::from(canon_header.trim_right());
        canon_header.push_str("\n");
        canon_header
    }

    fn build_test_headers() -> Headers {
        let mut headers = Headers::new();
        headers.set(Host {
            hostname: String::from("ecs.us-east-1.amazonaws.com"),
            port: None,
        });
        headers.set(AcceptEncoding(vec![qitem(Encoding::Identity)]));
        headers.set(XAmzTarget(String::from("AmazonEC2ContainerServiceV20141113.ListClusters")));
        headers.set(XAmzDate(time::strftime("%Y%m%dT%H%M%SZ", &time::now_utc()).unwrap()));
        headers.set(ContentType(
                Mime(
                    TopLevel::Application,
                    SubLevel::Ext(String::from("x-amz-json-1.1 test  remove   consecutive     spaces")),
                    vec![],
                )
            )
        );
        headers.set(ContentLength(2));
        headers
    }

    #[test]
    fn test_canonical_request_format() {
        let headers = self::build_test_headers();
        let canonical_request = self::build_canonical_request(&headers, "{}");
        println!("\n");
        println!("{}", headers);
        println!("\n");
        println!("{}", canonical_request)
    }
}
