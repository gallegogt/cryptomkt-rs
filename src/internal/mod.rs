extern crate hyper;
extern crate reqwest;
extern crate ring;

use std::collections::HashMap;
use self::reqwest::Url;
use self::hyper::header::Headers;

pub mod api;
pub mod request;

#[cfg(test)]
mod tests {
    use super::*;
    use self::request::{HttpReq, RequestError};
    use self::api::Api;

    const API_KEY: &'static str = "FS24FJ7";
    const SECRET_KEY: &'static str = "SFT23GSD";

    ///
    /// Configura la instancia de API para los diferentes Test que no requieren de
    /// una respuesta
    ///
    fn setup_test<'a>() -> Api<'a> {
        Api::new("FS24FJ7", "SFT23GSD", None)
    }

    #[test]
    fn get_domain_url() {
        let api = setup_test();
        assert_eq!(api.domain(), "https://api.cryptomkt.com/");
    }

    #[test]
    fn get_api_version() {
        let api = setup_test();
        assert_eq!(api.api_version(), "v1");
    }

    #[test]
    fn build_url() {
        let api = setup_test();
        assert_eq!(
            api.build_url("market", &HashMap::new()).as_str(),
            "https://api.cryptomkt.com/v1/market"
        );
    }

    #[test]
    fn build_api_signature_format() {
        let api = setup_test();
        let resp = api.build_signature_format("orders/active", &HashMap::new(), true);
        assert_eq!(resp.ends_with("/v1/orders/active"), true);
    }

    #[test]
    fn build_api_signature_format_with_params_as_get_method() {
        let api = setup_test();
        let mut params = HashMap::new();
        params.insert("amount".to_string(), "0.3".to_string());
        params.insert("market".to_string(), "ethclp".to_string());
        params.insert("price".to_string(), "10000".to_string());
        params.insert("type".to_string(), "buy".to_string());
        let resp = api.build_signature_format("orders/create", &params, true);

        assert_eq!(resp.ends_with("/v1/orders/create"), true);
    }

    #[test]
    fn build_api_signature_with_params_as_post_method() {
        let api = setup_test();
        let mut params = HashMap::new();
        params.insert("amount".to_string(), "0.3".to_string());
        params.insert("market".to_string(), "ethclp".to_string());
        params.insert("price".to_string(), "10000".to_string());
        params.insert("type".to_string(), "buy".to_string());
        let resp = api.build_signature_format("orders/create", &params, false);
        assert_ne!(resp.find("/v1/orders/create0.3ethclp10000buy"), None);
    }

    #[test]
    fn sign_msg() {
        let api = setup_test();
        let resp = api.sign_msg("1525053829/v1/orders/active");
        assert_eq!(resp, "eb2dfb910f14efee5184000c228ba095cd40ff4db24a1dfd9c52977478b9739b4409b6bc85a2fd6cdfd5c2361092c4b1".to_string());
    }

    #[test]
    fn sign_msg_with_payload() {
        let api = setup_test();
        let resp = api.sign_msg("1525055728/v1/orders/create0.3ethclp10000buy");
        assert_eq!(resp, "08ec7ce100a196d36970a77f7eee46c2e319e03edb953b0c05e5e9605b5c0d95cc7759f1a074f817f54527f618e90a1e".to_string());
    }

    ///
    ///
    /// Pruebas de respuestas para los diferentes ENDPOINTs
    ///
    ///
    struct MockRequest {
        resp_for_get: String,
        resp_for_post: String,
    }

    impl MockRequest {
        ///
        /// Inicializador de la clase
        ///
        pub fn new<'m>(resp_for_get: &'m str, resp_for_post: &'m str) -> Self {
            MockRequest {
                resp_for_get: resp_for_get.to_string(),
                resp_for_post: resp_for_post.to_string(),
            }
        }
    }

    impl HttpReq for MockRequest {
        ///
        ///  Argumentos:
        ///     url: Url
        ///     headers: Headers
        ///
        fn get(&self, _url: Url, _headers: Headers) -> Result<String, RequestError> {
            Ok(self.resp_for_get.clone())
        }
        ///
        ///  Argumentos:
        ///     url: Url
        ///     headers: Headers
        ///     payload: Datos a enviar a la URL especificada
        ///
        fn post(
            &self,
            _url: Url,
            _headers: Headers,
            _payload: HashMap<String, String>,
        ) -> Result<String, RequestError> {
            Ok(self.resp_for_post.clone())
        }
    }

    struct TestEnpoint {
        expect_response: String,
        expect_url: String,
        params: HashMap<String, String>,
        endpoint: String,
        is_post: bool,
    }

    impl TestEnpoint {
        ///
        /// Inicializador
        ///
        pub fn new<'e>(
            endpoint: &'e str,
            expect_response: &'e str,
            expect_url: &'e str,
            params: Option<HashMap<String, String>>,
            is_post: Option<bool>,
        ) -> Self {
            TestEnpoint {
                expect_response: expect_response.to_string(),
                expect_url: expect_url.to_string(),
                params: params.unwrap_or(HashMap::new()),
                endpoint: endpoint.to_string(),
                is_post: is_post.unwrap_or(false),
            }
        }
    }

    ///
    /// Configura la instancia de API para los diferentes Test que requieren de
    /// una respuesta
    ///
    /// #Argumentos
    ///     resp_for_get: Cadena que representa la respuesta en caso de una petición GET
    ///     resp_for_post: Cadena que representa la respuesta en caso de una petición POST
    ///
    fn setup_test_with_response<'m>(resp_for_get: &'m str, resp_for_post: &'m str) -> Api<'m> {
        let mock_transport = MockRequest::new(&resp_for_get, &resp_for_post);
        Api::new(API_KEY, SECRET_KEY, Some(Box::new(mock_transport)))
    }

    ///
    /// Configura los diferentes endpoints con sus posibles datos, respuestas y urls
    ///
    fn setup_endpoints() -> Vec<Box<TestEnpoint>> {
        let mut endpoints = Vec::new();
        // --
        // Endpoint Market
        // --
        //
        endpoints.push(Box::new(TestEnpoint::new(
            "market",
            "{\"status\": \"success\",\"data\": [\"ETHARS\",\"ETHCLP\"]}",
            "https://api.cryptomkt.com/v1/market",
            None,
            None,
        )));
        // --
        // Endpoint Ticker
        // --
        // market=ETHARS
        let mut ticker_params = HashMap::new();
        ticker_params.insert("market".to_string(), "ETHARS".to_string());
        endpoints.push(Box::new(TestEnpoint::new(
            "ticker",
            "{\"status\":\"success\",\"data\":[{\"high\":\"6888\",\"volume\":\"13.03\",\"low\":\"6303\",\"ask\":\"6887\",\"timestamp\":\"2017-08-2915:44:17.267526\",\"bid\":\"6416\",\"last_price\":\"6610\",\"market\":\"ETHARS\"}]}",
            "https://api.cryptomkt.com/v1/ticker?market=ETHARS",
            Some(ticker_params),
            None,
        )));
        // --
        // Endpoint Ordenes
        // --
        // "https://api.cryptomkt.com/v1/book?market=ETHCLP&page=0&type=buy"
        let mut books_params = HashMap::new();
        books_params.insert("market".to_string(), "ETHCLP".to_string());
        books_params.insert("type".to_string(), "buy".to_string());
        books_params.insert("page".to_string(), "0".to_string());

        endpoints.push(Box::new(TestEnpoint::new(
            "book",
            "{\"status\":\"success\",\"pagination\":{\"previous\":0,\"limit\":20,\"page\":0,\"next\":\"null\"},\"data\":[{\"timestamp\":\"2017-08-31T12:31:58.782060\",\"price\":\"252610\",\"amount\":\"0.6729\"},{\"timestamp\":\"2017-08-31T10:14:58.466285\",\"price\":\"252200\",\"amount\":\"7.6226\"},{\"timestamp\":\"2017-08-30T18:15:54.757558\",\"price\":\"252000\",\"amount\":\"2.9761\"},{\"timestamp\":\"2017-08-31T14:02:32.377008\",\"price\":\"251900\",\"amount\":\"7.9396\"},{\"timestamp\":\"2017-08-30T15:29:12.945642\",\"price\":\"251540\",\"amount\":\"0.7314\"},{\"timestamp\":\"2017-08-31T13:46:34.666282\",\"price\":\"250100\",\"amount\":\"0.0399\"}]}",
            "https://api.cryptomkt.com/v1/book?market=ETHCLP&page=0&type=buy",
            Some(books_params),
            None,
        )));
        // --
        // Endpoint Trades
        // --
        // "https://api.cryptomkt.com/v1/trades?market=ETHCLP&start=2017-05-20&end=2017-05-30&page=2"
        let mut trades_params = HashMap::new();
        trades_params.insert("market".to_string(), "ETHCLP".to_string());
        trades_params.insert("start".to_string(), "2017-05-20".to_string());
        trades_params.insert("end".to_string(), "2017-05-30".to_string());
        trades_params.insert("page".to_string(), "2".to_string());

        endpoints.push(Box::new(TestEnpoint::new(
            "trades",
            "{\"status\":\"success\",\"pagination\":{\"previous\":1,\"limit\":20,\"page\":2,\"next\":\"null\"},\"data\":[{\"market_taker\":\"buy\",\"timestamp\":\"2017-05-29T22:14:00.419466\",\"price\":\"155000\",\"amount\":\"0.129\",\"market\":\"ETHCLP\"},{\"market_taker\":\"buy\",\"timestamp\":\"2017-05-29T22:13:52.168265\",\"price\":\"155000\",\"amount\":\"0.6451\",\"market\":\"ETHCLP\"},{\"market_taker\":\"buy\",\"timestamp\":\"2017-05-29T22:01:52.054549\",\"price\":\"155000\",\"amount\":\"2.7441\",\"market\":\"ETHCLP\"},{\"market_taker\":\"buy\",\"timestamp\":\"2017-05-29T22:01:51.700777\",\"price\":\"154000\",\"amount\":\"3\",\"market\":\"ETHCLP\"},{\"market_taker\":\"buy\",\"timestamp\":\"2017-05-29T22:01:51.342244\",\"price\":\"151990\",\"amount\":\"0.0335\",\"market\":\"ETHCLP\"}]}",
            "https://api.cryptomkt.com/v1/trades?market=ETHCLP&start=2017-05-20&end=2017-05-30&page=2",
            Some(trades_params),
            None,
        )));
        // --
        // Endpoint Ordenes activas
        // --
        // "https://api.cryptomkt.com/v1/orders/active?market=ETHCLP&page=0"
        let mut oa_params = HashMap::new();
        oa_params.insert("market".to_string(), "ETHCLP".to_string());
        oa_params.insert("page".to_string(), "0".to_string());

        endpoints.push(Box::new(TestEnpoint::new(
            "orders/active",
            "{\"status\":\"success\",\"pagination\":{\"previous\":\"null\",\"limit\":20,\"page\":0,\"next\":\"null\"},\"data\":[{\"status\":\"active\",\"created_at\":\"2017-09-01T14:01:56.887272\",\"amount\":{\"original\":\"1.4044\",\"remaining\":\"1.4044\"},\"execution_price\":null,\"price\":\"7120\",\"type\":\"buy\",\"id\":\"M103966\",\"market\":\"ETHCLP\",\"updated_at\":\"2017-09-01T14:01:56.887272\"},{\"status\":\"active\",\"created_at\":\"2017-09-01T14:02:36.386967\",\"amount\":{\"original\":\"1.25\",\"remaining\":\"1.25\"},\"execution_price\":null,\"price\":\"8000\",\"type\":\"buy\",\"id\":\"M103967\",\"market\":\"ETHCLP\",\"updated_at\":\"2017-09-01T14:02:36.386967\"}]}",
            "https://api.cryptomkt.com/v1/orders/active?market=ETHCLP&page=0",
            Some(oa_params),
            None,
        )));
        // --
        // Endpoint Ordenes Ejecutadas
        // --
        // "https://api.cryptomkt.com/v1/orders/executed?market=ETHCLP&page=1"
        let mut oe_params = HashMap::new();
        oe_params.insert("market".to_string(), "ETHCLP".to_string());
        oe_params.insert("page".to_string(), "1".to_string());

        endpoints.push(Box::new(TestEnpoint::new(
            "orders/executed",
            "{\"status\":\"success\",\"pagination\":{\"previous\":\"null\",\"limit\":20,\"page\":0,\"next\":\"null\"},\"data\":[{\"status\":\"executed\",\"created_at\":\"2017-08-31T21:37:42.282102\",\"amount\":{\"executed\":\"0.6\",\"original\":\"3.75\"},\"execution_price\":\"8000\",\"executed_at\":\"2017-08-31T22:01:19.481403\",\"price\":\"8000\",\"type\":\"buy\",\"id\":\"M103959\",\"market\":\"ETHCLP\"},{\"status\":\"executed\",\"created_at\":\"2017-08-31T21:37:42.282102\",\"amount\":{\"executed\":\"0.5\",\"original\":\"3.75\"},\"execution_price\":\"8000\",\"executed_at\":\"2017-08-31T22:00:13.805482\",\"price\":\"8000\",\"type\":\"buy\",\"id\":\"M103959\",\"market\":\"ETHCLP\"},{\"status\":\"executed\",\"created_at\":\"2016-11-26T23:27:54.502024\",\"amount\":{\"executed\":\"1.5772\",\"original\":\"1.5772\"},\"execution_price\":\"6340\",\"executed_at\":\"2017-01-02T22:56:03.897534\",\"price\":\"6340\",\"type\":\"buy\",\"id\":\"M103260\",\"market\":\"ETHCLP\"}]}",
            "https://api.cryptomkt.com/v1/orders/executed?market=ETHCLP&page=1",
            Some(oe_params),
            None,
        )));
        // --
        // Endpoint Ordenes Ejecutadas
        // --
        // "https://api.cryptomkt.com/v1/orders/create"
        // {"amount": "0.3","market": "ethclp","price": "10000","type": "buy"}
        let mut oc_params = HashMap::new();
        oc_params.insert("market".to_string(), "ethclp".to_string());
        oc_params.insert("amount".to_string(), "0.3".to_string());
        oc_params.insert("price".to_string(), "10000".to_string());
        oc_params.insert("type".to_string(), "buy".to_string());

        endpoints.push(Box::new(TestEnpoint::new(
            "orders/create",
            "{\"status\":\"success\",\"data\":{\"status\":\"executed\",\"created_at\":\"2017-09-01T19:35:26.641136\",\"amount\":{\"executed\":\"0.3\",\"original\":\"0.3\"},\"avg_execution_price\":\"30000\",\"price\":\"10000\",\"type\":\"buy\",\"id\":\"M103975\",\"market\":\"ETHCLP\",\"updated_at\":\"2017-09-01T19:35:26.688106\"}}",
            "https://api.cryptomkt.com/v1/orders/create",
            Some(oc_params),
            Some(true),
        )));

        // --
        // Endpoint Estados de Orden
        // --
        // "https://api.cryptomkt.com/v1/orders/status?id=M103975"
        //
        let mut os_params = HashMap::new();
        os_params.insert("id".to_string(), "M103975".to_string());

        endpoints.push(Box::new(TestEnpoint::new(
            "orders/status",
            "{\"status\":\"success\",\"data\":{\"status\":\"active\",\"created_at\":\"2017-09-01T14:01:56.887272\",\"amount\":{\"executed\":\"0\",\"original\":\"1.4044\"},\"avg_execution_price\":\"0\",\"price\":\"7120\",\"type\":\"buy\",\"id\":\"M103966\",\"market\":\"ETHCLP\",\"updated_at\":\"2017-09-01T14:01:56.887272\"}}",
            "https://api.cryptomkt.com/v1/orders/status?id=M103975",
            Some(os_params),
            None,
        )));

        // --
        // Endpoint Cancelar Orden
        // --
        // "https://api.cryptomkt.com/v1/orders/cancel"
        //
        let mut os_params = HashMap::new();
        os_params.insert("id".to_string(), "M103975".to_string());

        endpoints.push(Box::new(TestEnpoint::new(
            "orders/cancel",
            "{\"status\":\"success\",\"data\":{\"status\":\"cancelled\",\"created_at\":\"2017-09-01T14:02:36.386967\",\"amount\":{\"executed\":\"0\",\"original\":\"1.25\"},\"avg_execution_price\":\"0\",\"price\":\"8000\",\"type\":\"buy\",\"id\":\"M103967\",\"market\":\"ETHCLP\",\"updated_at\":\"2017-09-01T14:02:36.386967\"}}",
            "https://api.cryptomkt.com/v1/orders/cancel",
            Some(os_params),
            Some(true),
        )));

        // --
        // Endpoint: Crear una orden de compra o venta en Instant Exchange
        // --
        // "https://api.cryptomkt.com/v1/orders/instant/get?market=ETHCLP&type=sell&amount=159"
        //
        let mut os_params = HashMap::new();
        os_params.insert("market".to_string(), "ETHCLP".to_string());
        os_params.insert("type".to_string(), "sell".to_string());
        os_params.insert("amount".to_string(), "159".to_string());

        endpoints.push(Box::new(TestEnpoint::new(
            "orders/instant/get",
            "{\"status\":\"success\",\"data\":{\"obtained\":\"18047138.226\",\"required\":\"159\"}}",
            "https://api.cryptomkt.com/v1/orders/instant/get?market=ETHCLP&type=sell&amount=159",
            Some(os_params),
            None,
        )));

        // --
        // Endpoint: Obtener cantidad estimada para una orden instantánea en Instant Exchange
        // --
        // "https://api.cryptomkt.com/v1/orders/instant/create"
        // {"market": "ETHCLP", "type": "buy", "amount": "10"}
        let mut os_params = HashMap::new();
        os_params.insert("market".to_string(), "ETHCLP".to_string());
        os_params.insert("type".to_string(), "buy".to_string());
        os_params.insert("amount".to_string(), "10".to_string());

        endpoints.push(Box::new(TestEnpoint::new(
            "orders/instant/create",
            "{\"status\":\"success\",\"data\":''}",
            "https://api.cryptomkt.com/v1/orders/instant/create",
            Some(os_params),
            Some(true),
        )));

        // --
        // Endpoint: Obtener balances
        // --
        // "https://api.cryptomkt.com/v1/balance"
        //
        endpoints.push(Box::new(TestEnpoint::new(
            "balance",
            "{\"status\":\"success\",\"data\":[{\"available\":\"120347\",\"wallet\":\"CLP\",\"balance\":\"120347\"},{\"available\":\"10.3399\",\"wallet\":\"ETH\",\"balance\":\"11.3399\"}]}",
            "https://api.cryptomkt.com/v1/balance",
            Some(HashMap::new()),
            None,
        )));

        endpoints
    }

    ///
    /// Retorna listado de mercados disponibles en CryptoMarket.
    ///
    #[test]
    fn test_edge_endpoints() {
        let tests_data = setup_endpoints();
        for it in tests_data.iter() {
            let mut get_expect = "";
            let mut post_expect = "";
            let expect;

            if it.is_post {
                post_expect = it.expect_response.as_str();
                expect = post_expect;
            } else {
                get_expect = it.expect_response.as_str();
                expect = get_expect;
            }

            let api = setup_test_with_response(get_expect, post_expect);
            let resp;
            let url;

            if it.is_post {
                resp = api.post_edge(it.endpoint.as_str(), it.params.clone());
                url = api.build_url(&it.endpoint.as_str(), &HashMap::new());
            } else {
                resp = api.get_edge(it.endpoint.as_str(), it.params.clone(), true);
                url = api.build_url(&it.endpoint.as_str(), &it.params.clone());
            }

            assert_eq!(
                url.as_str().len(), it.expect_url.len(),
                "las urls no coinciden para el endpoint: {} Url: {}",
                it.endpoint,
                url.as_str()
            );
            assert_eq!(resp, expect, "Error al probar el endpoint: {}", it.endpoint);
        }
    }
}
