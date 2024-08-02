use crate::errors::invoice_errors::InvoiceError;
use crate::models::invoice::Invoice;
use crate::models::invoice_line::InvoiceLine;
use uuid::Uuid;

pub trait InvoiceOperations {
    fn add_line_item(&mut self, item: InvoiceLine);

    // originally is number not sure if its index or id
    fn remove_invoice_line(&mut self, id: &Uuid) -> Result<(), InvoiceError>;

    // default case will be 0.0
    fn get_total(&self) -> f64;

    // on which id? date?
    fn merge_invoices(&mut self, other: &Invoice) -> Result<(), InvoiceError>;

    fn clone(&self) -> Result<Invoice, InvoiceError>;
}
