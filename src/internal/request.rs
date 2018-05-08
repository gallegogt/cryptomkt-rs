extern crate reqwest;

use self::reqwest::{Client};
use self::reqwest::header::Headers;
use self::reqwest::{StatusCode, Url};
use std::collections::HashMap;

#[derive(Debug)]
pub enum RequestError {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    Gone,
    Teapot,
    TooManyRequests,
    InternalServerError,
    ServiceUnavailable,
}

pub trait HttpReq {
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: Headers
    ///
    fn get(&self, url: Url, headers: Headers) -> Result<String, RequestError>;
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: Headers
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn post(
        &self,
        url: Url,
        headers: Headers,
        payload: HashMap<String, String>,
    ) -> Result<String, RequestError>;
}

pub struct CryptoMktRequest {
    client: Box<Client>,
}

impl CryptoMktRequest {
    ///
    /// Devuelve una nueva instancia
    ///
    pub fn new() -> Self {
        CryptoMktRequest {
            client: Box::new(Client::new()),
        }
    }
}

impl HttpReq for CryptoMktRequest {
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: Headers
    ///
    fn get(&self, url: Url, headers: Headers) -> Result<String, RequestError> {
        let result = self.client.get(url).headers(headers).send();
        match result {
            Ok(mut resp) => match resp.status() {
                StatusCode::Ok => match resp.text() {
                    Ok(txt) => Ok(txt),
                    Err(e) => {
                        error!(target: "cryptomarket", "GET: Request Text details: {:?}", e);
                        Err(RequestError::BadRequest)
                    }
                },

                StatusCode::Unauthorized => {
                    error!(target: "cryptomarket", "GET: StatusCode: {:?}", StatusCode::Unauthorized);
                    Err(RequestError::Unauthorized)
                }

                StatusCode::Forbidden => {
                    error!(target: "cryptomarket", "GET: StatusCode: {:?}", StatusCode::Forbidden);
                    Err(RequestError::Forbidden)
                }
                StatusCode::NotFound => {
                    error!(target: "cryptomarket", "GET: StatusCode: {:?}", StatusCode::NotFound);
                    Err(RequestError::NotFound)
                }
                StatusCode::MethodNotAllowed => {
                    error!(target: "cryptomarket", "GET: StatusCode: {:?}", StatusCode::MethodNotAllowed);
                    Err(RequestError::MethodNotAllowed)
                }
                StatusCode::NotAcceptable => {
                    error!(target: "cryptomarket", "GET: StatusCode: {:?}", StatusCode::NotAcceptable);
                    Err(RequestError::NotAcceptable)
                }
                StatusCode::Gone => {
                    error!(target: "cryptomarket", "GET: StatusCode: {:?}", StatusCode::Gone);
                    Err(RequestError::Gone)
                }
                StatusCode::TooManyRequests => {
                    error!(target: "cryptomarket", "GET: StatusCode: {:?}", StatusCode::TooManyRequests);
                    Err(RequestError::TooManyRequests)
                }
                StatusCode::InternalServerError => {
                    error!(target: "cryptomarket", "GET: StatusCode: {:?}", StatusCode::InternalServerError);
                    Err(RequestError::InternalServerError)
                }
                StatusCode::ServiceUnavailable => {
                    error!(target: "cryptomarket", "GET: StatusCode: {:?}", StatusCode::ServiceUnavailable);
                    Err(RequestError::ServiceUnavailable)
                }
                status => {
                    error!(target: "cryptomarket", "GET: StatusCode: {:?} Code({:?})", status, status.as_u16());
                    if status.as_u16() == 418 {
                        Err(RequestError::Teapot)
                    } else {
                        Err(RequestError::BadRequest)
                    }
                }
            },
            Err(e) => {
                error!(target: "cryptomarket", "GET {:?}", e);
                Err(RequestError::BadRequest)
            }
        }
    }
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: Headers
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn post(
        &self,
        url: Url,
        headers: Headers,
        payload: HashMap<String, String>,
    ) -> Result<String, RequestError> {
        let result = self.client.post(url).headers(headers).form(&payload).send();

        match result {
            Ok(mut resp) => match resp.status() {
                StatusCode::Ok => match resp.text() {
                    Ok(txt) => Ok(txt),
                    Err(e) => {
                        error!(target: "cryptomarket", "POST: Response Details: {:?}", e);
                        Err(RequestError::BadRequest)
                    }
                },
                StatusCode::Unauthorized => {
                    error!(target: "cryptomarket", "POST: StatusCode: {:?}", StatusCode::Unauthorized);
                    Err(RequestError::Unauthorized)
                }

                StatusCode::Forbidden => {
                    error!(target: "cryptomarket", "POST: StatusCode: {:?}", StatusCode::Forbidden);
                    Err(RequestError::Forbidden)
                }
                StatusCode::NotFound => {
                    error!(target: "cryptomarket", "POST: StatusCode: {:?}", StatusCode::NotFound);
                    Err(RequestError::NotFound)
                }
                StatusCode::MethodNotAllowed => {
                    error!(target: "cryptomarket", "POST: StatusCode: {:?}", StatusCode::MethodNotAllowed);
                    Err(RequestError::MethodNotAllowed)
                }
                StatusCode::NotAcceptable => {
                    error!(target: "cryptomarket", "POST: StatusCode: {:?}", StatusCode::NotAcceptable);
                    Err(RequestError::NotAcceptable)
                }
                StatusCode::Gone => {
                    error!(target: "cryptomarket", "POST: StatusCode: {:?}", StatusCode::Gone);
                    Err(RequestError::Gone)
                }
                StatusCode::TooManyRequests => {
                    error!(target: "cryptomarket", "POST: StatusCode: {:?}", StatusCode::TooManyRequests);
                    Err(RequestError::TooManyRequests)
                }
                StatusCode::InternalServerError => {
                    error!(target: "cryptomarket", "POST: StatusCode: {:?}", StatusCode::InternalServerError);
                    Err(RequestError::InternalServerError)
                }
                StatusCode::ServiceUnavailable => {
                    error!(target: "cryptomarket", "POST: StatusCode: {:?}", StatusCode::ServiceUnavailable);
                    Err(RequestError::ServiceUnavailable)
                }
                status => {
                    error!(target: "cryptomarket", "POST: StatusCode: {:?} Code({:?})", status, status.as_u16());
                    if status.as_u16() == 418 {
                        Err(RequestError::Teapot)
                    } else {
                        Err(RequestError::BadRequest)
                    }
                }
            },
            Err(e) => {
                error!(target: "cryptomarket", "POST {:?}", e);
                Err(RequestError::BadRequest)
            }
        }
    }
}
