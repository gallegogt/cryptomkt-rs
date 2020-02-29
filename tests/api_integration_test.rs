use cryptomkt::CryptoMktClient;

#[test]
fn test_api_get_markets() {
    let api = CryptoMktClient::new("APK", "SK");
    let markets = api.get_markets();
    assert!(markets.len() > 1);
}
