//! A marker trait to "supertype" each particular kind of ECS request.  Each ECSRequest must be
//! serde::ser::Serialize so that it can be serialized to json and set as the payload of the
//! request in the HTTP body.

use serde::ser::Serialize;

pub trait ECSRequest : Serialize {}
