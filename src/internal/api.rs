extern crate hyper;
extern crate reqwest;
extern crate ring;
// extern crate serde_json;

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::fmt::Write;

use self::ring::{digest, hmac};
use self::reqwest::Url;
use self::hyper::header::Headers;

/// X-MKT-APIKEY: La API key como un string
header! { (XMktApiKey, "X-MKT-APIKEY") => [String] }
/// X-MKT-SIGNATURE: El mensaje firmado generado por el usuario (ver abajo)
header! { (XMktSignature, "X-MKT-SIGNATURE") => [String] }
/// X-MKT-TIMESTAMP: Un timestamp para tu llamada
header! { (XMktTimestamp, "X-MKT-TIMESTAMP") => [String] }

use internal::request::{HttpReq, CryptoMktRequest};

pub struct Api<'a> {
    api_key: &'a str,
    secret_key: &'a str,
    domain: &'a str,
    api_version: &'a str,
    req: Box<HttpReq>,
}

impl<'a> Api<'a> {
    ///
    /// Crea una instancia de tipo API
    ///
    /// #Argumentos
    ///     api_key: Cryptomarket API_KEY
    ///     secret_key: Cryptomarket SECRET_KEY
    ///     http_transport: Interfaz por donde se harían las peticiones Get y Post al servicio
    ///
    pub fn new(
        api_key: &'a str,
        secret_key: &'a str,
        http_transport: Option<Box<HttpReq>>,
    ) -> Self {
        Api {
            api_key: api_key,
            secret_key: secret_key,
            domain: "https://api.cryptomkt.com/",
            api_version: "v1",
            req: http_transport.unwrap_or(Box::new(CryptoMktRequest::new())),
        }
    }
    /// Devuelve el dominio
    pub fn domain(&self) -> &'a str {
        self.domain
    }

    /// Devuelve la version del API
    pub fn api_version(&self) -> &'a str {
        self.api_version
    }

    ///
    /// Construye la URL
    ///
    /// #Argumentos
    ///     endpoint: Endpoint desde donde se va a extraer los datos
    ///     params: Parámetros de la url
    ///
    pub fn build_url(&self, endpoint: &'a str, params: &HashMap<String, String>) -> Url {
        let mut api_url = Url::parse(&self.domain).unwrap();
        // Adiciona la version de la API
        api_url = api_url
            .join(format!("{}/", &self.api_version).as_str())
            .unwrap();
        // Adiciona el endpoint
        api_url = api_url.join(endpoint).unwrap();

        for (key, value) in params {
            api_url
                .query_pairs_mut()
                .append_pair(key.as_str(), value.as_str());
        }
        api_url
    }

    ///
    ///
    /// #Argumentos
    ///     endpoint: Endpoint desde donde se va a extraer los datos
    ///     params: Parámetros de la url
    ///     is_public: indica si el endpoint es public
    ///
    pub fn get_edge(
        &self,
        endpoint: &'a str,
        params: HashMap<String, String>,
        is_public: bool,
    ) -> String {
        let api_url = self.build_url(endpoint, &params);
        let headers = self.build_headers(endpoint, &params, is_public, true);
        match self.req.get(api_url, headers) {
            Ok(resp) => resp,
            Err(_) => "".to_string(),
        }
    }

    ///
    ///
    /// #Argumentos
    ///     endpoint: Endpoint desde donde se va a extraer los datos
    ///     params: Parámetros de la url
    ///     is_public: indica si el endpoint es public
    ///
    pub fn post_edge(&self, endpoint: &'a str, payload: HashMap<String, String>) -> String {
        let api_url = self.build_url(endpoint, &HashMap::new());
        let headers = self.build_headers(endpoint, &payload, false, false);
        match self.req.post(api_url, headers, payload) {
            Ok(resp) => resp,
            Err(_) => "".to_string(),
        }
    }

    ///
    ///  Crea el formato para el header => X-MKT-SIGNATURE
    ///
    /// # Argumentos
    ///     endpoint: Dirección relativa desde donde se van a extraer los datos o donde se enviarán
    ///     payload: Parámetros de la URL
    ///     is_get: Define si el método de encuesta es GET
    ///
    pub fn build_signature_format(
        &self,
        endpoint: &'a str,
        payload: &HashMap<String, String>,
        is_get: bool,
    ) -> String {
        // body = str(timestamp)+'/v1/orders/create' + '0.3' + 'ethclp' + '10000' + 'buy'
        let mut signature: String = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs().to_string(),
            Err(_) => "".to_string(),
        };
        // Adiciona /api_version/endpoint
        signature += format!("/{}/{}", &self.api_version, &endpoint).as_str();
        // si es POST se adicionan los valores de las llaves
        if !is_get {
            let mut keys = payload.keys().collect::<Vec<_>>();
            // Ordena las llaves alfabéticamente
            keys.sort();
            for k in keys {
                signature += payload.get(k).unwrap();
            }
        }
        signature
    }

    ///
    /// Devuelve firmado el mensaje pasado como parámetro
    ///
    /// # Argumentos
    ///     msg: cadena de texto que se requiere firmar
    ///
    pub fn sign_msg(&self, msg: &'a str) -> String {
        let s_key = hmac::SigningKey::new(&digest::SHA384, self.secret_key.as_bytes());
        let sign = hmac::sign(&s_key, msg.as_bytes());

        let mut output = String::new();
        for byte in sign.as_ref() {
            write!(output, "{:02x}", byte).unwrap();
        }

        output
    }
    ///
    /// Conforma los headers para realizar la petición al servidor, en caso de no ser publica
    /// adiciona los headers para la autenticación
    ///
    ///  # Argumentos
    ///     endpoint: Endpoint desde donde se va a extraer los datos
    ///     payload: Parámetros de la url
    ///     is_public: indica si el endpoint es public
    ///     is_get: Define si el método de encuesta es GET
    ///
    fn build_headers(
        &self,
        endpoint: &'a str,
        payload: &HashMap<String, String>,
        is_public: bool,
        is_get: bool,
    ) -> Headers {
        let mut headers = Headers::new();
        if !is_public {
            let msg_to_sign = self.build_signature_format(endpoint, &payload, is_get);
            let timestamp = msg_to_sign.split("/").collect::<Vec<&str>>();
            headers.set(XMktApiKey(self.api_key.to_string()));
            headers.set(XMktSignature(self.sign_msg(msg_to_sign.as_str())));
            headers.set(XMktTimestamp(timestamp.first().unwrap().to_string()))
        }
        headers
    }
}
