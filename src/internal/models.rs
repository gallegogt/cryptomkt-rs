use serde_json::Value;

///
/// The ticker is a high-level overview of the state of the market. It will show you
/// the current bid and ask, as well as the latest market price. It also includes
/// information such as the daily volume and how much the price has moved during the last day.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticker {
    /// Highest price
    pub high: String,
    /// Lowest price
    pub low: String,
    /// Purchase price
    pub ask: String,
    /// Sale price
    pub bid: String,
    /// Last transaction price
    pub last_price: String,
    /// Market volume
    pub volume: String,
    /// Market pair
    pub timestamp: String,
    /// Date of consultation
    pub market: String,
}

///
/// A market order corresponds to a purchase or sale request within the
/// Exchange Market of CryptoMarket.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    /// Order limit price
    pub price: String,
    /// Order quantity
    pub timestamp: String,
    /// Creation date
    pub amount: String,
}

///
/// They correspond to transactions made in CryptoMarket.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    /// Transaction Type. buy or sell
    pub market_taker: String,
    /// Price at which the transaction was made
    pub price: String,
    /// Amount of the transaction
    pub amount: String,
    /// ID of the transaction
    #[serde(default)]
    pub tid: String,
    /// Date of the transaction
    pub timestamp: String,
    /// Market pair where the transaction was made
    pub market: String,
}


///
/// Amount
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Amount {
    /// Original quantity of the order
    #[serde(default)]
    pub original: String,
    /// Remaining amount of the order. Only in active orders
    #[serde(default)]
    pub remaining: String,
    /// Quantity executed of the order. Only in executed orders
    #[serde(default)]
    pub executed: String,
}

///
/// A market order corresponds to a purchase or sale request within the
/// Exchange Market of CryptoMarket.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    /// Order ID
    #[serde(default)]
    pub id: String,
    /// Order Status, active o executed
    #[serde(default)]
    pub status: String,
    /// Order Type. buy o sell
    #[serde(rename = "type")]
    pub order_type: String,
    /// Order limit price
    #[serde(default)]
    pub price: String,
    /// Ammount
    pub amount: Amount,
    /// Execution price
    #[serde(default)]
    pub execution_price: Value, // null / String
    /// Average weighted execution price. 0 if it is not executed.
    #[serde(default)]
    pub avg_execution_price: String,
    /// Market pair
    #[serde(default)]
    pub market: String,
    /// Creation date
    #[serde(default)]
    pub created_at: String,
    /// Update date. Only in active orders
    #[serde(default)]
    pub updated_at: String,
    /// Date of execution Only in executed orders
    #[serde(default)]
    pub executed_at: String,
}



///
/// An instant order corresponds to a purchase or sale request within the Instant
/// Exchange of CryptoMarket.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrdersInstant {
    /// If it is a purchase purchase request, it corresponds to the amount of
    /// cryptocurrency to receive if the purchase was made. If it is a sell sell
    /// request, it corresponds to the amount of local currency to be received if the sale is made.
    #[serde(default)]
    pub obtained: String,
    /// If it is a buy type request, it corresponds to the amount of local currency that you want
    /// to use to make the purchase. If type is sell, it corresponds to the amount of cryptocurrency
    /// that you want to use for the sale. Amount less than or equal to the amount requested. Modified
    /// by market liquidity.
    #[serde(default)]
    pub required: String,
}


///
/// A balance corresponds to the status of your cryptocurrency and local wallets
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
    /// Wallet at CryptoMarket
    pub wallet: String,
    /// Balance available
    pub available: String,
    /// Countable balance
    pub balance: String,
}


///
/// A balance corresponds to the status of your cryptocurrency and local wallets
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payment {
    /// Internal ID of the payment order
    pub id: i32,
    /// External ID
    pub external_id: String,
    /// State of the payment order. See below
    pub status: String,
    /// Amount of the payment order
    pub to_receive: String,
    /// Type of currency to be received for the payment order
    pub to_receive_currency: String,
    /// Amount waiting for the order to be accepted
    pub expected_amount: String,
    /// Type of currency waiting for the order to be accepted
    pub expected_currency: String,
    /// Payment order address
    pub deposit_address: String,
    /// Contact email to coordinate refunds
    pub refund_email: String,
    /// Url of the image of the order of payment QR
    pub qr: String,
    /// Observations
    pub obs: String,
    /// Notification URL
    pub callback_url: String,
    /// Error url
    pub error_url: String,
    /// Success URL
    pub success_url: String,
    /// Payment order voucher url
    pub payment_url: String,
    /// Creation date of the payment order
    pub created_at: String,
    /// Date of update of the payment order
    pub updated_at: String,
}
