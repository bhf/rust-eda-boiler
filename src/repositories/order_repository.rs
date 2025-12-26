use crate::model::order::Order;

pub trait OrderRepository {
    fn put(&mut self, order: Order);
    fn get(&self, order_id: u64) -> Option<&Order>;

}