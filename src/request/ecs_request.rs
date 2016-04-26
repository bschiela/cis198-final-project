//! Contains marker traits to "supertype" each particular kind of ECS request and response.
//! Each ECSRequest must be serde::ser::Serialize so that it can be serialized to json and
//! set as the payload (body) of an HTTP request.
//! Each ECSResponse must be serde::de::Deserialize so that it can be deserialized from the
//! body of the HTTP response.

use serde::ser::Serialize;
use serde::de::Deserialize;

pub trait ECSRequest : Serialize {}
pub trait ECSResponse : Deserialize {}
