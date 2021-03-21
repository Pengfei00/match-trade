use std::{collections::HashMap, sync::RwLock};

use rust_decimal::Decimal;

use crate::OrderSide;

use super::{queue::ResultQueue, Order, OrderBook};
pub struct Engine {
    order_books: HashMap<String, RwLock<OrderBook>>,
}

impl Engine {
    /// # Examples
    ///
    /// ```
    /// use match_trade::*;
    /// use rust_decimal::Decimal;
    /// use std::str::FromStr;
    /// let mut engine = Engine::new();
    /// let book = engine.add_book("DOGE/BTC",None);
    /// let order = Order::new(1,"DOGE/BTC",Decimal::from_str("1").unwrap(),Decimal::from_str("1").unwrap(),OrderKind::Limit,OrderSide::Buy,1000000000);
    /// engine.add_order(order.clone());
    /// engine.cancel_order("DOGE/BTC",order.id,order.price,order.side);
    /// ```
    pub fn new() -> Self {
        return Engine {
            order_books: HashMap::new(),
        };
    }

    /// 添加标的
    pub fn add_book(
        &mut self,
        symbol: &str,
        result_queue: Option<Box<dyn ResultQueue + Send + Sync>>,
    ) -> &RwLock<OrderBook> {
        let book = OrderBook::new(symbol.to_string(), result_queue);
        self.order_books.insert(symbol.to_string(), RwLock::new(book));
        return self.order_books.get(symbol).unwrap();
    }

    /// 新建order
    pub fn add_order(&self, order: Order) -> Result<bool, &'static str> {
        if let Some(book) = self.order_books.get(&order.symbol) {
            return book.write().unwrap().add_order(order);
        } else {
            return Err("not found symbol");
        }
    }
    
    /// 撤销order
    pub fn cancel_order(
        &self,
        symbol: &str,
        order_id: u64,
        price: Decimal,
        side: OrderSide,
    ) -> Option<Order> {
        if let Some(book) = self.order_books.get(symbol) {
            return book.write().unwrap().calcen_order(order_id, price, side);
        } else {
            return None;
        }
    }

    /// 查看买单，卖单总订单数
    pub fn len(&self) -> (usize, usize) {
        let mut buy = 0;
        let mut sell = 0;
        for (_, v) in &self.order_books {
            let book = v.read().unwrap();
            buy += book.buy_queue.len();
            sell += book.sell_queue.len()
        }
        return (buy, sell);
    }
    
}
