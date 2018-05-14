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
/// Cryptomarket API
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
    /// Peticion HTTP
    ///
    /// #Argumentos
    ///     method: Get(is_public) | Post
    ///     endpoint: Cadena de texto con el endpoint de la API (Ej: "orders/active" )
    ///     payload: Datos a enviar endpoint
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
