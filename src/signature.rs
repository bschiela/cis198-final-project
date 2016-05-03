//! This module implements Amazon's Signature Version 4 Signing Algorithm which is required in
//! order to make authenticated requests to any Amazon Web Service.

use hyper::header::{Headers, Header, HeaderFormat, Host, AcceptEncoding, ContentType, ContentLength};
use custom_headers::{XAmzTarget, XAmzDate};
use sodiumoxide::crypto::hash::sha256;
use sodiumoxide::crypto::auth::hmacsha256::State;
use region::Region;
use std::env;

/// The default algorithm used for calculating the authentication signature.
const SIGNING_ALGORITHM: &'static str = "AWS4-HMAC-SHA256";
/// The termination string used in the credential scope value.
const TERMINATION_STRING: &'static str = "aws4_request";
/// The name of the environment variable in which your AWS Secret Access Key should be stored.
const AWS_SECRET_ACCESS_KEY: &'static str = "AWS_SECRET_ACCESS_KEY";
/// The name of the environment variable in which your AWS Access Key ID should be stored.
const AWS_ACCESS_KEY_ID: &'static str = "AWS_ACCESS_KEY_ID";
/// A String constant used in deriving the signing key.
const AWS4: &'static str = "AWS4";
/// A String constant used in deriving the signing key.
const AWS4_REQUEST: &'static str = "aws4_request";
/// The Credential key string used in the Authorization header.
const CREDENTIAL: &'static str = "Credential";
/// The SignedHeaders key string used in the Authorization header.
const SIGNED_HEADERS: &'static str = "SignedHeaders";
/// The Signature key string used in the Authorization header.
const SIGNATURE: &'static str = "Signature";

/// Builds the Authorization HTTP header with all the required authentication information
/// prescribed in http://docs.aws.amazon.com/general/latest/gr/sigv4-add-signature-to-request.html .
/// Starts with the Signing Algorithm used, followed by a 'Credential=' key field, followed by your
/// AWS Access Key ID (which is sourced from the environment variable AWS_ACCESS_KEY_ID), followed
/// by the credential scope created during the signing process, followed by a comma, followed by
/// a 'SignedHeaders=' key string with the signed_headers from the signing process following as
/// the field, followed by the 'Signature=' key string followed by the signature calculated during
/// the signing process!
pub fn build_auth_header(headers: &Headers, body: &str, region: Region, serv_abbrev: &str) -> String {
    let (signature, credential_scope, signed_headers) = calculate_signature(headers, body, region, serv_abbrev);
    let mut auth_header = String::from(SIGNING_ALGORITHM);
    auth_header.push_str(" ");

    auth_header.push_str(CREDENTIAL);
    auth_header.push_str("=");
    auth_header.push_str(&get_from_environment(AWS_ACCESS_KEY_ID));
    auth_header.push_str("/");
    auth_header.push_str(&credential_scope);
    println!("CREDENTIAL SCOPE!! {}", credential_scope);
    auth_header.push_str(", ");

    auth_header.push_str(SIGNED_HEADERS);
    auth_header.push_str("=");
    auth_header.push_str(&signed_headers);
    auth_header.push_str(", ");

    auth_header.push_str(SIGNATURE);
    auth_header.push_str("=");
    auth_header.push_str(&signature);
    println!("auth_header={}", auth_header);
    auth_header
}

/// Calculates the Version 4 Signature according to the guidelines listed at
/// http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html .
/// Returns the (signature, credential_scope, signed_headers) in a triple-tuple so that
/// these values can be used to construct the Authorization HTTP header.
pub fn calculate_signature(headers: &Headers,
                           body: &str,
                           region: Region,
                           serv_abbrev: &str) -> (String, String, String) {
    let (canonical_request, signed_headers) = build_canonical_request(headers, body);
    let hashed_canonical_request = hash_to_hex(&canonical_request);
    let (string_to_sign, credential_scope) = build_string_to_sign(headers, region, serv_abbrev, &hashed_canonical_request);
    let signing_key = derive_signing_key(headers, region, serv_abbrev);
    let signature = sign(&signing_key, &string_to_sign);
    (signature, credential_scope, signed_headers)
}


/// Builds and returns the canonical request String according to the guidelines at
/// http://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html .
/// The canonical request contains the HTTP headers with lowercase names followed by their
/// value, with consecutive spaces converted to single spaces.  The headers must appear in
/// order sorted by character code in lowercase, followed by a list of headers included in the
/// signed request, followed by SHA256-hashed body.  The entire request is then hashed again
/// and returned in a (String, String)=(canonical_request, signed_headers) tuple so that the
/// signed_headers value used during the signing process can be built into the Authorization
/// HTTP header.
fn build_canonical_request(headers: &Headers, body: &str) -> (String, String) {
    let mut canon_req = String::from("POST\n");
    let mut signed_headers = String::new();
    canon_req.push_str("/\n"); // canonical URI (empty)
    canon_req.push_str("\n"); // canonical query string (empty)

    // CANONICAL HEADERS
    // must be sorted lexicographically by lowercase header name
/*    let accept_encoding: &AcceptEncoding = headers.get().unwrap();
    canon_req.push_str(&self::fmt_canonical_header(
            AcceptEncoding::header_name(),
            &(accept_encoding as &(HeaderFormat + Send + Sync)).to_string()
    ));
    signed_headers.push_str(&AcceptEncoding::header_name().to_lowercase());
    signed_headers.push_str(";");
*/
    let content_length: &ContentLength = headers.get().unwrap();
    canon_req.push_str(&self::fmt_canonical_header(
            ContentLength::header_name(),
            &(content_length as &(HeaderFormat + Send + Sync)).to_string()
    ));
    signed_headers.push_str(&ContentLength::header_name().to_lowercase());
    signed_headers.push_str(";");

    let content_type: &ContentType = headers.get().unwrap();
    canon_req.push_str(&self::fmt_canonical_header(
            ContentType::header_name(),
            &(content_type as &(HeaderFormat + Send + Sync)).to_string()
    ));
    signed_headers.push_str(&ContentType::header_name().to_lowercase());
    signed_headers.push_str(";");

    let host: &Host = headers.get().unwrap();
    canon_req.push_str(&self::fmt_canonical_header(
            Host::header_name(),
            &(host as &(HeaderFormat + Send + Sync)).to_string()
    ));
    signed_headers.push_str(&Host::header_name().to_lowercase());
    signed_headers.push_str(";");

    let x_amz_date: &XAmzDate = headers.get().unwrap();
    canon_req.push_str(&self::fmt_canonical_header(
            XAmzDate::header_name(),
            &(x_amz_date as &(HeaderFormat + Send + Sync)).to_string()
    ));
    signed_headers.push_str(&XAmzDate::header_name().to_lowercase());
    signed_headers.push_str(";");

    let x_amz_target: &XAmzTarget = headers.get().unwrap();
    canon_req.push_str(&self::fmt_canonical_header(
            XAmzTarget::header_name(),
            &(x_amz_target as &(HeaderFormat + Send + Sync)).to_string()
    ));
    signed_headers.push_str(&XAmzTarget::header_name().to_lowercase());

    // add list of signed headers in body
    canon_req.push_str("\n");
    canon_req.push_str(&signed_headers);
    canon_req.push_str("\n");

    // add hashed payload
    canon_req.push_str(&self::hash_to_hex(body));
    (canon_req, signed_headers)
}

/// Formats a single header according to the canonical format.  Header names must appear in
/// lowercase, followed by a ':', followed by the header value, with consecutive spaces
/// converted to single spaces.  A newline character terminates the String.
fn fmt_canonical_header(name: &str, value: &str) -> String {
    let mut header = String::from(name).to_lowercase();
    header.push_str(":");
    header.push_str(value);
    // convert sequential spaces to single spaces
    let mut canon_header = String::new();
    for token in header.split_whitespace() {
        canon_header.push_str(token);
        canon_header.push_str(" ");
    }
    // remove trailing space and append newline
    canon_header = String::from(canon_header.trim_right());
    canon_header.push_str("\n");
    canon_header
}

/// Hashes the input &str using SHA256, and converts the resulting digest to a lowercase
/// hexadecimal String.
fn hash_to_hex(input: &str) -> String {
    let digest = sha256::hash(input.as_bytes());
    hex_encode(&digest.0)
}

/// Encodes the input byte slice as a lowercase hexadecimal value.
fn hex_encode(digest: &[u8; 32]) -> String {
    let mut hex = String::new();
    for byte in digest {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

/// Builds the String To Sign according to the guidelines at
/// http://docs.aws.amazon.com/general/latest/gr/sigv4-create-string-to-sign.html .
/// The String To Sign is used with a derived signing key to calculate the signature.
/// Returns a tuple containing (string_to_sign, credential_scope) so that credential scope value
/// can be reused in the Authorization HTTP header.
fn build_string_to_sign(headers: &Headers,
                        region: Region,
                        serv_abbrev: &str,
                        hashed_canon_req: &str) -> (String, String) {
    // start with signing algorithm
    let mut string_to_sign = String::from(SIGNING_ALGORITHM);
    string_to_sign.push_str("\n");
    // followed by request date value
    let x_amz_date: &XAmzDate = headers.get().unwrap();
    let x_amz_date_val: &str = &(x_amz_date as &(HeaderFormat + Send + Sync)).to_string();
    string_to_sign.push_str(x_amz_date_val);
    string_to_sign.push_str("\n");
    // followed by credential scope
    let credential_scope = build_credential_scope(x_amz_date_val, region, serv_abbrev);
    string_to_sign.push_str(&credential_scope);
    string_to_sign.push_str("\n");
    // followed by hashed canonical request
    string_to_sign.push_str(hashed_canon_req);
    (string_to_sign, credential_scope)
}

/// Builds the Credential Scope String, which is the date portion of the XAmzDate header,
/// followed by the region, followed by the service abbreviation, followed by the termination
/// string "aws4_request", (each separated by a "/" character), followed by a newline character.
/// For example:
/// 20160421/us-east-1/ecs/aws4_request\n
fn build_credential_scope(datetime: &str, region: Region, serv_abbrev: &str) -> String {
    let mut cred_scope = String::from(datetime.split("T").nth(0).unwrap());
    cred_scope.push_str("/");
    cred_scope.push_str(&region.to_string());
    cred_scope.push_str("/");
    cred_scope.push_str(serv_abbrev);
    cred_scope.push_str("/");
    cred_scope.push_str(TERMINATION_STRING);
    cred_scope
}

/// Derives the signing key from your AWS secret access key, the date of your request, the service
/// name, and the region the request is being sent to.  AWS credentials are sourced from
/// environment variables.  Your AWS secret access key must be stored in an environment variable
/// called AWS_SECRET_ACCESS_KEY.
fn derive_signing_key(headers: &Headers, region: Region, serv_abbrev: &str) -> [u8; 32] {
    let mut init_key = String::from(AWS4);
    init_key.push_str(&get_from_environment(AWS_SECRET_ACCESS_KEY));
    let date: &XAmzDate = headers.get().unwrap();
    let date_val = date.0.split("T").nth(0).unwrap(); // use only the date portion
    // derive the key
    println!("init_key={}", init_key);
    let mut state = State::init(&init_key.as_bytes());

    println!("hashing date={}", date_val);
    state.update(&date_val.as_bytes());
    let date_key = state.finalize();

    println!("hashing region={}", region.to_string());
    state = State::init(&date_key.0);
    state.update(&region.to_string().as_bytes());
    let region_key = state.finalize();

    println!("hashing service={}", serv_abbrev);
    state = State::init(&region_key.0);
    state.update(&serv_abbrev.as_bytes());
    let service_key = state.finalize();

    println!("hashing termination_string={}", AWS4_REQUEST);
    state = State::init(&service_key.0);
    state.update(&AWS4_REQUEST.as_bytes());
    let signing_key = state.finalize();

    println!("derived signing key = {:?}", signing_key.0);
    signing_key.0
}

/// Gets the environment variable env_var_name from your current environment.  This is used to get
/// your AWS Access Key ID from the AWS_ACCESS_KEY_ID environment variable and your AWS Secret 
/// Key ID from the AWS_SECRET_KEY_ID environment variable.  If these variables aren't set, the
/// client will panic.
fn get_from_environment(env_var_name: &str) -> String {
    match env::var(env_var_name) {
        Ok(val) => {
            if val == "" {
                println!("Your AWS Secret Access Key and AWS Access Key ID must be stored in the \
                    environment variables called AWS_SECRET_ACCESS_KEY and AWS_ACCESS_KEY_ID, \
                    respectively.\nTry:\n$ export AWS_SECRET_ACCESS_KEY=\"your_secret_access_key\"\n\
                    $ export AWS_ACCESS_KEY_ID=\"your_access_key_id\"\n");
                panic!("Couldn't obtain AWS credentials from environment!");
            } else {
                val
            }
        }
        Err(e) => {
            println!("Couldn't interpret {}: {}", env_var_name, e);
            println!("Your AWS Secret Access Key and AWS Access Key ID must be stored in the \
                environment variables called AWS_SECRET_ACCESS_KEY and AWS_ACCESS_KEY_ID, \
                respectively.\nTry:\n$ export AWS_SECRET_ACCESS_KEY=\"your_secret_access_key\"\n\
                $ export AWS_ACCESS_KEY_ID=\"your_access_key_id\"\n");
            panic!("Couldn't obtain AWS credentials from environment!");
        }
    }
}

/// Signs the 'string to sign' and returns the calculated signature.
fn sign(signing_key: &[u8; 32], string_to_sign: &str) -> String {
    let mut state = State::init(signing_key);
    state.update(&string_to_sign.as_bytes());
    let signature = state.finalize();
    hex_encode(&signature.0)
}

#[cfg(test)]
mod test {
    use std::env;
    use region::Region;
    use hyper::header::Headers;
    use custom_headers::XAmzDate;

    #[test]
    fn test_digest_to_hex() {
        // expected hash value of an empty string
        let expected = String::from("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        println!("{}", super::hash_to_hex(""));
        assert_eq!(expected, super::hash_to_hex(""))
    }
    
    // using the example at http://docs.aws.amazon.com/general/latest/gr/sigv4-calculate-signature.html
    #[test]
    fn test_derive_signing_key() {
        init_test_state();
        let test_headers = build_test_headers();
        
        let expected_bytes = vec![196, 175, 177, 204, 87, 113, 216, 113, 118, 58, 57, 62, 68,
            183, 3, 87, 27, 85, 204, 40, 66, 77, 26, 94, 134, 218, 110, 211, 193, 84, 164, 185];
        let result = super::derive_signing_key(&test_headers, Region::USEast1, "iam");
        assert_eq!(32, result.len());
        for (i, byte) in result.iter().enumerate() {
            println!("{}", byte);
            assert_eq!(*expected_bytes.get(i).unwrap(), *byte as i32);
        }
    }

    #[test]
    fn test_sign() {
        init_test_state();
        let test_headers = build_test_headers();
        let signing_key = super::derive_signing_key(&test_headers, Region::USEast1, "iam");
        let string_to_sign = 
            "AWS4-HMAC-SHA256\n\
            20150830T123600Z\n\
            20150830/us-east-1/iam/aws4_request\n\
            f536975d06c0309214f805bb90ccff089219ecd68b2577efef23edd43b7e1a59";
        let expected = "5d672d79c15b13162d9279b0855cfba6789a8edb4c82c400e06b5924a6f2b5d7";
        let result = super::sign(&signing_key, &string_to_sign);
        assert_eq!(expected, result);
    }

    fn build_test_headers() -> Headers {
        let mut headers = Headers::new();
        headers.set(XAmzDate(String::from("20150830T000000")));
        headers
    }

    fn init_test_state() {
        let aws_secret_access_key = "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY";
        env::set_var(super::AWS_SECRET_ACCESS_KEY, aws_secret_access_key);
    }
}
