//! A struct for a ListClusters request which can be serialized to json and set as the body of an
//! HTTP request.

use request::ecs_request::ECSRequest;

pub struct ListClustersRequest {
    /// must be between 1 and 100, inclusive; defaults to 100 if missing or invalid
    max_results: Option<u8>,
    /// an optional token returned by a previous ListClusters request indicating where to start
    /// the next page of paginated output (if there are more results than max_results)
    next_token: Option<String>,
}

impl ECSRequest for ListClustersRequest {}
