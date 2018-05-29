use std::collections::HashMap;
use internal::api::Api;
use internal::errors::CryptoMktErrorType;
use internal::request::CryptoMktRequest;
use serde::de::DeserializeOwned;

///
/// Métodos de encuestas soportados por el API
///
pub enum RequestMethod {
    /// HTTP Request POST
    Post,
    /// HTTP Request GET, El parámetro define si la petición es para un
    /// endpoint publico o no
    Get(bool),
}

///
/// # Cryptomkt API
///
/// Permite el acceso al API de cryptomarket si conoces bien los diferentes
/// endpoints. Para más información sobre los endpoints ir a [Developers Cryptomkt](http://developers.cryptomkt.com/)
///
/// ## Ejemplo
///
/// ```
/// extern crate cryptomkt;
/// use cryptomkt::CryptoMktApi;
///
/// let api = CryptoMktApi::new("<API Key>", "<Secret Key>");
/// println!("Api version: {}", api.version());
/// println!("Api domain: {}", api.domain());
/// ```
///
#[derive(Debug, Clone)]
pub struct CryptoMktApi {
    i_api: Box<Api<CryptoMktRequest>>,
}

impl CryptoMktApi {
    ///
    /// Crea la nueva instancia del API
    ///
    pub fn new<'a>(api_key: &'a str, secret_key: &'a str) -> Self {
        CryptoMktApi {
            i_api: Box::new(Api::<CryptoMktRequest>::new(
                api_key,
                secret_key,
                Box::new(CryptoMktRequest::new()),
            )),
        }
    }

    ///
    /// Devuelve el dominio
    ///
    pub fn domain(&self) -> String {
        self.i_api.domain()
    }

    ///
    /// Devuelve la version del API
    ///
    pub fn version(&self) -> String {
        self.i_api.api_version()
    }

    ///
    /// Realiza una petición HTTP al servidor
    ///
    /// ```
    /// extern crate cryptomkt;
    /// use cryptomkt::{CryptoMktApi, Market, RequestMethod};
    /// use cryptomkt::response::MarketResponse;
    /// use std::collections::HashMap;
    ///
    /// let api = CryptoMktApi::new("<API Key>", "<Secret Key>");
    /// let resp = api.call::<MarketResponse>(RequestMethod::Get(true), "market", HashMap::new());
    /// match resp {
    ///     Ok(value) => {
    ///         let mut market_list = Vec::new();
    ///         for it in value.data {
    ///             market_list.push(Market::new(api.clone(), it.clone().as_str()));
    ///         }
    ///         println!("{:?}", market_list[0].get_name());
    ///     }
    ///     Err(e) => {
    ///         println!("{:?}", e);
    ///     }
    /// }
    /// ```
    ///
    /// #Argumentos
    ///     `method`: Enum que representa el método de encuesta al servidor: Get(is_public) | Post
    ///     `endpoint`: Cadena de texto con el endpoint de la API (Ej: "orders/active" ), no debe comenzar por "/"
    ///     `payload`: Hashmap que representan los datos de encuesta para el servidor
    ///
    pub fn call<'a, T>(
        &self,
        method: RequestMethod,
        endpoint: &'a str,
        payload: HashMap<String, String>,
    ) -> Result<T, CryptoMktErrorType>
    where
        T: DeserializeOwned,
    {
        match method {
            RequestMethod::Get(is_public) => self.i_api.get_edge(endpoint, payload, is_public),
            RequestMethod::Post => self.i_api.post_edge(endpoint, payload),
        }
    }
}
