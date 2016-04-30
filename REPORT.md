# Rust AWS Client for Amazon EC2 Container Service
William Schiela

## Summary
For this project I built an Amazon Web Services client for Amazon ECS.  The only API currently
supported is a ListClusters action.  A user can import this crate, create an `ECSClient` and a
`ListClustersRequest`, and query Amazon ECS's HTTP API for a list of Amazon Resource Names (ARNs)
describing the user's clusters in a particular AWS region.  The client handles setting the required
HTTP headers for the request, serializing the request parameters to a json blob in the body of the
HTTP request, signing the request using Amazon's Signature Version 4 Signing algorithm to derive
authentication keys, and deserializing and returning the response from Amazon ECS.  A user of this
crate need only build the request with the desired parameters and pass it to the corresponding API
in the client.  For example:

```rust
extern crate ecs_client;

use ecs_client::ecs_client::ECSClient;
use ecs_client::region::Region;
use ecs_client::action::*;

let client = ECSClient::for_region(Region::USWest2);
let request = list_clusters::ListClustersRequest::new();
let response: list_clusters::ListClustersResponse = ecs_client.list_clusters(request);
for cluster_arn in response.get_cluster_arns() {
    println!("{}", cluster_arn);
}
```

Note that the client sources your AWS credentials from the environment of the current process.
You'll have to set up an IAM user with particular permissions as outlined in the
[Setting Up with Amazon ECS Guide](http://docs.aws.amazon.com/AmazonECS/latest/developerguide/get-set-up-for-amazon-ecs.html).
Once you generate an AWS access key ID and an AWS secret access key, they should be stored in the
environment variables AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY, respectively, like so:

```
$ export AWS_ACCESS_KEY_ID="accessKeyIdExample"
$ expost AWS_SECRET_ACCESS_KEY="secretAccessKeyExample"
```

Then the client running in that environment will be able to make API requests to Amazon ECS on
your behalf.



## Approximate time spent

## Accomplishments
Libsodium binding and AWS SigV4 algorithm

## Components, structure, design decisions
Tried to decouple Client and Request but too tightly coupled
Easily expandable
Go through each module

## Testing approach and results
Unit tests for sigv4 and serializing
Integ tests for end-to-end

## Limitations
Only one type of request
No error handling
Limited ECS datatypes (would have made a TaskDefinition::from_file(PATH) function)

## Postmortem
What went well
-- design and decoupling ECSRequests from ECSClient, generic client implementations
		make new API requests easy to implement
--
What would do differently
-- Use format!() when trying to build long strings with lots of inputs
-- read the documentation and source code of main dependencies (i.e. hyper) to guide
		further design decisions (wasted time with RequestBuilder being a consuming builder)
