use crate::model::order::Order;
use crate::repositories::order_repo::in_memory_order_repository::InMemoryOrderRepository;
use crate::services::oms::OMSService;
use crate::services::oms_service::oms_handler::OmsHandler;
use crate::services::oms_service::oms_handler_error::OmsHandlerError;

mod model;
mod repositories;
mod services;

fn main() {
    env_logger::init();

    log::info!("starting up");

    let mut order_repository = InMemoryOrderRepository::new();
    let oms_error = OmsHandlerError::new(1);
    let oms_handler = OmsHandler::new(oms_error, &mut order_repository);

    process_messages(oms_handler);
}

///
/// Process messages from an inbound stream and pass
/// them into the OmsHandler as a flyweight.
///
fn process_messages(mut handler: OmsHandler) {
    // Use a tight loop read some bytes from an input stream and create an order
    let mut order = Order::new(123, 100, 101);
    let order_id = order.id();

    match handler.process_order(order) {
        Ok(value) => {
            log::info!("Processed order: {}", order_id);
        }
        Err(error) => {
            log::error!("Error processing order: {}", error);
        }
    }
}
