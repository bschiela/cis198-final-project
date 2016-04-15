//! A marker trait to "supertype" each particular type of ECS request

use serde::ser::Serialize;

pub trait ECSRequest : Serialize {}
