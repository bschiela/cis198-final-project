//! A struct for a ListClusters request which can be serialized to json and set as the body of an
//! HTTP request.

use request::ecs_request::ECSRequest;

pub struct ListClustersRequest {
    /// The max number of cluster results returned in paginated output. 
    /// Must be between 1 and 100, inclusive.
    /// If omitted, defaults to 100.
    maxResults: Option<u8>,
    /// The value returned from a previous paginated request. 
    /// Pagination continues from the end of the previous results that returned the value.
    /// The value returned is null if there are no more results to return.
    nextToken: Option<String>,
}

impl ECSRequest for ListClustersRequest {}
