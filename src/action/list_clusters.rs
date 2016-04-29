//! Defines request and response types for a ListClusters action.

use action::ecs_action::{ECSRequest, ECSResponse};
use custom_ser;

/// A ListClusters request type which can be serialized to json and set as the body of an HTTP
/// request.  Construct one of these and pass it to your client in the ecs_client::list_clusters()
/// function.
#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct ListClustersRequest {
    /// The max number of cluster results returned in paginated output. 
    /// Must be between 1 and 100, inclusive.
    /// If omitted, defaults to 100.
    #[serde(skip_serializing_if="custom_ser::is_none")]
    maxResults: Option<u8>,
    /// A value returned by the previous request indicating where to begin the next page of paginated
    /// output.  This value will be None if there are no more results to return.  Otherwise, this
    /// value will hold Some(String) which can be used in a subsequent ListClustersRequest to
    /// obtain the next page of paginated output, where the previous output exceeded 100 results
    /// or the value indicated by maxResults in the ListClustersRequest.
    #[serde(skip_serializing_if="custom_ser::is_none")]
    nextToken: Option<String>,
}

/// A ListClusters response type which can be deserialized from the body of an HTTP response. If
/// the request was successful, one of these will be returned from ecs_client::list_clusters().
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ListClustersResponse {
    /// The list of full Amazon Resource Name (ARN) entries for each cluster associated with your
    /// account.
    clusterArns: Vec<String>,
    /// A value returned by the previous request indicating where to begin the next page of paginated
    /// output.  This value will be None if there are no more results to return.  Otherwise, this
    /// value will hold Some(String) which can be used in a subsequent ListClustersRequest to
    /// obtain the next page of paginated output, where the previous output exceeded 100 results
    /// or the value indicated by maxResults in the ListClustersRequest.
    nextToken: Option<String>,
}

/// Used so that a ListClustersRequest can be passed as a generic ECSRequest.
impl ECSRequest for ListClustersRequest {}

/// Used so that a ListClustersResponse can be returned as a generic ECSResponse.
impl ECSResponse for ListClustersResponse {}

/// Implements some convenience methods for building a ListClustersRequest.
impl ListClustersRequest {
    /// Creates a default ListClustersRequest with no field values.
    pub fn new() -> Self {
        ListClustersRequest {
            maxResults: None,
            nextToken: None,
        }
    }

    /// Creates a ListClustersRequest with maxResults set but no nextToken.
    pub fn with_max_results(max_results: u8) -> Self {
        ListClustersRequest {
            maxResults: Some(max_results),
            nextToken: None,
        }
    }

    /// Creates a ListClustersRequest with nextToken set but no maxResults.
    pub fn with_next_token(next_token: String) -> Self {
        ListClustersRequest {
            maxResults: None,
            nextToken: Some(next_token),
        }
    }

    /// Creates a ListClustersRequest with all fields set.
    pub fn with_all(max_results: u8, next_token: String) -> Self {
        ListClustersRequest {
            maxResults: Some(max_results),
            nextToken: Some(next_token),
        }
    }
}

/// Implements some convenience methods for looking at values returned in a ListClustersResponse.
impl ListClustersResponse {
    /// Gets a reference to the Vec of Amazon Resource Name (ARN) Strings describing your Amazon
    /// ECS Compute Clusters.
    pub fn get_cluster_arns(&self) -> &Vec<String> {
        &self.clusterArns
    }

    /// Gets a reference to the nextToken value returned by the previous request.  This will
    /// return &None of there are no more results to display, otherwise it will return Some(String)
    /// which can be used in a subsequent request to get the next page of paginated output.
    pub fn get_next_token(&self) -> &Option<String> {
        &self.nextToken
    }
}

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
    fn test_value_order_irrelevant() {
        let req = ListClustersRequest {
            maxResults: Some(50),
            nextToken: Some(String::from("token")),
        };
        let expected: value::Value = serde_json::from_str("{\"nextToken\":\"token\",\"maxResults\":50}").unwrap();
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
        let expected: value::Value = serde_json::from_str("{}").unwrap();
        let actual: value::Value = value::to_value(&req);
        assert_eq!(expected, actual);

        let json_string = serde_json::to_string(&req).unwrap();
        println!("{}", &json_string);
    }
}
