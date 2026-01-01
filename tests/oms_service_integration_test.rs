use rust_edaboiler::services::oms::OMSService;
use rust_edaboiler::model::order::Order;
use rust_edaboiler::repositories::order_repo::in_memory_order_repository::InMemoryOrderRepository;
use rust_edaboiler::services::oms_service::oms_handler::OmsHandler;
use rust_edaboiler::services::oms_service::oms_handler_error::OmsHandlerError;

#[test]
fn test_process_order() {
    let order_repository = Box::new(InMemoryOrderRepository::new());
    let oms_error = OmsHandlerError::new(1);
    let mut oms_handler = OmsHandler::new(oms_error, order_repository);

    let order = Order::new(123, 100, 101);
    let order_id = order.id();

    let result = oms_handler.process_order(order);

    assert!(result.is_ok());
}
