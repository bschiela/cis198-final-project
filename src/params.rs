//! Defines structs which contain the relevant parameters for each type of request to Amazon ECS.
//! Each can be serialized to a json blob and set as the body of the HTTP request.

/// allowed parameters for a ListClusters request
pub struct ListClustersParams {
    /// must be between 1 and 100, inclusive; defaults to 100 if missing or invalid
    max_results: Option<u8>,
    /// an optional token returned by a previous ListClusters request indicating where to start
    /// the next page of paginated output (if there are more results than max_results)
    next_token: Option<String>,
}

// TODO expand API once ListClusters works

