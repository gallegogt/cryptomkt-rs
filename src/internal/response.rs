
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pagination {
    pub previous: i16,
    pub limit: i16,
    pub page: i16,
    pub next: Option<i16>,
}

///
/// Estructura de la respuesta para el Broker CryptoMkt
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoMktResponse<T> {
    pub status: String,
    pub data: T,
    pub pagination: Option<Pagination>
}

