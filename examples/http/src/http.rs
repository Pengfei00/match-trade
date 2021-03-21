use std::str::FromStr;

use actix_web::{HttpResponse,  Result, web,error};
use match_trade::{Engine,  Order, OrderKind, OrderSide};
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TradeOrderReq {
    order_id: u64,
    price: String,
    symbol: String,
    volumn: String,
    side: OrderSide,
    kind: OrderKind,
    timestamp: i64,
}

#[derive(Deserialize)]
pub struct CancelOrderReq {
    order_id: u64,
    price: String,
    symbol: String,
    side: OrderSide,
}

pub async fn trade(
    data: web::Data<Engine>,
    info: web::Json<TradeOrderReq>,
) -> Result<HttpResponse> {
    if let (Ok(p), Ok(v)) = (
        Decimal::from_str(&info.price),
        Decimal::from_str(&info.volumn),
    ) {
        let order = Order::new(
            info.order_id,
            &info.symbol,
            p,
            v,
            info.kind.clone(),
            info.side.clone(),
            info.timestamp,
        );
        let res = data.add_order(order);
        if let Err(e) = res{
            return Err(error::ErrorBadRequest(e.to_string()));
        }else{
            return Ok(HttpResponse::Ok().body("sucess"))
        }
    } else {
        return Err(error::ErrorBadRequest("error"));
    }
}

pub async fn cancel(
    data: web::Data<Engine>,
    info: web::Json<CancelOrderReq>,
) -> Result<HttpResponse> {
    if let Ok(p) = Decimal::from_str(&info.price) {
        if let Some(order) = data.cancel_order(&info.symbol, info.order_id, p, info.side.clone()) {
            return Ok(HttpResponse::Ok().json(order));
        }
    };
    Err(error::ErrorNotFound("not found"))
}

