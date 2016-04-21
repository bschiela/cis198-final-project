//! This module contains the ECSClient which can be used to interact with Amazon ECS's API.

use region::Region;
use action::ECSAction;
use request::*;
use custom_headers::{XAmzTarget, XAmzDate};
use signature;

use hyper;
use hyper::header::{Headers, Host, AcceptEncoding, Encoding, qitem, ContentType, ContentLength, Authorization};
use hyper::mime::{Mime, TopLevel, SubLevel};

use serde_json;

use time;

/// The service abbreviation string for Amazon ECS.
const SERVICE_ABBREVIATION: &'static str = "ecs";
/// The MIME sublevel content type of an ECS HTTP request body.
const AMZ_SUBLEVEL_CONTENT_TYPE: &'static str = "x-amz-json-1.1";
/// The ECS API version this request is meant for.
const ECS_API_VERSION: &'static str = "AmazonEC2ContainerServiceV20141113";

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

    /// Creates an HTTP request to be sent to Amazon ECS.
    /// Signs the request using Amazon's Signature Version 4 Signing Algorithm.
    /// Serializes the service request to json format and sets it as the payload in the HTTP body.
    /// Sends the request to ECS and returns the response.
    fn sign_and_send<T: ecs_request::ECSRequest>(&self,
                                                 action: ECSAction,
                                                 request: T) -> i32 {
        let body: String = serde_json::to_string(&request).unwrap();
        let mut headers: Headers = self.build_headers(action, body.len() as u64);
        let signature = signature::calculate_signature(&headers, &body, self.region, SERVICE_ABBREVIATION);
        headers.set(Authorization(signature));
        
        let req_builder = self.client.post("/");

        // TODO return response (change return value from i32)
        let response = req_builder.headers(headers).body(&body).send();
        unimplemented!()
    }

    /// Builds a hyper::header::Headers with the Host, Accept-Encoding, X-Amz-Target, X-Amz-Date,
    /// Content-Type, and Content-Length HTTP headers set.
    fn build_headers(&self, action: ECSAction, content_length: u64) -> Headers {
        let mut headers: Headers = Headers::new();
        headers.set(Host {
            hostname: self.build_hostname(),
            port: None,
        });
        headers.set(AcceptEncoding(vec![qitem(Encoding::Identity)]));
        headers.set(XAmzTarget(self.build_x_amz_target(action)));
        headers.set(XAmzDate(time::strftime("%Y%m%dT%H%M%SZ", &time::now_utc()).unwrap()));
        headers.set(ContentType(
                Mime(
                    TopLevel::Application,
                    SubLevel::Ext(String::from(AMZ_SUBLEVEL_CONTENT_TYPE)),
                    vec![],
                )
            )
        );
        headers.set(ContentLength(content_length));
        headers
    }

    /// Builds and returns the hostname String used in the Host header.
    fn build_hostname(&self) -> String {
        let mut hostname = String::from(SERVICE_ABBREVIATION);
        hostname.push_str(".");
        hostname.push_str(&self.region.to_string());
        hostname.push_str(".amazonaws.com");
        hostname
    }

    /// Builds and returns the target String used in the X-Amz-Target header.
    fn build_x_amz_target(&self, action: ECSAction) -> String {
        let mut target = String::from(ECS_API_VERSION);
        target.push_str(".");
        target.push_str(&action.to_string());
        target
    }
}
