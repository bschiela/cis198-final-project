initSidebarItems({"constant":[["SIGNING_ALGORITHM","The default algorithm used for calculating the authentication signature."]],"fn":[["build_canonical_request","Builds and returns the canonical request String according to the guidelines at http://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html . The canonical request contains the HTTP headers with lowercase names followed by their value, with consecutive spaces converted to single spaces.  The headers must appear in order sorted by character code in lowercase, followed by a list of headers included in the signed request, followed by SHA256-hashed body.  The entire request is then hashed again and returned as a String."],["calculate_signature","Calculates the Version 4 Signature according to the guidelines listed at http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html ."],["fmt_canonical_header","Formats a single header according to the canonical format.  Header names must appear in lowercase, followed by a ':', followed by the header value, with consecutive spaces converted to single spaces.  A newline character terminates the String."],["hash_to_hex","Hashes the input &str using SHA256, and converts the resulting digest to a lowercase hexadecimal String."]]});