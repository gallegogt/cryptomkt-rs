use serde_json::Value;

// ============ Ticker ==============
/// El ticker es una visión general de alto nivel del estado del mercado.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticker {
    /// Precio más alto
    pub high: String,
    /// Precio más bajo
    pub low: String,
    /// Precio de compra
    pub ask: String,
    /// Precio de venta
    pub bid: String,
    /// Precio última transacción
    pub last_price: String,
    /// Volumen del mercado
    pub volume: String,
    /// Fecha de consulta
    pub timestamp: String,
    /// Par de mercado
    pub market: String,
}

// ============ Book ==============
/// Ordenes activas
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    /// Precio límite de la orden
    pub price: String,
    /// Cantidad de la orden
    pub timestamp: String,
    /// Fecha de creación
    pub amount: String,
}

// ============ Trades ==============
/// Trades realizados en CryptoMarket.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    /// Tipo de transacción. buy o sell
    pub market_taker: String,
    /// Precio al cual se realizó la transacción
    pub price: String,
    /// Cantidad de la transacción
    pub amount: String,
    /// ID de la transacción
    #[serde(default)]
    pub tid: String,
    /// Fecha de la transacción
    pub timestamp: String,
    /// Par de mercado donde se realizó la transacción
    pub market: String,
}

// ============ Order ==============
/// Amount
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Amount {
    /// Cantidad original de la orden
    #[serde(default)]
    pub original: String,
    /// Cantidad restante de la orden. Solo en órdenes activas
    #[serde(default)]
    pub remaining: String,
    /// Cantidad ejecutada de la orden. Solo en órdenes ejecutadas
    #[serde(default)]
    pub executed: String,
}

/// Orden del Mercado, corresponde a una solicitud de compra o
/// venta dentro del Exchange Market de CryptoMarket.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    /// ID de la orden
    #[serde(default)]
    pub id: String,
    /// Estado de la orden. active o executed
    #[serde(default)]
    pub status: String,
    /// Tipo de orden. buy o sell
    #[serde(rename = "type")]
    pub order_type: String,
    /// Precio límite de la orden
    #[serde(default)]
    pub price: String,
    /// Cantidad
    pub amount: Amount,
    /// Precio de ejecución
    #[serde(default)]
    pub execution_price: Value, // null / String
    /// Precio de ejecución promedio ponderado. 0 si no se ejecuta.
    #[serde(default)]
    pub avg_execution_price: String,
    /// Par de mercado
    #[serde(default)]
    pub market: String,
    /// Fecha de creación
    #[serde(default)]
    pub created_at: String,
    /// Fecha de actualización. Solo en órdenes activas
    #[serde(default)]
    pub updated_at: String,
    /// Fecha de ejecución. Solo en órdenes ejecutadas
    #[serde(default)]
    pub executed_at: String,
}

// ============ Órdenes instantáneas ==============

/// Una orden instantánea corresponde a una solicitud de compra o venta dentro del Instant
/// Exchange de CryptoMarket.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrdersInstant {
    /// Si es una petición de compra buy, corresponde a la cantidad de
    /// criptomoneda a recibir si se efectuase la compra. Si es una petición
    /// de venta sell, corresponde a la cantidad de moneda local a recibir si
    /// se efectuase la venta.
    #[serde(default)]
    pub obtained: String,
    /// Si es una petición de tipo buy, corresponde a la cantidad de moneda
    /// local que se quiere utilizar para realizar la compra. Si type es sell,
    /// corresponde a la cantidad de criptomoneda que se quiere utilizar para la venta.
    /// Monto menor o igual a la cantidad solicitada. Modificado por la liquidez del mercado.
    #[serde(default)]
    pub required: String,
}

// ============ Balance ==============
/// Un balance corresponde al estado de tus billeteras de criptomonedas y locales
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
    /// Billetera en CryptoMarket
    pub wallet: String,
    /// Saldo disponible
    pub available: String,
    /// Saldo contable
    pub balance: String,
}

// ============ Orden pago ==============
/// Un balance corresponde al estado de tus billeteras de criptomonedas y locales
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payment {
    /// ID interno de la orden de pago
    pub id: i32,
    /// ID externo
    pub external_id: String,
    /// Estado de la orden de pago. Ver más abajo
    pub status: String,
    /// Monto de la orden de pago
    pub to_receive: String,
    /// Tipo de moneda a recibir por la orden de pago
    pub to_receive_currency: String,
    /// Cantidad que espera la orden para ser aceptada
    pub expected_amount: String,
    /// Tipo de moneda que espera la orden para ser aceptada
    pub expected_currency: String,
    /// Dirección de la orden de pago
    pub deposit_address: String,
    /// Correo electrónico de contacto para coordinar reembolsos
    pub refund_email: String,
    /// Url de la imagen QR de la orden de pago
    pub qr: String,
    /// Observaciones
    pub obs: String,
    /// Url de notificación
    pub callback_url: String,
    /// Url de error
    pub error_url: String,
    /// Url de éxito
    pub success_url: String,
    /// Url de voucher de orden de pago
    pub payment_url: String,
    /// Fecha de creación de la orden de pago
    pub created_at: String,
    /// Fecha de actualización de la orden de pago
    pub updated_at: String,
}
