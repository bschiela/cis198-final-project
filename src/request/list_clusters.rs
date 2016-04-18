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
    use serde_json::value;

    #[test]
    fn test_full_blob() {
        let req = ListClustersRequest {
            maxResults: Some(50),
            nextToken: Some(String::from("token")),
        };
        let expected: value::Value = serde_json::from_str("{\"maxResults\":50,\"nextToken\":\"token\"}").unwrap();
        let actual: value::Value = value::to_value(&req);
        assert_eq!(expected, actual);

        let json_string = serde_json::to_string(&req).unwrap();
        println!("{}", &json_string);
    }

    #[test]
    fn test_empty_blob() {
        let req = ListClustersRequest {
            maxResults: None,
            nextToken: None,
        };
        let expected: value::Value = serde_json::from_str("{\"maxResults\":null,\"nextToken\":null}").unwrap();
        let actual: value::Value = value::to_value(&req);
        assert_eq!(expected, actual);

        let json_string = serde_json::to_string(&req).unwrap();
        println!("{}", &json_string);
    }
}
