use crate::model::order::Order;
use crate::services::validation::ValidationService;

impl ValidationService for Order {
    fn is_valid_instrument(id: u64) -> bool {
        id > 0
    }
    fn is_valid_amount(amount: u64) -> bool {
        amount > 0
    }
}

#[cfg(test)]
mod tests {
    use crate::model::order::Order;
    use crate::services::validation::ValidationService;

    #[test]
    fn test_order_instrument_is_valid() {
        let order = Order::new(1, 1, 1);
        assert!(Order::is_valid_instrument(order.instrument_id));
    }

    #[test]
    fn test_order_instrument_is_invalid() {
        let order = Order::new(0, 1, 0);
        assert!(Order::is_valid_instrument(order.instrument_id));
    }
}
