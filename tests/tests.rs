use std::{convert::TryInto, str::FromStr};
use chrono::Utc;
use match_trade::{Engine, Order, OrderKind, OrderSide};
use rust_decimal::Decimal;


#[test]
fn test_limit_order(){
    let mut engine = create_engine();
    create_order(10, OrderKind::Limit, OrderSide::Buy,"100","100", &mut engine);
    assert!(engine.len()==(10,0));
    create_order(10, OrderKind::Limit, OrderSide::Sell,"101","100", &mut engine);
    assert!(engine.len()==(10,10));
    create_order(10, OrderKind::Limit, OrderSide::Sell,"100","10", &mut engine);
    assert!(engine.len()==(9,10));
    create_order(5, OrderKind::Limit, OrderSide::Sell,"100","10", &mut engine);
    assert!(engine.len()==(9,10));
    create_order(5, OrderKind::Limit, OrderSide::Sell,"100","10", &mut engine);
    assert!(engine.len()==(8,10));
    create_order(10, OrderKind::Limit, OrderSide::Buy,"100","10", &mut engine);
    assert!(engine.len()==(18,10));
    create_order(10, OrderKind::Limit, OrderSide::Buy,"101","100", &mut engine);
    assert!(engine.len()==(18,0));
    let order = Order::new(
        1,
        "BTC/DOGE",
        Decimal::from_str("100").unwrap(),
        Decimal::from_str("100").unwrap(),
        OrderKind::Limit,
        OrderSide::Buy,
        Utc::now().timestamp_millis(),
    );
    engine.add_order(order.clone()).unwrap();
    engine.add_order(order.clone()).unwrap_err();
    assert!(engine.len()==(19,0));
}

#[test]
fn test_market_order(){
    let mut engine = create_engine();
    create_order(10, OrderKind::Limit, OrderSide::Buy,"100","100", &mut engine);
    create_order(10, OrderKind::Limit, OrderSide::Buy,"101","100", &mut engine);
    assert!(engine.len()==(20,0));
    create_order(1, OrderKind::Market, OrderSide::Sell,"0","2000", &mut engine);
    assert!(engine.len()==(10,1));
}

#[test]
fn test_fok_order(){
    let mut engine = create_engine();
    create_order(10, OrderKind::Limit, OrderSide::Buy,"100","100", &mut engine);
    assert!(engine.len()==(10,0));
    create_order(1, OrderKind::FOK, OrderSide::Sell,"101","3000", &mut engine);
    assert!(engine.len()==(10,0));
}
#[test]
fn test_ioc_order(){
    let mut engine = create_engine();
    create_order(10, OrderKind::Limit, OrderSide::Buy,"100","100", &mut engine);
    assert!(engine.len()==(10,0));
    create_order(20, OrderKind::IOC, OrderSide::Sell,"100","100", &mut engine);
    assert!(engine.len()==(0,0));
}


fn create_order(count:i64,kind:OrderKind,side:OrderSide,price:&str,volumn:&str,engine:&mut Engine){
    let now = Utc::now().timestamp_nanos();
    for i in  0..count {
        let order = Order::new(
            (now+i).try_into().unwrap(),
            "BTC/DOGE",
            Decimal::from_str(price).unwrap(),
            Decimal::from_str(volumn).unwrap(),
            kind.clone(),
            side.clone(),
            Utc::now().timestamp_millis(),
        );
        match engine.add_order(order){
            Err(e)=>println!("{}",e),
            Ok(v)=>(),
        }
    }
}

fn create_engine()->Engine{
    let mut engine = Engine::new();
    {
        engine.add_book("BTC/DOGE", None);
    }
    return  engine
}