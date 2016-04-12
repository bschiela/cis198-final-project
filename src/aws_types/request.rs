//! A general, Signature Version 4 Signable AWS Request

use std::cell::RefCell;
use std::rc::Rc;

use hyper::client::RequestBuilder;
use hyper::header::{Host, AcceptEncoding, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

use aws_types::region::Region;

/// The MIME sublevel content type of an ECS HTTP request body
const AMZ_SUBLEVEL_CONTENT_TYPE: &'static str = "x-amz-json-1.1";

/// A trait defining default implementations for setting headers common to all ECS requests and
/// signing requests using Amazon's Authorization Version 4 Signing Algorithm
trait AWSRequest {
    /// The particular action associated with this request
    type Action;
    /// Gets the RequestBuilder object from the particular request type
    fn get_mut_request_builder(&mut self) -> Rc<RefCell<RequestBuilder>>;

    /// Gets the region that this request is being sent to
    fn get_region(&self) -> Region;

    /// Sets the Host header
    fn set_host_header(&mut self) {
        let region = self.get_region();
        let mut hostname = String::from("ecs.");
        hostname.push_str(&region.to_string());
        hostname.push_str(".amazonaws.com");

        let wrapped_req_builder = self.get_mut_request_builder().clone();
        let req_builder = wrapped_req_builder.borrow_mut();
        req_builder.header(
            Host {
                hostname: hostname,
                port: None,
            }
        );
    }

    /// Sets the Accept-Encoding header
    fn set_accept_encoding_header(&mut self) {
        let wrapped_req_builder = self.get_mut_request_builder().clone();
        let req_builder = wrapped_req_builder.borrow_mut();
        req_builder.header(AcceptEncoding(vec![]));
    }

    // TODO: set content-length header immediately after setting body

    /// Sets the X-Amz-Target header
    fn set_x_amz_target_header(&mut self) {
        unimplemented!()
    }
    
    /// Sets the X-Amz-Date header
    fn set_x_amz_date_header(&mut self) {
        unimplemented!()
    }

    /// Sets the Content-Type header
    fn set_content_type_header(&mut self) {
        let wrapped_req_builder = self.get_mut_request_builder().clone();
        let req_builder = wrapped_req_builder.borrow_mut();
        req_builder.header(
            ContentType(
                Mime(
                    TopLevel::Application,
                    SubLevel::Ext(String::from(AMZ_SUBLEVEL_CONTENT_TYPE)),
                    vec![],
                )
            )
        );
    }

    /// Sets the Authorization header
    /// note: may be replaced by a sign() or sign_and_send() function
    fn set_authorization_header(&mut self) {
        unimplemented!()
    }
}
