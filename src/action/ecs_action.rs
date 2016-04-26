//! This module contains types which are useful for describing actions that the ECSClient
//! can perform on your behalf.

use std::fmt::{Display, Formatter, Error};
use serde::ser::Serialize;
use serde::de::Deserialize;

/// An enum defining the possible actions which can be sent to Amazon ECS.
#[derive(Clone, Copy, Debug)]
pub enum ECSAction {
    ListClusters,
}

/// Used primarily to map an ECSAction to its corresponding string representation to be used in
/// the X-Amz-Target header.
impl Display for ECSAction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let action_str = match self {
            &ECSAction::ListClusters => "ListClusters",
        };

        write!(f, "{}", action_str)
    }
}

/// A marker trait to "supertype" each particular kind of ECS request.  Each ECSRequest must be
/// serde::ser::Serialize so that it can be serialzied to json and set as the payload (body)
/// of an HTTP request.
pub trait ECSRequest : Serialize {}

/// A marker trait to "supertype" each particular kind of ECS response.  Each ECSResponse must be
/// serde::de::Deserialize so that it can be deserialized from the json blob in the body of the
/// HTTP response.
pub trait ECSResponse : Deserialize {}
