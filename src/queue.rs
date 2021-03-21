use rust_decimal::Decimal;

pub trait ResultQueue{
    /// 成交成功
    fn trade_success(&self,maker_id:u64,taker_id:u64,volumn:Decimal,price:Decimal);

    /// 订单被动取消 （主动撤销订单不会调用)
    fn cancel_order(&self,order_id:u64,volumn:Decimal);
}
pub struct NoneQueue{
}

impl ResultQueue for NoneQueue {
    #[allow(unused_variables)]
    fn trade_success(&self,maker_id:u64,taker_id:u64,volumn:Decimal,price:Decimal) {
       
    }

    #[allow(unused_variables)]
    fn cancel_order(&self,order_id:u64,volumn:Decimal) {
    }
}
