extern crate ecs_client;

use ecs_client::ecs_client::ECSClient;
use ecs_client::action::list_clusters;
use ecs_client::region::Region;

#[test]
fn test_list_clusters_no_fields() {
    let ecs_client = ECSClient::for_region(Region::USWest2);
    let request = list_clusters::ListClustersRequest::new();
    let response = ecs_client.list_clusters(request);
    println!("got response...\n{:?}", response);
}
