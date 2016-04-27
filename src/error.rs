//! This module defines Error types that may be returned when sending a request to Amazon ECS.
//! Note that the ECS API reference defines 'Exception' types, but we map each Amazon ECS 
//! Exception to a Rust Error.

// note: might have to make this serde::de::Deserialize depending on how errors are returned.
/// A general ECSError.
pub struct ECSError {
    /// The status code that was returned with the HTTP response.
    status_code: u16,
//    /// The error message that was returned with the HTTP response.
//    message: String,
}
