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
your behalf.  You can create access keys for your user by visiting the Identity and Access
Management (IAM) console, selecting 'Users' from the navigation sidebar, clicking on the name of
the IAM user for which you want to generate access keys (i.e. the user you created in the Setting
Up with Amazon ECS tutorial above), navigating to the 'Security Credentials'
tab, and clicking the 'Create Access Key' button.



## Approximate time spent
I spent about 50-60 hours coding for this project, and probably 20-30 hours reading through source
code and documentation for hyper, libsodium, serde, Amazon AWS, Amazon IAM, Amazon EC2, Amazon ECS,
and Amazon API Gateway.



## Accomplishments
The Amazon Signature Version 4 Signing process ended up requiring the bulk of the work for this
project.  I was able to install and link [libsodium](https://github.com/jedisct1/libsodium), a
major crytpography and authentication library, as an external build dependency and use the Rust
bindings provided by [Sodiumoxide](https://github.com/dnaq/sodiumoxide) to access libsodium's
crypto and authentication methods.  I was then able to implement the entire Signature Version 4
Signing algorithm used to authenticate requests to Amazon Web Services, as outlined 
[here](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html).
The process involves
1) building a 'canonical request' string with the hashed body payload and all request headers in a
particular order and format,
2) hashing the canonical request and building a 'string to sign' containing the signing algorithm,
date, and credential scope,
3) deriving a signing key from the user's credential and signing the string to sign from part 2),
and 4) building an Authentication header string containing the signature to be used in the HTTP
request.
The module `signature.rs` implements this signing algorithm and passes the unit tests provided
in the Amazon Signature Version 4 Signing Process Guide.



## Components, structure, design decisions
This client for Amazon ECS is designed as a simple wrapper struct over a hyper::Client.  The
client is passed the request parameters and handles building and sending the HTTP request in the
appropriate way.  I wanted to decouple the `hyper::client::request::Request` from the
`hyper::client::Client` so that requests could be built
up gradually by the user before being moved into the client to be sent to an Amazon ECS endpoint.
However, Hyper's `RequestBuilder` uses a consuming builder pattern which consumes the builder each
time something is added to the request, and the only way to create a `RequestBuilder` is with a
`Client`.  Because of this dependence I was unable to decouple hyper's `Request` and `RequestBuilder` from
hyper's `Client` by extending the consuming builder pattern in a struct separate from the `Client`.  I
was, however, able to build `ECSRequest` and `ECSClient` structs in a functionally independent way so
that users can build and manipulate their requests separately before handing them off to the client
to be forwarded to ECS.  All the types used to describe `Action`s with corresponding ECS request and
response types are defined in the `action` module.  This includes marker traits for writing
generic functions over `ECSRequest`s and `ECSResponse`s as well as structs to hold the request
parameters.  Right now the only API supported is the `ListClusters` action.

The `ecs_client.rs` module contains the `ECSClient`.  I was able to write the client in such a way
that makes expanding and adding new APIs relatively easy.  Most of the heavy lifting is done by
the `ECSClient::sign_and_send()` function which is generic over all types of ECSRequests.  The
particular API methods (such as `ECSClient::list_clusters()`) only pass on the relevant information
to `sign_and_send()` and deserialize the response to the proper `ECSResponse` type.  Adding more ECS
data types is also very easy with `serde`.  The [ECS data types](http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_Types.html)'s
descriptions have straightforward translations into Rust structs and simply need to be marked with
a `#[derive(Serialize, Deserialize)]` to make them json-(de)serializable by serde.  A function is
provided in `custom_ser.rs` which can be used along with serde's `#[serde(skip_serializing_if="$PATH")]`
annotation to skip serializing values which are `std::option::Option::None`.  Additionally, custom
`XAmzDate` and `XAmzTarget` hyper HTTP headers have been created in `custom_headers.rs` using
hyper macros.

The `region.rs` module defines the AWS regions in which Amazon ECS is supported, while the
`signature.rs` module implements Amazon's Signature Version 4 Signing Algorithm used to
add authentication information to the user's requests.  I have just begun to define some error
types in the `error.rs` module but they have not yet been integrated with the client.



## Testing approach and results
The testing suite for this project is minimal and is the first thing I would expand in the future.
I added unit-like tests at the end of some modules to test the correctness of lower level
functions in that module.  For instance, Amazon's online guide to the Signature Version 4 Signing
Algorithm had some examples of inputs and expected outputs for different parts of the signing
process which I copied over to my unit tests in `signature.rs`.  I also added some quick unit
tests to validate serde's serialization of the ListClusterRequest struct.

I added integration tests to `tests/lib.rs` to test the end-to-end functionality of the client. At
the moment, there is only one integration test to test an empty ListClusters request to Amazon ECS.
The integration test creates an `ECSClient` and a `ListClustersRequest`, uses the client to
send the request, and then prints the response.  The test passes as long as the client receives a
non-error response from the Amazon ECS endpoint.  Note that this test contacts the actual Amazon
ECS endpoint and therefore requires proper credentials to be stored in the environment of the
process in which the client is running.

A documentation test similar to the integration test described above has also been added to the
documentation at the crate level in `lib.rs`.  This provides an example to the user on how to use
the crate to make ECS requests.  As this example also contacts the real Amazon ECS endpoint, it
also requires proper credentials to be stored in the environment in order to pass.



## Limitations
The client is limited in that it can only make a `ListCluster`s request to Amazon ECS.  Ideally the
client's API will be expanded in the future to encompass ECS's complete API.

There is also definitely a lack of proper error-handling.  Each API request could potentially
return a number of
client- and server-side errors and so the client's API functions should return `Result`s.  I have
only just begun building the error types so that generic AWS errors can be abstracted into traits
which more specific ECS errors can then implement.  I have already added the type alias for the
custom error type to the `ecs_client.rs` module but I have not yet begun checking for and
deserializing errors and returning `Result`s.

As an SDK, the crate is missing many of the ECS data
types described at http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_Types.html because
I wanted to start with the simplest possible request before moving onto requests that require
types containing nested structs.  It is relatively straightforward to translate these data type
descriptions to Rust structs and mark them as `serde::ser::Serialize` or `serde::de::Deserialize`
as necessary, and this would be required in a minimum viable Amazon ECS SDK for Rust.  I would
particularly focus on the user interface for creating `TaskDefinition`s.  Most commonly,
users have their task definitions already written in json blobs in standalone .json files.  It
would be exceedingly helpful, then, to implement a `TaskDefinition::from_file($PATH)` function which
parses the blob at the end of the given `$PATH` and constructs the `TaskDefinition` struct in Rust
for you.  Using serde to construct the `TaskDefinition` from the .json file contents would be the
easiest way to implement such a method for this extremely common use case.

Though not necessary for a minimum viable product, it would also be nice to have some validations
to enforce parameter constraints when creating `ECSRequest`s.  This would help the user debug their
requests offline without having to wait for an `InvalidParameterValue` or similar error to be
returned from Amazon ECS.



## Postmortem
I think the overall designing and refactoring of the client went well.  The code base that I
currently have is in a good position to be added to and expanded and already provides some of the
marker types and generic functions necessary for that expansion.  I also think the user interface
exposed by the client is simple and easy to use, and in particular building and manipulating
requests based on the user code's own business logic is much easier with the request types
decoupled from the client.

The Amazon Version 4 Signing process was much more involved than I had initially anticipated.  It
involved manually building an HTTP-like request string in a particular format and sorted order and
hashing the strings together to derive the authenticaion information.  Building these strings to
Amazon's precise specifications was difficult and the current test suite in `signature.rs`
definitely must be expanded.  I would begin refactoring the current `signature.rs` module by first
building all the strings involved in the signing process using `std::format!()` instead of `.push_str()`.
That would probably be the easiest way to simplify my code and facilitate debugging any future
errors that might arise.
