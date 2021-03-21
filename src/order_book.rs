use std::cmp::min;
use rust_decimal::{prelude::Zero, Decimal};

use super::{queue::ResultQueue, Order, OrderQueue, OrderSide};

pub struct OrderBook {
    pub symbol: String,
    pub buy_queue: OrderQueue,
    pub sell_queue: OrderQueue,
    pub result_queue: Option<Box<dyn ResultQueue + Send + Sync>>,
}

impl OrderBook {
    pub fn new(
        symbol: String,
        result_queue: Option<Box<dyn ResultQueue + Send + Sync>>,
    ) -> Self {
        let book = OrderBook {
            symbol: symbol,
            buy_queue: OrderQueue::new(OrderSide::Buy, false),
            sell_queue: OrderQueue::new(OrderSide::Sell, true),
            result_queue: result_queue,
        };
        return book;
    }


    fn trade(&mut self, taker_order: &mut Order) -> bool {
        // 返回true为 需要放入队列
        let queue: &mut OrderQueue;
        match taker_order.side {
            OrderSide::Buy => {
                // 获取最新卖价
                let maker_price = self.sell_queue.first_price();
                // 卖价大于买价 或者没有卖单 单子放入队列
                if maker_price.is_none() || maker_price.unwrap() > taker_order.price {
                    return true;
                }
                // 否则开始搓单
                queue = &mut self.sell_queue;
            }
            OrderSide::Sell => {
                // 获取最新买价
                let maker_price = self.buy_queue.first_price();
                // 买价低于买价 或者没有买单 单子放入队列
                if maker_price.is_none() || maker_price.unwrap() < taker_order.price {
                    return true;
                }
                queue = &mut self.buy_queue;
                // 否则开始搓单
            }
        };

        if let Some((maker_order_id, volumn, price)) = queue.first_order(|value| {
            if let Some(maker_order) = value {
                let volumn = min(maker_order.volumn, taker_order.volumn);
                maker_order.volumn -= volumn;
                taker_order.volumn -= volumn;
                return Some((maker_order.id, volumn, maker_order.price));
            };
            return None;
        }) {
            if let Some(result_queue) = &self.result_queue {
                result_queue.trade_success(maker_order_id, taker_order.id, volumn, price);
            };
        }
        return false;
    }

    /// 查看是否存在当前订单号
    pub fn exists_order(&self, order: &Order) -> bool {
        match order.side {
            OrderSide::Buy => self.buy_queue.exists_order_id(order.id),
            OrderSide::Sell => self.sell_queue.exists_order_id(order.id),
        }
    }

    /// 新建限价单
    pub fn limit(&mut self, mut taker_order: Order) -> Result<bool, &'static str> {
        loop {
            if self.trade(&mut taker_order) {
                return match taker_order.side {
                    OrderSide::Buy => self.buy_queue.push(taker_order),
                    OrderSide::Sell => self.sell_queue.push(taker_order),
                };
            } else if taker_order.volumn.is_zero() {
                return Ok(true);
            }
        }
    }

    /// 新建市价单
    pub fn market(&mut self, mut order: Order) -> Result<bool, &'static str> {
        match order.side {
            OrderSide::Buy => {
                if let Some(first_price) = self.sell_queue.first_price() {
                    order.price = first_price;
                    return self.limit(order);
                } else {
                    if let Some(result_queue) = &self.result_queue {
                        result_queue.cancel_order(order.id, order.volumn)
                    }
                    return Err("empty sell queue");
                }
            }
            OrderSide::Sell => {
                if let Some(first_price) = self.buy_queue.first_price() {
                    order.price = first_price;
                    return self.limit(order);
                } else {
                    if let Some(result_queue) = &self.result_queue {
                        result_queue.cancel_order(order.id, order.volumn)
                    }
                    return Err("empty buy queue");
                }
            }
        }
    }

    /// 新建IOC单
    pub fn ioc(&mut self, mut taker_order: Order) -> Result<bool, &'static str> {
        self.trade(&mut taker_order);
        if !taker_order.volumn.is_zero() {
            if let Some(result_queue) = &self.result_queue {
                result_queue.cancel_order(taker_order.id, taker_order.volumn)
            };
        }
        Ok(true)
    }
    
    /// 新建FOK单
    pub fn fok(&mut self, taker_order: Order) -> Result<bool, &'static str> {
        let less;
        match taker_order.side {
            OrderSide::Buy => {
                less = false;
            }
            OrderSide::Sell => {
                less = true;
            }
        };

        let mut volumn = Decimal::zero();
        'outer: for queue in &self.sell_queue.list {
            for maker_order in &queue.list {
                if volumn >= taker_order.volumn {
                    break 'outer;
                } else if (less && maker_order.price < taker_order.price)
                    || (!less && maker_order.price > taker_order.price)
                {
                    break 'outer;
                } else {
                    volumn += maker_order.volumn;
                }
            }
        }
        if volumn >= taker_order.volumn {
            return self.limit(taker_order);
        } else {
            if let Some(result_queue) = &self.result_queue {
                result_queue.cancel_order(taker_order.id, taker_order.volumn)
            };
            return Ok(true);
        }
    }

    /// 新建订单通过订单kind判断 下单类型
    pub fn add_order(&mut self, order: Order) -> Result<bool, &'static str> {
        if self.exists_order(&order) {
            return Err("duplicate order_id");
        }
        return match order.kind {
            super::OrderKind::Limit => self.limit(order),
            super::OrderKind::Market => self.market(order),
            super::OrderKind::IOC => self.ioc(order),
            super::OrderKind::FOK => self.fok(order),
        };
    }

    /// 撤销订单
    pub fn calcen_order(
        &mut self,
        order_id: u64,
        price: Decimal,
        side: OrderSide,
    ) -> Option<Order> {
        return match side {
            super::OrderSide::Buy => self.buy_queue.remove(price, order_id),
            super::OrderSide::Sell => self.sell_queue.remove(price, order_id),
        };
    }
}
