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

pub use api::{CryptoMktApi, RequestMethod};
pub use market::Market;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashMap;
//     use internal::response::CryptoMktResponse;

//     type SymbolsResponse = CryptoMktResponse<Vec<String>>;

//     #[test]
//     fn test_api_call() {
//         let api_ck = CryptoMktApi::new("api_key", "secret_key");
//         let response = api_ck.call::<SymbolsResponse>(RequestMethod::Get(true), "market", HashMap::new());
//         match response {
//             Ok(rsp) => {
//                 println!("{:?}", rsp);
//                 assert!(rsp.data.len() != 0);
//             },
//             Err(e) => {
//                 println!("{:?}", e);
//                 assert!(false);
//             }
//         }
//     }
// }
