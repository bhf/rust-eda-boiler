pub struct Order {
    id: u64,
    amount: u64,
    instrument_id: u64
}

impl Order {
    pub fn new(id: u64, amount: u64, instrument_id: u64) -> Order {
        Self { id, amount, instrument_id }
    }

    pub fn update_amount(&mut self, amount: u64) {
        self.amount += amount;
    }
}