extern crate serde;
extern crate serde_json;
#[macro_use] extern crate hyper;
extern crate time;

// required to use serde on stable rust
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
