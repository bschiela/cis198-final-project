//! This module contains the ECSClient which can be used to interact with Amazon ECS's API.
//! TODO provide examples

use aws_types::region::Region;
use hyper;

pub struct ECSClient {
    region: Region,
    client: hyper::Client
}

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
}
