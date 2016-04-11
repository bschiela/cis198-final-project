//! This module contains the ECSClient which can be used to interact with Amazon ECS's API.
//! TODO provide examples

use aws_types::region::Region;

pub struct ECSClient {
    region: Region
}
