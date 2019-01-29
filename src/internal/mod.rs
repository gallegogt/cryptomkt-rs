//!
//! Módulo interno `internal`
//!
//! En este módulo se encuentran las implementaciones de más bajo nivel para el acceso
//! al API junto con sus respectivas pruebas
//!
extern crate hyper;
extern crate reqwest;
extern crate ring;
extern crate serde;

pub mod api;
pub mod errors;
pub mod models;
pub mod request;
pub mod response;

#[cfg(test)]
mod tests {
    use internal::api::Api;
    use internal::errors::CryptoMktResult;
    use internal::request::HttpReq;
    use serde_json::json;

    use reqwest::header::HeaderMap;
    use reqwest::Url;
    use std::collections::HashMap;

    use response::{
        BalanceResponse, BookResponse, EmptyResponse, MarketResponse, OrderResponse,
        OrdersInstantResponse, SimpleOrderResponse, TickerResponse, TradeResponse,
    };
    const API_KEY: &'static str = "FS24FJ7";
    const SECRET_KEY: &'static str = "SFT23GSD";

    ///
    ///
    /// Pruebas de respuestas para los diferentes ENDPOINTs
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
        ///     headers: HeaderMap
        ///
        fn get(&self, _url: Url, _headers: HeaderMap) -> CryptoMktResult<String> {
            Ok(self.resp_for_get.clone())
        }
        ///
        ///  Argumentos:
        ///     url: Url
        ///     headers: HeaderMap
        ///     payload: Datos a enviar a la URL especificada
        ///
        fn post(
            &self,
            _url: Url,
            _headers: HeaderMap,
            _payload: HashMap<String, String>,
        ) -> CryptoMktResult<String> {
            Ok(self.resp_for_post.clone())
        }
    }
    ///
    /// Configura la instancia de API para los diferentes Test que no requieren de
    /// una respuesta
    ///
    fn setup_test() -> Api<MockRequest> {
        Api::<MockRequest>::new("FS24FJ7", "SFT23GSD", Box::new(MockRequest::new("", "")))
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

    #[test]
    fn test_response_for_market_list() {
        let mock_transport = MockRequest::new(
            "{\"status\": \"success\",\"data\": [\"ETHARS\",\"ETHCLP\"]}",
            "",
        );
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({"status": "success","data": ["ETHARS","ETHCLP"]});
        let resp = api
            .get_edge::<MarketResponse>("market", HashMap::new(), true)
            .unwrap();

        assert_eq!(
            resp.status, expected["status"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );

        for it in 1..resp.data.len() {
            assert_eq!(
                resp.data[it], expected["data"][it],
                "Los mercados no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
        }
    }

    #[test]
    fn test_response_for_ticker() {
        let mock_transport = MockRequest::new(
            "{\"status\":\"success\",\"data\":[{\"high\":\"6888\",\"volume\":\"13.03\",\"low\":\"6303\",\"ask\":\"6887\",\"timestamp\":\"2017-08-2915:44:17.267526\",\"bid\":\"6416\",\"last_price\":\"6610\",\"market\":\"ETHARS\"}]}",
            "",
        );
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({
            "status":"success","data":[{"high":"6888","volume":"13.03","low":"6303","ask":"6887","timestamp":"2017-08-2915:44:17.267526","bid":"6416","last_price":"6610","market":"ETHARS"}]
        });
        let mut ticker_params = HashMap::new();
        ticker_params.insert("market".to_string(), "ETHARS".to_string());
        let resp = api
            .get_edge::<TickerResponse>("ticker", ticker_params, true)
            .unwrap();

        assert_eq!(
            resp.status, expected["status"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );

        for it in 1..resp.data.len() {
            assert_eq!(
                resp.data[it].high, expected["data"][it]["high"],
                "Los mercados no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
        }
    }

    #[test]
    fn test_response_for_book() {
        let mock_transport = MockRequest::new(
            "{\"status\":\"success\",\"pagination\":{\"previous\":0,\"limit\":20,\"page\":0,\"next\":\"null\"},\"data\":[{\"timestamp\":\"2017-08-31T12:31:58.782060\",\"price\":\"252610\",\"amount\":\"0.6729\"},{\"timestamp\":\"2017-08-31T10:14:58.466285\",\"price\":\"252200\",\"amount\":\"7.6226\"},{\"timestamp\":\"2017-08-30T18:15:54.757558\",\"price\":\"252000\",\"amount\":\"2.9761\"},{\"timestamp\":\"2017-08-31T14:02:32.377008\",\"price\":\"251900\",\"amount\":\"7.9396\"},{\"timestamp\":\"2017-08-30T15:29:12.945642\",\"price\":\"251540\",\"amount\":\"0.7314\"},{\"timestamp\":\"2017-08-31T13:46:34.666282\",\"price\":\"250100\",\"amount\":\"0.0399\"}]}",
            "",
        );
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({
            "status":"success",
            "pagination":{"previous":0,"limit":20,"page":0,"next":"null"},
            "data":[
                {"timestamp":"2017-08-31T12:31:58.782060","price":"252610","amount":"0.6729"},{"timestamp":"2017-08-31T10:14:58.466285","price":"252200","amount":"7.6226"},{"timestamp":"2017-08-30T18:15:54.757558","price":"252000","amount":"2.9761"},{"timestamp":"2017-08-31T14:02:32.377008","price":"251900","amount":"7.9396"},{"timestamp":"2017-08-30T15:29:12.945642","price":"251540","amount":"0.7314"},{"timestamp":"2017-08-31T13:46:34.666282","price":"250100","amount":"0.0399"}
            ]
        });
        let mut params = HashMap::new();
        params.insert("market".to_string(), "ETHCLP".to_string());
        params.insert("type".to_string(), "buy".to_string());
        params.insert("page".to_string(), "0".to_string());

        let resp = api.get_edge::<BookResponse>("book", params, true).unwrap();

        assert_eq!(
            resp.status, expected["status"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );

        for it in 1..resp.data.len() {
            assert_eq!(
                resp.data[it].timestamp, expected["data"][it]["timestamp"],
                "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
        }
        let pagination = resp.pagination;
        assert_eq!(
            pagination.limit, 20,
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            pagination, 0
        );
    }

    #[test]
    fn test_response_for_trades() {
        let mock_transport = MockRequest::new(
            "{\"status\":\"success\",\"pagination\":{\"previous\":1,\"limit\":20,\"page\":2,\"next\":\"null\"},\"data\":[{\"market_taker\":\"buy\",\"timestamp\":\"2017-05-29T22:14:00.419466\",\"price\":\"155000\",\"amount\":\"0.129\",\"market\":\"ETHCLP\"},{\"market_taker\":\"buy\",\"timestamp\":\"2017-05-29T22:13:52.168265\",\"price\":\"155000\",\"amount\":\"0.6451\",\"market\":\"ETHCLP\"},{\"market_taker\":\"buy\",\"timestamp\":\"2017-05-29T22:01:52.054549\",\"price\":\"155000\",\"amount\":\"2.7441\",\"market\":\"ETHCLP\"},{\"market_taker\":\"buy\",\"timestamp\":\"2017-05-29T22:01:51.700777\",\"price\":\"154000\",\"amount\":\"3\",\"market\":\"ETHCLP\"},{\"market_taker\":\"buy\",\"timestamp\":\"2017-05-29T22:01:51.342244\",\"price\":\"151990\",\"amount\":\"0.0335\",\"market\":\"ETHCLP\"}]}",
            "",
        );
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({
            "status":"success",
            "pagination":{"previous":1,"limit":20,"page":2,"next":"null"},
            "data":[
                {"market_taker":"buy","timestamp":"2017-05-29T22:14:00.419466","price":"155000","amount":"0.129","market":"ETHCLP"},{"market_taker":"buy","timestamp":"2017-05-29T22:13:52.168265","price":"155000","amount":"0.6451","market":"ETHCLP"},{"market_taker":"buy","timestamp":"2017-05-29T22:01:52.054549","price":"155000","amount":"2.7441","market":"ETHCLP"},{"market_taker":"buy","timestamp":"2017-05-29T22:01:51.700777","price":"154000","amount":"3","market":"ETHCLP"},{"market_taker":"buy","timestamp":"2017-05-29T22:01:51.342244","price":"151990","amount":"0.0335","market":"ETHCLP"}
            ]
        });
        let mut params = HashMap::new();
        params.insert("market".to_string(), "ETHCLP".to_string());
        params.insert("start".to_string(), "2017-05-20".to_string());
        params.insert("end".to_string(), "2017-05-30".to_string());
        params.insert("page".to_string(), "2".to_string());

        let resp = api
            .get_edge::<TradeResponse>("trades", params, true)
            .unwrap();

        assert_eq!(
            resp.status, expected["status"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );

        for it in 1..resp.data.len() {
            assert_eq!(
                resp.data[it].market_taker, expected["data"][it]["market_taker"],
                "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
            assert_eq!(
                resp.data[it].timestamp, expected["data"][it]["timestamp"],
                "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
        }
        let pagination = resp.pagination;
        assert_eq!(
            pagination.limit, 20,
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            pagination, 0
        );
        assert_eq!(
            pagination.page, 2,
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            pagination, 0
        );
    }

    #[test]
    fn test_response_for_orders_active() {
        let mock_transport = MockRequest::new(
            "{\"status\":\"success\",\"pagination\":{\"previous\":\"null\",\"limit\":20,\"page\":0,\"next\":\"null\"},\"data\":[{\"status\":\"active\",\"created_at\":\"2017-09-01T14:01:56.887272\",\"amount\":{\"original\":\"1.4044\",\"remaining\":\"1.4044\"},\"execution_price\":null,\"price\":\"7120\",\"type\":\"buy\",\"id\":\"M103966\",\"market\":\"ETHCLP\",\"updated_at\":\"2017-09-01T14:01:56.887272\"},{\"status\":\"active\",\"created_at\":\"2017-09-01T14:02:36.386967\",\"amount\":{\"original\":\"1.25\",\"remaining\":\"1.25\"},\"execution_price\":null,\"price\":\"8000\",\"type\":\"buy\",\"id\":\"M103967\",\"market\":\"ETHCLP\",\"updated_at\":\"2017-09-01T14:02:36.386967\"}]}",
            "",
        );
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({
            "status":"success",
            "pagination":{"previous":"null","limit":20,"page":0,"next":"null"},
            "data":[
                {"status":"active","created_at":"2017-09-01T14:01:56.887272", "amount":{"original":"1.4044","remaining":"1.4044"}, "execution_price":null,"price":"7120","type":"buy","id":"M103966","market":"ETHCLP","updated_at":"2017-09-01T14:01:56.887272"},
                {"status":"active","created_at":"2017-09-01T14:02:36.386967","amount":{"original":"1.25","remaining":"1.25"},"execution_price":null,"price":"8000","type":"buy","id":"M103967","market":"ETHCLP","updated_at":"2017-09-01T14:02:36.386967"}
            ]
        });
        let mut params = HashMap::new();
        params.insert("market".to_string(), "ETHCLP".to_string());
        params.insert("page".to_string(), "0".to_string());

        let resp = api
            .get_edge::<OrderResponse>("orders/active", params, true)
            .unwrap();

        assert_eq!(
            resp.status, expected["status"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );

        for it in 1..resp.data.len() {
            assert_eq!(
                resp.data[it].created_at, expected["data"][it]["created_at"],
                "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
        }
        let pagination = resp.pagination;
        assert_eq!(
            pagination.limit, 20,
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            pagination, 0
        );
    }
    #[test]
    fn test_response_for_orders_executed() {
        let mock_transport = MockRequest::new(
            "{\"status\":\"success\",\"pagination\":{\"previous\":\"null\",\"limit\":20,\"page\":0,\"next\":\"null\"},\"data\":[{\"status\":\"executed\",\"created_at\":\"2017-08-31T21:37:42.282102\",\"amount\":{\"executed\":\"0.6\",\"original\":\"3.75\"},\"execution_price\":\"8000\",\"executed_at\":\"2017-08-31T22:01:19.481403\",\"price\":\"8000\",\"type\":\"buy\",\"id\":\"M103959\",\"market\":\"ETHCLP\"},{\"status\":\"executed\",\"created_at\":\"2017-08-31T21:37:42.282102\",\"amount\":{\"executed\":\"0.5\",\"original\":\"3.75\"},\"execution_price\":\"8000\",\"executed_at\":\"2017-08-31T22:00:13.805482\",\"price\":\"8000\",\"type\":\"buy\",\"id\":\"M103959\",\"market\":\"ETHCLP\"},{\"status\":\"executed\",\"created_at\":\"2016-11-26T23:27:54.502024\",\"amount\":{\"executed\":\"1.5772\",\"original\":\"1.5772\"},\"execution_price\":\"6340\",\"executed_at\":\"2017-01-02T22:56:03.897534\",\"price\":\"6340\",\"type\":\"buy\",\"id\":\"M103260\",\"market\":\"ETHCLP\"}]}",
            "",
        );
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({
            "status":"success",
            "pagination":{"previous":"null","limit":20,"page":0,"next":"null"},
            "data":[
                {"status":"executed","created_at":"2017-08-31T21:37:42.282102","amount":{"executed":"0.6","original":"3.75"},"execution_price":"8000","executed_at":"2017-08-31T22:01:19.481403","price":"8000","type":"buy","id":"M103959","market":"ETHCLP"},
                {"status":"executed","created_at":"2017-08-31T21:37:42.282102","amount":{"executed":"0.5","original":"3.75"},"execution_price":"8000","executed_at":"2017-08-31T22:00:13.805482","price":"8000","type":"buy","id":"M103959","market":"ETHCLP"},
                {"status":"executed","created_at":"2016-11-26T23:27:54.502024","amount":{"executed":"1.5772","original":"1.5772"},"execution_price":"6340","executed_at":"2017-01-02T22:56:03.897534","price":"6340","type":"buy","id":"M103260","market":"ETHCLP"}
                ]
        });
        let mut params = HashMap::new();
        params.insert("market".to_string(), "ETHCLP".to_string());
        params.insert("page".to_string(), "1".to_string());

        let resp = api
            .get_edge::<OrderResponse>("orders/executed", params, true)
            .unwrap();

        assert_eq!(
            resp.status, expected["status"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );

        for it in 1..resp.data.len() {
            assert_eq!(
                resp.data[it].created_at, expected["data"][it]["created_at"],
                "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
            assert_eq!(
                resp.data[it].status, expected["data"][it]["status"],
                "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
            assert_eq!(
                resp.data[it].execution_price, expected["data"][it]["execution_price"],
                "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
        }
        let pagination = resp.pagination;
        assert_eq!(
            pagination.limit, 20,
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            pagination, 0
        );
    }

    #[test]
    fn test_response_for_orders_create() {
        let mock_transport = MockRequest::new(
            "",
            "{\"status\":\"success\",\"data\":{\"status\":\"executed\",\"created_at\":\"2017-09-01T19:35:26.641136\",\"amount\":{\"executed\":\"0.3\",\"original\":\"0.3\"},\"avg_execution_price\":\"30000\",\"price\":\"10000\",\"type\":\"buy\",\"id\":\"M103975\",\"market\":\"ETHCLP\",\"updated_at\":\"2017-09-01T19:35:26.688106\"}}"
        );
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({
            "status":"success",
            "data":{
                "status":"executed","created_at":"2017-09-01T19:35:26.641136",
                "amount":{"executed":"0.3","original":"0.3"},
                "avg_execution_price":"30000","price":"10000",
                "type":"buy","id":"M103975","market":"ETHCLP",
                "updated_at":"2017-09-01T19:35:26.688106"
            }
        });
        let mut params = HashMap::new();
        params.insert("market".to_string(), "ethclp".to_string());
        params.insert("amount".to_string(), "0.3".to_string());
        params.insert("price".to_string(), "10000".to_string());
        params.insert("type".to_string(), "buy".to_string());

        let resp = api
            .post_edge::<SimpleOrderResponse>("orders/create", params)
            .unwrap();

        assert_eq!(
            resp.status, expected["status"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );

        assert_eq!(
            resp.data.created_at, expected["data"]["created_at"],
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            resp, expected
        );
        assert_eq!(
            resp.data.status, expected["data"]["status"],
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            resp, expected
        );
        assert_eq!(
            resp.data.execution_price, expected["data"]["execution_price"],
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            resp, expected
        );
    }

    #[test]
    fn test_response_for_orders_status() {
        let mock_transport = MockRequest::new(
            "{\"status\":\"success\",\"data\":{\"status\":\"active\",\"created_at\":\"2017-09-01T14:01:56.887272\",\"amount\":{\"executed\":\"0\",\"original\":\"1.4044\"},\"avg_execution_price\":\"0\",\"price\":\"7120\",\"type\":\"buy\",\"id\":\"M103966\",\"market\":\"ETHCLP\",\"updated_at\":\"2017-09-01T14:01:56.887272\"}}",
            ""
        );
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({
            "status":"success",
            "data":{"status":"active","created_at":"2017-09-01T14:01:56.887272","amount":{"executed":"0","original":"1.4044"},"avg_execution_price":"0","price":"7120","type":"buy","id":"M103966","market":"ETHCLP","updated_at":"2017-09-01T14:01:56.887272"}
        });
        let mut params = HashMap::new();
        params.insert("id".to_string(), "M103975".to_string());

        let resp = api
            .get_edge::<SimpleOrderResponse>("orders/status", params, true)
            .unwrap();

        assert_eq!(
            resp.status, expected["status"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );
        assert_eq!(
            resp.data.created_at, expected["data"]["created_at"],
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            resp, expected
        );
        assert_eq!(
            resp.data.status, expected["data"]["status"],
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            resp, expected
        );
        assert_eq!(
            resp.data.execution_price, expected["data"]["execution_price"],
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            resp, expected
        );
    }

    #[test]
    fn test_response_for_orders_cancel() {
        let mock_transport = MockRequest::new(
            "",
            "{\"status\":\"success\",\"data\":{\"status\":\"cancelled\",\"created_at\":\"2017-09-01T14:02:36.386967\",\"amount\":{\"executed\":\"0\",\"original\":\"1.25\"},\"avg_execution_price\":\"0\",\"price\":\"8000\",\"type\":\"buy\",\"id\":\"M103967\",\"market\":\"ETHCLP\",\"updated_at\":\"2017-09-01T14:02:36.386967\"}}"
        );
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({
           "status":"success",
           "data":{
               "status":"cancelled","created_at":"2017-09-01T14:02:36.386967","amount":{"executed":"0","original":"1.25"},"avg_execution_price":"0","price":"8000","type":"buy","id":"M103967","market":"ETHCLP","updated_at":"2017-09-01T14:02:36.386967"
           }
        });

        let mut params = HashMap::new();
        params.insert("id".to_string(), "M103975".to_string());

        let resp = api
            .post_edge::<SimpleOrderResponse>("orders/cancel", params)
            .unwrap();

        assert_eq!(
            resp.status, expected["status"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );
        assert_eq!(
            resp.data.created_at, expected["data"]["created_at"],
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            resp, expected
        );
        assert_eq!(
            resp.data.status, expected["data"]["status"],
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            resp, expected
        );
        assert_eq!(
            resp.data.execution_price, expected["data"]["execution_price"],
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            resp, expected
        );
    }

    #[test]
    fn test_response_for_orders_instant_get() {
        let mock_transport = MockRequest::new(
            "{\"status\":\"success\",\"data\":{\"obtained\":\"18047138.226\",\"required\":\"159\"}}",
            ""
        );
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({
           "status":"success","data":{"obtained":"18047138.226","required":"159"}
        });

        let mut params = HashMap::new();
        params.insert("market".to_string(), "ETHCLP".to_string());
        params.insert("type".to_string(), "sell".to_string());
        params.insert("amount".to_string(), "159".to_string());

        let resp = api
            .get_edge::<OrdersInstantResponse>("orders/instant/get", params, false)
            .unwrap();

        assert_eq!(
            resp.status, expected["status"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );
        assert_eq!(
            resp.data.obtained, expected["data"]["obtained"],
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            resp, expected
        );
        assert_eq!(
            resp.data.required, expected["data"]["required"],
            "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
            resp, expected
        );
    }

    #[test]
    fn test_response_for_orders_instant_create() {
        let mock_transport = MockRequest::new("", "{\"status\":\"success\",\"data\":\"\"}");
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({
           "status": "success",
           "data": ""
        });

        let mut params = HashMap::new();
        params.insert("market".to_string(), "ETHCLP".to_string());
        params.insert("type".to_string(), "buy".to_string());
        params.insert("amount".to_string(), "10".to_string());

        let resp = api
            .post_edge::<EmptyResponse>("orders/instant/create", params)
            .unwrap();

        assert_eq!(
            resp.status, expected["status"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );
    }

    #[test]
    fn test_response_for_balance() {
        let mock_transport = MockRequest::new(
            "{\"status\":\"success\",\"data\":[{\"available\":\"120347\",\"wallet\":\"CLP\",\"balance\":\"120347\"},{\"available\":\"10.3399\",\"wallet\":\"ETH\",\"balance\":\"11.3399\"}]}",
            ""
        );
        let api = Api::<MockRequest>::new(API_KEY, SECRET_KEY, Box::new(mock_transport));

        let expected = json!({
           "status":"success",
           "data":[
               {"available":"120347","wallet":"CLP","balance":"120347"},{"available":"10.3399","wallet":"ETH","balance":"11.3399"}
            ]
        });

        let params = HashMap::new();
        let resp = api
            .get_edge::<BalanceResponse>("balance", params, false)
            .unwrap();

        for it in 1..resp.data.len() {
            assert_eq!(
                resp.data[it].available, expected["data"][it]["available"],
                "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
            assert_eq!(
                resp.data[it].wallet, expected["data"][it]["wallet"],
                "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
            assert_eq!(
                resp.data[it].balance, expected["data"][it]["balance"],
                "Los datos no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
        }
    }
}
