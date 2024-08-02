use std::fmt;

#[derive(Debug)]
pub enum InvoiceError {
    InvoiceError(String),
    InvoiceLineError(String),
}

impl fmt::Display for InvoiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvoiceError::InvoiceError(e) => write!(f, "Invoice error: {}", e),
            InvoiceError::InvoiceLineError(e) => write!(f, "Invoice Line error: {}", e),
        }
    }
}
