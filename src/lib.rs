#[macro_use]
extern crate log;
extern crate reqwest;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod internal;
mod api;
pub use api::CryptoMktApi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
