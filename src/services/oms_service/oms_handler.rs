use crate::model::order::Order;
use crate::services::oms::OMSService;
use crate::services::oms_service::oms_handler_error::{OMSError, OmsHandlerError};
use crate::services::validation::OrderValidationService;
use crate::repositories::order_repository::OrderRepository;


pub struct OmsHandler{
    pub error: OmsHandlerError,
    pub order_repo: Box<dyn OrderRepository>,
}

impl OmsHandler {
    pub fn new(error: OmsHandlerError, order_repo: Box<dyn OrderRepository>) -> Self {
        Self { error, order_repo }
    }
}

impl OMSService for OmsHandler {

    fn process_order(&mut self, order: Order) -> Result<(), &OmsHandlerError> {

        if !Order::is_valid_instrument(order.instrument_id()) {
            self.error.set_error_code(OMSError::InvalidInstrument);
            self.error.set_id(order.id());
            return Err(&self.error)
        }

        if !Order::is_valid_amount(order.instrument_id()) {
            self.error.set_error_code(OMSError::InvalidAmount);
            self.error.set_id(order.id());
            return Err(&self.error)
        }

        self.order_repo.put(order);

        Ok(())
    }

}
