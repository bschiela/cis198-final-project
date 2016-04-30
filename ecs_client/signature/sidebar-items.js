initSidebarItems({"constant":[["AWS4","A String constant used in deriving the signing key."],["AWS4_REQUEST","A String constant used in deriving the signing key."],["AWS_ACCESS_KEY_ID","The name of the environment variable in which your AWS Access Key ID should be stored."],["AWS_SECRET_ACCESS_KEY","The name of the environment variable in which your AWS Secret Access Key should be stored."],["CREDENTIAL","The Credential key string used in the Authorization header."],["SIGNATURE","The Signature key string used in the Authorization header."],["SIGNED_HEADERS","The SignedHeaders key string used in the Authorization header."],["SIGNING_ALGORITHM","The default algorithm used for calculating the authentication signature."],["TERMINATION_STRING","The termination string used in the credential scope value."]],"fn":[["build_auth_header","Builds the Authorization HTTP header with all the required authentication information prescribed in http://docs.aws.amazon.com/general/latest/gr/sigv4-add-signature-to-request.html . Starts with the Signing Algorithm used, followed by a 'Credential=' key field, followed by your AWS Access Key ID (which is sourced from the environment variable AWS_ACCESS_KEY_ID), followed by the credential scope created during the signing process, followed by a comma, followed by a 'SignedHeaders=' key string with the signed_headers from the signing process following as the field, followed by the 'Signature=' key string followed by the signature calculated during the signing process!"],["build_canonical_request","Builds and returns the canonical request String according to the guidelines at http://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html . The canonical request contains the HTTP headers with lowercase names followed by their value, with consecutive spaces converted to single spaces.  The headers must appear in order sorted by character code in lowercase, followed by a list of headers included in the signed request, followed by SHA256-hashed body.  The entire request is then hashed again and returned in a (String, String)=(canonical_request, signed_headers) tuple so that the signed_headers value used during the signing process can be built into the Authorization HTTP header."],["build_credential_scope","Builds the Credential Scope String, which is the date portion of the XAmzDate header, followed by the region, followed by the service abbreviation, followed by the termination string \"aws4_request\", (each separated by a \"/\" character), followed by a newline character. For example: 20160421/us-east-1/ecs/aws4_request\\n"],["build_string_to_sign","Builds the String To Sign according to the guidelines at http://docs.aws.amazon.com/general/latest/gr/sigv4-create-string-to-sign.html . The String To Sign is used with a derived signing key to calculate the signature. Returns a tuple containing (string_to_sign, credential_scope) so that credential scope value can be reused in the Authorization HTTP header."],["calculate_signature","Calculates the Version 4 Signature according to the guidelines listed at http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html . Returns the (signature, credential_scope, signed_headers) in a triple-tuple so that these values can be used to construct the Authorization HTTP header."],["derive_signing_key","Derives the signing key from your AWS secret access key, the date of your request, the service name, and the region the request is being sent to.  AWS credentials are sourced from environment variables.  Your AWS secret access key must be stored in an environment variable called AWS_SECRET_ACCESS_KEY."],["fmt_canonical_header","Formats a single header according to the canonical format.  Header names must appear in lowercase, followed by a ':', followed by the header value, with consecutive spaces converted to single spaces.  A newline character terminates the String."],["get_from_environment","Gets the environment variable env_var_name from your current environment.  This is used to get your AWS Access Key ID from the AWS_ACCESS_KEY_ID environment variable and your AWS Secret  Key ID from the AWS_SECRET_KEY_ID environment variable.  If these variables aren't set, the client will panic."],["hash_to_hex","Hashes the input &str using SHA256, and converts the resulting digest to a lowercase hexadecimal String."],["hex_encode","Encodes the input byte slice as a lowercase hexadecimal value."],["sign","Signs the 'string to sign' and returns the calculated signature."]]});