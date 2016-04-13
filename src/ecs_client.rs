//! This module contains the ECSClient which can be used to interact with Amazon ECS's API.

use region::Region;
use action::ECSAction;
use params::ListClustersParams;

use hyper;
use hyper::client::RequestBuilder;
use hyper::header::{Headers, Host, AcceptEncoding, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

/// The MIME sublevel content type of an ECS HTTP request body
const AMZ_SUBLEVEL_CONTENT_TYPE: &'static str = "x-amz-json-1.1";

pub struct ECSClient {
    region: Region,
    client: hyper::Client
}

// TODO provide code examples of how to use client
// TODO document all functions
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

    pub fn list_clusters(&self) {
        unimplemented!()
    }

    fn set_headers<'a>(&self, req_builder: RequestBuilder<'a>, action: ECSAction) -> RequestBuilder<'a> {
        let mut headers: Headers = Headers::new();
        headers.set(Host {
            hostname: self.compute_hostname(),
            port: None,
        });
        headers.set(AcceptEncoding(vec![]));
        // TODO set X-Amz-Target header
        // TODO set X-Amz-Date header
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

    // TODO make general over all types of body_params
    fn set_body(&self, req_builder: RequestBuilder, body_params: ListClustersParams) -> RequestBuilder {
        unimplemented!()
        // TODO compute and set content-length header
    }

    fn sign(&self, req_builder: RequestBuilder) -> RequestBuilder {
        unimplemented!()
        // TODO get credentials from environment
        // TODO compute AuthV4 Signature
        // TODO set Authorization header
    }

    fn compute_hostname(&self) -> String {
        let mut hostname = String::from("ecs.");
        hostname.push_str(&self.region.to_string());
        hostname.push_str(".amazonaws.com");
        hostname
    }
}
