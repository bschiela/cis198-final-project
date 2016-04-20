//! This module implements Amazon's Signature Version 4 Signing Algorithm which is required in
//! order to make authenticated requests to any Amazon Web Service.

use hyper::header::{Headers, Header, HeaderFormat, Host, AcceptEncoding, ContentType, ContentLength};
use custom_headers::{XAmzTarget, XAmzDate};

/// The default algorithm used for calculating the authentication signature.
const SIGNING_ALGORITHM: &'static str = "AWS4-HMAC-SHA256";

/// Calculates the Version 4 Signature according to the guidelines listed at
/// http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html .
pub fn calculate_signature(headers: &Headers, body: &str) -> String {
    let canonical_request = build_canonical_request(headers, body);
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

    // add list of signed headers
    canon_req.push_str(&signed_headers);
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

