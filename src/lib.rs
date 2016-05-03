//! This crate contains an ECSClient which can be used to interact with Amazon ECS's API.
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
//! ```
//! extern crate ecs_client;
//!
//! use ecs_client::ecs_client::ECSClient;
//! use ecs_client::region::Region;
//! use ecs_client::action::*;
//!
//! fn main() {
//!     let ecs_client = ECSClient::for_region(Region::USWest2);
//!     let request = list_clusters::ListClustersRequest::new();
//!     let response: list_clusters::ListClustersResponse = ecs_client.list_clusters(request);
//!     for cluster_arn in response.get_cluster_arns() {
//!         println!("{}", cluster_arn);
//!     }
//! }
//! ```

#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate hyper;
extern crate time;
extern crate sodiumoxide; // for bindings to a crypto library

#[cfg(feature = "serde_macros")]
include!("lib.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
