//!
//! cryptomkt-rs provides a simple and powerful implementation for CryptoMarket API.
//!
//! Example where the last ticker is shown for each market available in the exchange market Criptomarket
//!
//! ```
//! extern crate cryptomkt;
//! use cryptomkt::{CryptoMktClient, OrderType};
//!
//! const API_KEY: &'static str = "<API_KEY>";
//! const API_SECRET: &'static str = "<API SECRET>";
//!
//! fn main() {
//!
//!     let client = CryptoMktClient::new(API_KEY, API_SECRET);
//!
//!     // Get the markets available in the exchange
//!     let markets = client.get_markets();
//!     for m in markets.iter() {
//!         println!("{}", m.get_name());
//!
//!         // GET current Ticker
//!         match m.get_current_ticker() {
//!             Ok(ticker) => {
//!                 println!("{:?}", ticker);
//!             }
//!             Err(e) => {
//!                 println!("{:?}", e);
//!             }
//!         }
//! }
//! ```

#[macro_use]
extern crate log;
extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod api;
mod client;
mod internal;
mod market;

pub use api::{CryptoMktApi, RequestMethod};
pub use client::CryptoMktClient;
pub use internal::models;
pub use internal::response;
pub use market::{Market, OrderType};
