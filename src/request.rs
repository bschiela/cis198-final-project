//! A Signature Version 4 Signable AWS Request to ECS

use std::mem;

use hyper::client::RequestBuilder;
use hyper::header::{Host, AcceptEncoding, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

use region::Region;

/// The MIME sublevel content type of an ECS HTTP request body
const AMZ_SUBLEVEL_CONTENT_TYPE: &'static str = "x-amz-json-1.1";

pub struct ECSRequest<'a> {
    region: Region,
    req_builder: RequestBuilder<'a>,
}

impl<'a> ECSRequest<'a> {
    /// Creates a new ECSRequest destined for the specified Region
    pub fn new(region: Region, req_builder: RequestBuilder<'a>) -> ECSRequest {
        ECSRequest {
            region: region,
            req_builder: req_builder,
        }
    }

    /// Sets the Host header
    fn set_host_header(&mut self) {
        let mut hostname = String::from("ecs.");
        hostname.push_str(&self.region.to_string());
        hostname.push_str(".amazonaws.com");

        // TODO fix this
        //self.req_builder.header(
        //    Host {
        //        hostname: hostname,
        //        port: None,
        //    }
        //);
    }

    /// Sets the Accept-Encoding header
    fn set_accept_encoding_header(&mut self) {
        unimplemented!()
        // TODO fix this
        //self.req_builder.header(AcceptEncoding(vec![]));
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
        unimplemented!()
        // TODO fix this
        //self.req_builder.header(
        //    ContentType(
        //        Mime(
        //            TopLevel::Application,
        //            SubLevel::Ext(String::from(AMZ_SUBLEVEL_CONTENT_TYPE)),
        //            vec![],
        //        )
        //    )
        //);
    }

    /// Sets the Authorization header
    /// note: may be replaced by a sign() or sign_and_send() function
    fn set_authorization_header(&mut self) {
        unimplemented!()
    }
}
