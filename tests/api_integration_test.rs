extern crate cryptomkt;

use cryptomkt::CryptoMktApi;

#[test]
fn test_api_info() {
    let api = CryptoMktApi::new("APK", "SK");

    assert_eq!(api.version(), "v1" );
    assert_eq!(api.domain(), "https://api.cryptomkt.com/" );
}