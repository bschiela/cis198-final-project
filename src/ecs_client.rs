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

/// The service abbreviation string for Amazon ECS
const SERVICE_ABBREVIATION: &'static str = "ecs";
/// The MIME sublevel content type of an ECS HTTP request body
const AMZ_SUBLEVEL_CONTENT_TYPE: &'static str = "x-amz-json-1.1";
/// The ECS API version this request is meant for
const ECS_API_VERSION: &'static str = "AmazonEC2ContainerServiceV20141113";
/// The default algorithm used for calculating the authentication signature
const SIGNING_ALGORITHM: &'static str = "AWS4-HMAC-SHA256";

pub struct ECSClient {
    region: Region,
    client: hyper::Client
}

// TODO provide code examples of how to use client
impl ECSClient {
    /// creates a new ECSClient for the specified Region
    pub fn new(region: Region) -> ECSClient {
        ECSClient {
            region: region,
            client: hyper::Client::new()
        }
    }

    /// sets the Region to which the client sends requests
    pub fn set_region(&mut self, region: Region) {
        self.region = region;
    }

    pub fn list_clusters(&self, request: list_clusters::ListClustersRequest) {
        let response = self.sign_and_send(ECSAction::ListClusters, request);
        // TODO: deserialize and return response
        unimplemented!()
    }

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

    fn set_body<'a>(&self, req_builder: RequestBuilder<'a>, body: &'a str) -> RequestBuilder<'a> {
        let content_length = body.len();
        req_builder.body(body).header(ContentLength(content_length as u64))
    }

    fn compute_hostname(&self) -> String {
        let mut hostname = String::from(SERVICE_ABBREVIATION);
        hostname.push_str(".");
        hostname.push_str(&self.region.to_string());
        hostname.push_str(".amazonaws.com");
        hostname
    }

    fn compute_x_amz_target(&self, action: ECSAction) -> String {
        let mut target = String::from(ECS_API_VERSION);
        target.push_str(".");
        target.push_str(&action.to_string());
        target
    }

    fn build_canonical_request(&self, headers: &Headers) -> String {
        let mut canon_req = String::from("POST\n");
        canon_req.push_str("/\n"); // canonical URI (empty)
        canon_req.push_str("\n"); // canonical query string (empty)
        // canonical headers
        // must be sorted lexicographically by lowercase header name
        let accept_encoding: &AcceptEncoding = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                AcceptEncoding::header_name(),
                &(accept_encoding as &(HeaderFormat + Send + Sync)).to_string()
        ));
        let content_length: &ContentLength = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                ContentLength::header_name(),
                &(content_length as &(HeaderFormat + Send + Sync)).to_string()
        ));
        let content_type: &ContentType = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                ContentType::header_name(),
                &(content_length as &(HeaderFormat + Send + Sync)).to_string()
        ));
        let host: &Host = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                Host::header_name(),
                &(host as &(HeaderFormat + Send + Sync)).to_string()
        ));
        let x_amz_date: &XAmzDate = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                XAmzDate::header_name(),
                &(x_amz_date as &(HeaderFormat + Send + Sync)).to_string()
        ));
        let x_amz_target: &XAmzTarget = headers.get().unwrap();
        canon_req.push_str(&self.fmt_canonical_header(
                XAmzTarget::header_name(),
                &(x_amz_target as &(HeaderFormat + Send + Sync)).to_string()
        ));
        canon_req
    }

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
    use hyper::header::{Headers, HeaderFormat, Host, AcceptEncoding, Encoding, qitem, ContentType, ContentLength};
    use custom_headers::{XAmzTarget, XAmzDate};
    use time;
    use hyper::mime::{Mime, TopLevel, SubLevel};

    #[test]
    fn test_view_headers() {
        let mut headers: Headers = Headers::new();
        headers.set(Host {
            hostname: String::from("ecstest.us-east-1.amazonaws.com"),
            port: None,
        });
        headers.set(AcceptEncoding(vec![qitem(Encoding::Identity)]));
        headers.set(XAmzTarget(String::from("API_Version.ListClusters")));
        headers.set(XAmzDate(time::strftime("%Y%m%dT%H%M%SZ", &time::now_utc()).unwrap()));
        headers.set(ContentType(
                Mime(
                    TopLevel::Application,
                    SubLevel::Ext(String::from("application/amz-json-1.1")),
                    vec![],
                )
            )
        );

        let host_val: &Host = headers.get().unwrap();
        println!("{}", host_val.hostname);
        let xamz_date: &XAmzDate = headers.get().unwrap();
        let date_val = &xamz_date.0;
        let acc_enc: &AcceptEncoding = headers.get().unwrap();
        println!("{:?}", xamz_date);
        println!("{}", date_val);
        println!("\n");
        println!("{}", headers);
        println!("{}", host_val as &(HeaderFormat + Send + Sync));
        println!("{}", acc_enc as &(HeaderFormat + Send + Sync));
    }
}
