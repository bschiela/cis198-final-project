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
/// A String constant used in deriving the signing key.
const AWS4: &'static str = "AWS4";
/// A String constant used in deriving the signing key.
const AWS4_REQUEST: &'static str = "aws4_request";

/// Calculates the Version 4 Signature according to the guidelines listed at
/// http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html .
pub fn calculate_signature(headers: &Headers, body: &str, region: Region, serv_abbrev: &str) -> String {
    let canonical_request = build_canonical_request(headers, body);
    let hashed_canonical_request = hash_to_hex(&canonical_request);
    let string_to_sign = build_string_to_sign(headers, region, serv_abbrev, &hashed_canonical_request);
    let signing_key = derive_signing_key(headers, region, serv_abbrev);
    unimplemented!()
}


/// Builds and returns the canonical request String according to the guidelines at
/// http://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html .
/// The canonical request contains the HTTP headers with lowercase names followed by their
/// value, with consecutive spaces converted to single spaces.  The headers must appear in
/// order sorted by character code in lowercase, followed by a list of headers included in the
/// signed request, followed by SHA256-hashed body.  The entire request is then hashed again
/// and returned as a String.
fn build_canonical_request(headers: &Headers, body: &str) -> String {
    let mut canon_req = String::from("POST\n");
    let mut signed_headers = String::new();
    canon_req.push_str("/\n"); // canonical URI (empty)
    canon_req.push_str("\n"); // canonical query string (empty)

    // CANONICAL HEADERS
    // must be sorted lexicographically by lowercase header name
    let accept_encoding: &AcceptEncoding = headers.get().unwrap();
    canon_req.push_str(&self::fmt_canonical_header(
            AcceptEncoding::header_name(),
            &(accept_encoding as &(HeaderFormat + Send + Sync)).to_string()
    ));
    signed_headers.push_str(&AcceptEncoding::header_name().to_lowercase());
    signed_headers.push_str(";");

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
    signed_headers.push_str("\n");

    // add list of signed headers in body
    canon_req.push_str("\n");
    canon_req.push_str(&signed_headers);

    // add hashed payload
    canon_req.push_str(&self::hash_to_hex(body));
    canon_req
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
    let mut hashed = String::new();
    for byte in &digest.0 {
        hashed.push_str(&format!("{:02x}", byte));
    }
    hashed
}

/// Builds the String To Sign according to the guidelines at
/// http://docs.aws.amazon.com/general/latest/gr/sigv4-create-string-to-sign.html .
/// The String To Sign is used with a derived signing key to calculate the signature.
fn build_string_to_sign(headers: &Headers,
                        region: Region,
                        serv_abbrev: &str,
                        hashed_canon_req: &str) -> String {
    // start with signing algorithm
    let mut string_to_sign = String::from(SIGNING_ALGORITHM);
    string_to_sign.push_str("\n");
    // followed by request date value
    let x_amz_date: &XAmzDate = headers.get().unwrap();
    let x_amz_date_val: &str = &(x_amz_date as &(HeaderFormat + Send + Sync)).to_string();
    string_to_sign.push_str(x_amz_date_val);
    string_to_sign.push_str("\n");
    // followed by credential scope
    string_to_sign.push_str(&build_credential_scope(x_amz_date_val, region, serv_abbrev));
    // followed by hashed canonical request
    string_to_sign.push_str(hashed_canon_req);
    string_to_sign
}

/// Builds the Credential Scope String, which is the date portion of the XAmzDate header,
/// followed by the region, followed by the service abbreviation, followed by the termination
/// string "aws4_request", (each separated by a "/" character), followed by a newline character.
/// For example:
/// 20160421/us-east-1/ecs/aws4_request\n
fn build_credential_scope(datetime: &str, region: Region, serv_abbrev: &str) -> String {
    let mut cred_scope = String::from(datetime.split("T").nth(1).unwrap());
    cred_scope.push_str("/");
    cred_scope.push_str(&region.to_string());
    cred_scope.push_str("/");
    cred_scope.push_str(serv_abbrev);
    cred_scope.push_str("/");
    cred_scope.push_str(TERMINATION_STRING);
    cred_scope.push_str("\n");
    cred_scope
}

/// Derives the signing key from your AWS secret access key, the date of your request, the service
/// name, and the region the request is being sent to.  AWS credentials are sourced from
/// environment variables.  Your AWS secret access key must be stored in an environment variable
/// called AWS_SECRET_ACCESS_KEY.
fn derive_signing_key(headers: &Headers, region: Region, serv_abbrev: &str) -> [u8; 32] {
    let mut init_key = String::from(AWS4);
    init_key.push_str(&get_aws_secret_access_key());
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

/// Gets your AWS Secret Access Key from the environment variable AWS_SECRET_ACCESS_KEY
fn get_aws_secret_access_key() -> String {
    match env::var(AWS_SECRET_ACCESS_KEY) {
        Ok(val) => {
            if val == "" {
                panic!("Your AWS Secret Access Key must be stored in the environment variable called AWS_SECRET_ACCESS_KEY.\n
                Try: \n $ export AWS_SECRET_ACCESS_KEY=\"yoursecretaccesskey\"\n");
            } else {
                val
            }
        }
        Err(e) => {
            println!("Couldn't interpret {}: {}", AWS_SECRET_ACCESS_KEY, e);
            panic!("Couldn't obtain AWS Secret Access Key from environment!");
        }
    }
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
        let aws_secret_access_key = "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY";
        env::set_var(super::AWS_SECRET_ACCESS_KEY, aws_secret_access_key);
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

    fn build_test_headers() -> Headers {
        let mut headers = Headers::new();
        headers.set(XAmzDate(String::from("20150830T000000")));
        headers
    }
}
