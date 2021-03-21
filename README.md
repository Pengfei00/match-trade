# match trade engine
撮合引擎

Supported features:

* limit orders
* market orders
* ioc orders
* fok orders
* cancelling order

## example
examples/http
```
use match_trade::*;
use rust_decimal::Decimal;
use std::str::FromStr;
let mut engine = Engine::new();
let book = engine.add_book("DOGE/BTC",None);
let order = Order::new(1,"DOGE/BTC",Decimal::from_str("1").unwrap(),Decimal::from_str("1").unwrap(),OrderKind::Limit,OrderSide::Buy,1000000000);
engine.add_order(order.clone());
engine.cancel_order("DOGE/BTC",order.id,order.price,order.side);
```