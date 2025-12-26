use crate::model::order::Order;
use crate::services::oms_service::oms_handler_error::OmsHandlerError;

pub trait OMSService {
    fn process_order(&mut self, order: Order) -> Result<(), &OmsHandlerError>;

}