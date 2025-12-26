use std::fmt;

pub struct OmsHandlerError {
    id: u64,
    error_code: OMSError
}

impl OmsHandlerError {
    pub fn new(id: u64) -> OmsHandlerError {
        Self{id, error_code: OMSError::None}
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn error_code(&self) -> &OMSError {
        &self.error_code
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn set_error_code(&mut self, error_code: OMSError) {
        self.error_code = error_code;
    }
}

impl fmt::Display for OmsHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OmsHandlerError: code {}, id {}", self.error_code, self.id)
    }
}

pub enum OMSError{
    InvalidParams,
    InvalidAccount,
    NotConnected,
    None
}

impl fmt::Display for OMSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            OMSError::InvalidParams => "Invalid parameters",
            OMSError::InvalidAccount => "Invalid account",
            OMSError::NotConnected => "Not connected",
            OMSError::None => "No error",
        };
        write!(f, "{}", msg)
    }
}