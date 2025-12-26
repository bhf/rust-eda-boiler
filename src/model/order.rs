#[derive(Debug)]
pub struct Order {
    id: u64,
    amount: u64,
    instrument_id: u64
}

impl Order {
    pub fn new(id: u64, amount: u64, instrument_id: u64) -> Order {
        Self { id, amount, instrument_id }
    }

    pub fn populate(&mut self, id: u64, amount: u64, instrument_id: u64) {
        self.set_id(id);
        self.set_amount(amount);
        self.set_instrument_id(instrument_id);
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn instrument_id(&self) -> u64 {
        self.instrument_id
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn set_amount(&mut self, amount: u64) {
        self.amount = amount;
    }

    pub fn set_instrument_id(&mut self, instrument_id: u64) {
        self.instrument_id = instrument_id;
    }
}