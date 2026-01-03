use crate::model::order::Order;
use crate::repositories::order_repository::OrderRepository;
use std::collections::HashMap;

pub struct InMemoryOrderRepository {
    orders: HashMap<u64, Order>,
}

impl InMemoryOrderRepository {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
        }
    }
}

impl OrderRepository for InMemoryOrderRepository {
    fn put(&mut self, order: Order) {
        self.orders.insert(order.id(), order);
    }

    fn get(&self, order_id: u64) -> Option<&Order> {
        self.orders.get(&order_id)
    }

    fn remove(&mut self, order_id: u64) -> Option<Order> {
        self.orders.remove(&order_id)
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

    #[test]
    fn test_remove_existing_order() {
        let mut repo = InMemoryOrderRepository::new();
        let order = Order::new(1, 100, 200);
        repo.put(order);
        let removed = repo.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id(), 1);
        assert!(repo.get(1).is_none());
    }

    #[test]
    fn test_remove_nonexistent_order() {
        let mut repo = InMemoryOrderRepository::new();
        let removed = repo.remove(999);
        assert!(removed.is_none());
    }

    #[test]
    fn test_remove_twice() {
        let mut repo = InMemoryOrderRepository::new();
        let order = Order::new(2, 101, 201);
        repo.put(order);
        let first_remove = repo.remove(2);
        assert!(first_remove.is_some());
        let second_remove = repo.remove(2);
        assert!(second_remove.is_none());
    }
}
