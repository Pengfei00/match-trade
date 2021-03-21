use std::collections::HashMap;

use super::{Order, OrderSide};
use rust_decimal::{prelude::Zero, Decimal};
use skiplist::OrderedSkipList;

pub struct OrderQueue {
    pub side: OrderSide,
    pub list: OrderedSkipList<OrderQueueItem>,
    order_ids: HashMap<u64, ()>,
}

impl OrderQueue {
    pub fn new(side: OrderSide, desc: bool) -> OrderQueue {
        // todo 添加队列
        OrderQueue {
            side: side,
            order_ids: HashMap::new(),
            list: unsafe {
                OrderedSkipList::with_comp(move |a: &OrderQueueItem, b: &OrderQueueItem| {
                    let read_a = a;
                    let read_b = b;
                    if desc {
                        // 倒序
                        return read_a
                            .price
                            .partial_cmp(&read_b.price)
                            .expect("Element cannot be ordered.");
                    } else {
                        // 正序
                        return (-read_a.price)
                            .partial_cmp(&-read_b.price)
                            .expect("Element cannot be ordered.");
                    }
                })
            },
        }
    }
    pub fn push(&mut self, order: Order) -> Result<bool, &'static str> {
        if self.order_ids.contains_key(&order.id) {
            return Err("duplicate order_id");
        }
        self.order_ids.insert(order.id, ());
        let price = order.price;
        let mut q = OrderQueueItem::new(price);
        if let Some(mut queue) = self.list.remove_first(&q) {
            queue.push(order);
            self.list.insert(queue);
        } else {
            q.push(order);
            self.list.insert(q);
        }
        Ok(true)
    }

    pub fn first_order<T, F>(&mut self, mut func:F) -> Option<T>
    where
        F: FnMut(Option<&mut Order>) -> Option<T>,
    {
        return self.first_depth(move |queue, this| {
            let remove_order_id: Option<u64>;
            let res: Option<T>;
            if let Some(q) = queue {
                if let Some(order) = q.list.first_mut() {
                    res = func(Some(order));
                    if order.volumn.is_zero() {
                        remove_order_id = Some(order.id);
                    } else {
                        remove_order_id = None;
                    }
                } else {
                    return func(None);
                }
                if let Some(order_id) = remove_order_id {
                    this.remove_queue_item(q, order_id);
                }
                return res;
            } else {
                return func(None);
            }
        });
    }

    pub fn first_depth<F, T>(&mut self, func: F) -> Option<T>
    where
        F: FnOnce(Option<&mut OrderQueueItem>, &mut Self) -> Option<T>,
    {
        if let Some(mut queue) = self.list.pop_front() {
            let res = func(Some(&mut queue), self);
            if queue.len() > 0 {
                self.list.insert(queue);
            }
            return res;
        } else {
            return func(None, self);
        }
    }

    pub fn first_price(&self) -> Option<Decimal> {
        if let Some(queue) = self.list.front() {
            if let Some(order) = queue.list.first() {
                return Some(order.price);
            }
        }
        return None;
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, price: Decimal, order_id: u64) -> Option<Order> {
        if !self.order_ids.contains_key(&order_id) {
            return None;
        }
        let item_queue = OrderQueueItem::new(price);
        if let Some(mut queue) = self.list.remove_first(&item_queue) {
            self.order_ids.remove(&order_id);
            let order = queue.remove(order_id);
            if !queue.list.is_empty() {
                self.list.insert(queue);
            }
            return order;
        } else {
            return None;
        }
    }

    pub fn remove_queue_item(&mut self, queue: &mut OrderQueueItem, order_id: u64) -> bool {
        if !self.order_ids.contains_key(&order_id) {
            return false;
        }
        self.order_ids.remove(&order_id);
        queue.remove(order_id);
        true
    }

    pub fn exists_order_id(&self, order_id: u64) -> bool {
        self.order_ids.contains_key(&order_id)
    }
    pub fn len(&self) -> usize {
        let mut c = 0;
        for i in &self.list {
            c += i.len()
        }
        return c;
    }
}

pub struct OrderQueueItem {
    pub price: Decimal,
    pub list: Vec<Order>,
}

impl OrderQueueItem {
    pub fn new(price: Decimal) -> OrderQueueItem {
        return OrderQueueItem {
            price: price,
            list: Vec::new(),
        };
    }
    fn push(&mut self, order: Order) {
        self.list.push(order);
    }
    pub fn peek_mut(&mut self) -> Option<&mut Order> {
        return self.list.first_mut();
    }
    fn remove(&mut self, order_id: u64) -> Option<Order> {
        let mut index = 0;
        for i in &self.list {
            if i.id == order_id {
                return Some(self.list.remove(index));
            } else {
                index += 1;
            }
        }
        return None;
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        return self.list.len();
    }
    #[allow(dead_code)]
    pub fn volumn(&self) -> Decimal {
        let mut volumn = Decimal::zero();
        for i in &self.list {
            volumn += i.volumn
        }
        return volumn;
    }
}
