#[macro_use]
extern crate log;
extern crate reqwest;
#[macro_use]
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod internal;
mod api;
mod market;
mod client;

pub use internal::models;
pub use market::{Market, OrderType};
pub use client::CryptoMktClient;
