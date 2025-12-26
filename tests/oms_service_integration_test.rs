use oms::services::oms::OMSService;
use oms::model::order::Order;
use oms::repositories::order_repo::in_memory_order_repository::InMemoryOrderRepository;
use oms::services::oms_service::oms_handler::OmsHandler;
use oms::services::oms_service::oms_handler_error::OmsHandlerError;

#[test]
fn test_process_order() {
    let mut order_repository = InMemoryOrderRepository::new();
    let oms_error = OmsHandlerError::new(1);
    let mut oms_handler = OmsHandler::new(oms_error, &mut order_repository);

    let order = Order::new(123, 100, 101);
    let order_id = order.id();

    let result = oms_handler.process_order(order);

    assert!(result.is_ok());
}
