//! This module contains the ECSClient which can be used to interact with Amazon ECS's API.
//! The client sources your AWS credentials from the environment in which it is running.  You'll
//! need to create an AWS IAM user and add them to a security group with proper permissions, as
//! outlined here: http://docs.aws.amazon.com/AmazonECS/latest/developerguide/get-set-up-for-amazon-ecs.html .
//! Once you've created an IAM user, you can generate security credentials by going to the IAM
//! console, clicking on Users in the navigation pane, selecting your user from the list, choosing
//! the Security Credentials tab, and clicking Create Access Key.  This will generate an AWS access
//! key ID and an AWS secret access key.  Store these in the environment in which your client will
//! be running by setting the AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY, respectively, as such:
//!
//! ```text
//! $ export AWS_ACCESS_KEY_ID="your_access_key_id"
//! $ export AWS_SECRET_ACCESS_KEY="your_secret_access_key"
//! ```
//!
//! With these credentials created and stored appropriately, the ECSClient should be able to make
//! requests to Amazon ECS on your behalf.  Simply create an ECSClient, build a request, and call
//! the relevent API action on the client.  For example (note that the code below should panic if
//! no security credentials have been stored in the environment):
//!
//! ```should_panic
//! extern crate ecs_client;
//!
//! use ecs_client::ecs_client::ECSClient;
//! use ecs_client::region::Region;
//! use ecs_client::action::*;
//!
//! fn main() {
//!     let client = ECSClient::for_region(Region::USWest2);
//!     let request = list_clusters::ListClustersRequest::new();
//!     let response: list_clusters::ListClustersResponse = ecs_client.list_clusters(request);
//!     for cluster_arn in response.get_cluster_arns() {
//!         println!("{}", cluster_arn);
//!     }
//! }
//! ```

use region::Region;
use action::*;
use custom_headers::{XAmzTarget, XAmzDate};
use signature;
use error;

use hyper;
use hyper::header::{Headers, Host, AcceptEncoding, Encoding, qitem, ContentType, ContentLength, Authorization};
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde_json;
use time;
use std::result;
use std::io::Read;

/// The service abbreviation string for Amazon ECS.
const SERVICE_ABBREVIATION: &'static str = "ecs";
/// The MIME sublevel content type of an ECS HTTP request body.
const AMZ_SUBLEVEL_CONTENT_TYPE: &'static str = "x-amz-json-1.1";
/// The ECS API version this request is meant for.
const ECS_API_VERSION: &'static str = "AmazonEC2ContainerServiceV20141113";

/// A type alias to set the default error as an ECSError.
type Result<T> = result::Result<T, error::ECSError>;

#[derive(Debug)]
pub struct ECSClient {
    region: Region,
    client: hyper::Client
}

impl ECSClient {
    /// Creates a new ECSClient for the specified Region.
    pub fn for_region(region: Region) -> ECSClient {
        ECSClient {
            region: region,
            client: hyper::Client::new(),
        }
    }

    /// Sets the Region to which the client sends requests.
    pub fn set_region(&mut self, region: Region) {
        self.region = region;
    }

    /// Lists all of your compute clusters on ECS.
    pub fn list_clusters(&self, request: list_clusters::ListClustersRequest) -> list_clusters::ListClustersResponse {
        let mut response = self.sign_and_send(ecs_action::ECSAction::ListClusters, request);
        let mut response_body = String::new();
        response.read_to_string(&mut response_body).unwrap();
        let list_clusters_response: list_clusters::ListClustersResponse = serde_json::from_str(&response_body).unwrap();
        list_clusters_response
    }

    /// Creates an HTTP request to be sent to Amazon ECS.
    /// Signs the request using Amazon's Signature Version 4 Signing Algorithm.
    /// Serializes the service request to json format and sets it as the payload in the HTTP body.
    /// Sends the request to ECS and returns the hyper::client::Response.
    fn sign_and_send<T: ecs_action::ECSRequest>(&self,
                                                action: ecs_action::ECSAction,
                                                request: T) -> hyper::client::Response {
        let body: String = serde_json::to_string(&request).unwrap();
        let mut headers: Headers = self.build_headers(action, body.len() as u64);
        let auth_header = signature::build_auth_header(&headers, &body, self.region, SERVICE_ABBREVIATION);
        headers.set(Authorization(auth_header));
        
        let req_builder = self.client.post(&self.build_request_uri());

        println!("Sending request...\n{}", headers);
        println!("Request body...\n{}", body);
        let response = req_builder.headers(headers).body(&body).send();
        println!("Received response...\n{:?}", response);
        response.unwrap()
    }

    /// Builds the request URI based on the Region this client is currently configured to send
    /// requests to.
    fn build_request_uri(&self) -> String {
        let mut uri = String::from("https://");
        uri.push_str(&self.build_hostname());
        uri
    }

    /// Builds a hyper::header::Headers with the Host, Accept-Encoding, X-Amz-Target, X-Amz-Date,
    /// Content-Type, and Content-Length HTTP headers set.
    fn build_headers(&self, action: ecs_action::ECSAction, content_length: u64) -> Headers {
        let mut headers: Headers = Headers::new();
        headers.set(Host {
            hostname: self.build_hostname(),
            port: None,
        });
//        headers.set(AcceptEncoding(vec![qitem(Encoding::Identity)]));
//        headers.set(XAmzTarget(self.build_x_amz_target(action)));
        headers.set(XAmzDate(time::strftime("%Y%m%dT%H%M%SZ", &time::now_utc()).unwrap()));
//        headers.set(ContentType(
//                Mime(
//                    TopLevel::Application,
//                    SubLevel::Ext(String::from(AMZ_SUBLEVEL_CONTENT_TYPE)),
//                    vec![],
//                )
//            )
//        );
//        headers.set(ContentLength(content_length));
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
    fn build_x_amz_target(&self, action: ecs_action::ECSAction) -> String {
        let mut target = String::from(ECS_API_VERSION);
        target.push_str(".");
        target.push_str(&action.to_string());
        target
    }
}
