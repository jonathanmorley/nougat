extern crate hyper;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate serde_xml_rs;
extern crate tempdir;
extern crate url;

pub mod client;
pub mod feed;
pub mod package;
