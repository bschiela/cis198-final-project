//! A struct for a ListClusters request which can be serialized to json and set as the body of an
//! HTTP request.

use request::ecs_request::ECSRequest;

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
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

/// For writing generic functions over ECSRequests
impl ECSRequest for ListClustersRequest {}

#[cfg(test)]
mod test {
    use super::ListClustersRequest;
    use serde_json;

    #[test]
    fn test_full_blob() {
        let req = ListClustersRequest {
            maxResults: Some(50),
            nextToken: Some(String::from("token")),
        };
        let ser = serde_json::to_string(&req).unwrap();
        assert_eq!("{\"maxResults\":50,\"nextToken\":\"token\"}", &ser);
        println!("{}", &ser);
    }

    #[test]
    fn test_empty_blob() {
        let req = ListClustersRequest {
            maxResults: None,
            nextToken: None,
        };
        let ser = serde_json::to_string(&req).unwrap();
        assert_eq!("{\"maxResults\":null,\"nextToken\":null}", &ser);
        println!("{}", &ser);
    }
}
