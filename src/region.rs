//! An enum defining the regions in which Amazon ECS is supported

use std::fmt::{Display, Formatter, Error};

#[derive(Clone, Copy)]
pub enum Region {
    USEast1,
    USWest1,
    USWest2,
    EUWest1,
    EUCentral1,
    APNortheast1,
    APSoutheast1,
    APSoutheast2,
}

/// Used primarily to map Region variants to their string representation
impl Display for Region {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let region_str = match self {
            &Region::USEast1 => "us-east-1",
            &Region::USWest1 => "us-west-1",
            &Region::USWest2 => "us-west-2",
            &Region::EUWest1 => "eu-west-1",
            &Region::EUCentral1 => "eu-central-1",
            &Region::APNortheast1 => "ap-northeast-1",
            &Region::APSoutheast1 => "ap-southeast-1",
            &Region::APSoutheast2 => "ap-southeast-2",
        };

        write!(f, "{}", region_str)
    }
}
