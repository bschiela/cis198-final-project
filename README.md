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
I spent about 50-60 hours coding for this project, and probably 20-30 hours reading
documentation for hyper, libsodium, serde, Amazon AWS, Amazon IAM, Amazon EC2 and Amazon ECS.



## Accomplishments
The Amazon Signature Version 4 Signing process ended up requiring the bulk of the work for this
project.  I was able to install and link [libsodium](https://github.com/jedisct1/libsodium), a
major crytpography and authentication library, as an external build dependency and use the Rust
bindings provided by [Sodiumoxide](https://github.com/dnaq/sodiumoxide) to access libsodium's
crypto methods.  I was then able to implement the entire Signature Version 4 Signing algorithm
used to authenticate requests to Amazon Web Services, as outlined [here](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html).
The process involves
1) building a 'canonical request' string with all request headers in a
particular order and format and with the hashed body payload
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
appropriate way.  I wanted to decouple the Request from the Client so that requests could be built
up gradually by the user before being moved into the client to be sent to an Amazon ECS endpoint.
However, Hyper's RequestBuilder uses a consuming builder pattern which consumes the builder each
time something is added to the request, and the only way to create a RequestBuilder is with a
Client.  Because of this dependence I was unable to decouple hyper's Request and RequestBuilder from
hyper's Client by extending the consuming builder pattern in a struct separate from the Client.  I
was, however, able to build ECSRequest and ECSClient structs in a functionally independent way so
that users can build and manipulate their requests separately before handing them off to the client
to be forwarded to ECS.  All the types used to describe Actions with corresponding ECS request and
response types are defined in the `action` module.  This includes marker traits for writing
generic functions over ECSRequests and ECSResponses as well as structs to hold the request
parameters.  Right now the only API supported is ListClusters.

The `ecs_client.rs` module contains the ECSClient.  I was able to write the client in such a way
that makes expanding and adding new APIs relatively easy.  Most of the heavy lifting is done by
the `ECSClient::sign_and_send()` function which is generic over all types of ECSRequests.  The
particular API methods (such as ECSClient::list_clusters()) only pass on the relevant information
to `sign_and_send()` and deserialize the response to the proper ECSResponse type.  Adding more ECS
data types is also made easy with `serde`.  The [ECS data types](http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_Types.html)
descriptions have straightforward translations into Rust structs and simply need to be marked with
a `#[derive(Serialize, Deserialize)]` to make them json-(de)serializable by serde.  A function is
provided in `custom_ser.rs` which can be used along with serde's `#[serde(skip_serializing_if="$PATH")]
annotation to skip serializing values which are `std::option::Option::None`.  Additionally, custom
`XAmzDate` and `XAmzTarget` hyper HTTP headers have been derived in `custom_headers.rs` using
hyper macros.

The `region.rs` module defines the AWS regions in which Amazon ECS is supported.  While the
`signature.rs` module implements Amazon's Signature Version 4 Signing Algorithm used to
add authentication information to the user's requests.  I have just begun to define some error
types in the `error.rs` module but they have not yet been integrated with the client.



## Testing approach and results
The testing suite for this project is minimal and is the first thing I would expand in the future.
I added unit-like tests at the end of some modules to test the correctness of lower level
functions in that module.  For instance, Amazon's online guide to the Signature Version 4 Signing
algorithm had some examples of inputs and expected outputs for different parts of the signing
process which I copied over to my unit tests in `signature.rs`.  I also added some quick unit
tests to validate serde's serialization of ListClusterRequest structs.

I added integration tests to `tests/lib.rs` to test the end-to-end functionality of the client. At
the moment, there is only one integration test to test an empty ListClusters request to Amazon ECS,
which is currently failing.  I was initially getting a 'BadRequest' error response back from ECS,
which I was able to fix by minimizing the headers I was adding to the request to the bare minimum
in order to simplify building the canonical request in the Signature Version 4 Signing process.
I'm now getting a 'Forbidden' error, leading me to believe that there is something wrong with the
IAM user's roles or permissions that are associated with the credentials I'm using to make the
test requests.



## Limitations
The client is limited in that it can only make a ListClusters request to Amazon ECS.  Ideally the
client's API would be expanded in the future to encompass ECS's complete API.  There is also
definitely a lack of proper error-handling.  Each API request could potentially return a number of
client- and server-side errors and so the client's API functions should return Results.  I have
only just begun building the error types so that generic AWS errors can be abstracted into traits
which more specific ECS errors can then implement.  I have already added the type alias for the
custom Error type to the `ecs_client.rs` module but I have not yet begun checking for and
deserializing errors and returning Results.  As an SDK, the crate is missing many of the ECS data
types described at http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_Types.html because
I wanted to start with the simplest possible request before moving onto requests which required
types containing nested structs.  It is relatively straightforward to translate these data type
descriptions to Rust structs and mark them as `serde::ser::Serialize` or `serde::de::Deserialize`
as necessary, and this would be required for a minimum viable Amazon ECS SDK for Rust.  I would
particularly focus on the user interface for creating TaskDefinitions.  The most common use case is
that users have their task definitions already written in json blobs in standalone .json files.  It
would be exceedingly helpful, then, to implement a TaskDefinition::from_file($PATH) function which
parses the blob at the end of the given $PATH and constructs the TaskDefinition Rust struct for
you.



## Postmortem
I think the overall designing and refactoring of the client went well.  The code base that I
currently have is in a good position to be added to and expanded and already provides some of the
marker types and generic functions necessary for that expansion.  I also think the user interface
exposed by the client is simple and easy to use, and in particular building and manipulating
requests based on the user code's own logic is much easier with the request types decoupled from
the client.

The Amazon Version 4 Signing process was much more involved than I initially had anticipated.  It
involved manually building an HTTP-like request string in a particular format and sorted order and
hashing the strings together to derive the authenticaion information.  Building these strings to
Amazon's precise specifications was difficult (sometimes on account of ambiguous documentation)
and there are probably still some bugs to be fixed, so the current test suite in `signature.rs`
must be expanded.  I made the mistake of adding all the typical AWS request headers rather than
starting with just the bare minimum number of headers to get a request working and had to simplify
my code to debug my integration tests.  I would begin debugging the current request error by first
building all the strings involved in the signing process using format!() instead of .push_str().
That would probably be the easiest way to simplify my code and facilitate debugging.
