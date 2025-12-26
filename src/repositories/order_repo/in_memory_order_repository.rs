use std::collections::HashMap;
use crate::model::order::Order;
use crate::repositories::order_repository::OrderRepository;

pub struct InMemoryOrderRepository {
    orders: HashMap<u64, Order>,
}

impl InMemoryOrderRepository {
    pub fn new() -> Self {
        Self { orders: HashMap::new() }
    }
}

impl OrderRepository for InMemoryOrderRepository {

    fn put(&mut self, order: Order) {
        self.orders.insert(order.id(), order);
    }

    fn get(&self, order_id: u64) -> Option<&Order> {
        self.orders.get(&order_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::order::Order;

    #[test]
    fn test_put_and_get_order() {
        let mut repo = InMemoryOrderRepository::new();
        let order = Order::new(1, 100, 200);
        repo.put(order);

        let retrieved = repo.get(1);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), 1);
    }

    #[test]
    fn test_get_nonexistent_order() {
        let repo = InMemoryOrderRepository::new();
        assert!(repo.get(999).is_none());
    }

    #[test]
    fn test_overwrite_order() {
        let mut repo = InMemoryOrderRepository::new();
        let order1 = Order::new(1, 100, 200);
        let order2 = Order::new(1, 300, 400);
        repo.put(order1);
        repo.put(order2);

        let retrieved = repo.get(1).unwrap();
        assert_eq!(retrieved.id(), 1);
    }
}
