use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum OrderSide {
    Buy,
    Sell,
}
#[derive(Clone, Deserialize, Serialize)]
pub enum OrderKind {
    Limit,
    Market,
    IOC,//Immediate-Or-Cancel
    FOK,//Fill-Or-Kill
}
#[derive(Clone, Serialize)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub price: Decimal,
    pub volumn: Decimal,
    pub kind: OrderKind,
    pub side: OrderSide,
    pub timestamp: i64,
}

impl Order {
    pub fn new(
        id: u64,
        symbol: &str,
        price: Decimal,
        volumn: Decimal,
        kind: OrderKind,
        side: OrderSide,
        timestamp: i64,
    ) -> Order {
        Order {
            id,
            symbol: symbol.to_string(),
            price: price,
            volumn: volumn,
            kind,
            side,
            timestamp,
        }
    }
}
