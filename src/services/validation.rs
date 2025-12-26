pub trait OrderValidationService {
    fn is_valid_instrument(id: u64) -> bool;
    fn is_valid_amount(amount: u64) -> bool;
}

pub trait ExecutionValidationService {
    fn is_valid_account(id: u64) -> bool;
    fn is_valid_amount(amount: u64) -> bool;
}