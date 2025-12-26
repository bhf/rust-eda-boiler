use crate::model::order::Order;
use crate::services::oms::OMSService;
use crate::services::oms_service::oms_handler_error::OmsHandlerError;
use crate::services::validation::OrderValidationService;
use crate::repositories::order_repository::OrderRepository;


pub struct OmsHandler<'a>{
    pub error: OmsHandlerError,
    pub order_repo: &'a mut dyn OrderRepository,
}

impl<'a> OmsHandler<'a> {
    pub fn new(error: OmsHandlerError, order_repo: &'a mut dyn OrderRepository) -> Self {
        Self { error, order_repo }
    }
}

impl OMSService for OmsHandler<'_> {

    fn process_order(&mut self, order: Order) -> Result<(), &OmsHandlerError> {

        if !Order::is_valid_instrument(order.instrument_id()) {
            return Err(&self.error)
        }

        self.order_repo.put(order);

        Ok(())
    }

}
