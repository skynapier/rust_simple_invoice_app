use super::invoice_line::InvoiceLine;
use crate::errors::invoice_errors::InvoiceError;
use crate::traits::invoice_operations::InvoiceOperations;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub invoice_date: NaiveDate,
    pub invoice_number: Uuid,
    pub line_items: Vec<InvoiceLine>,
}

impl Invoice {
    pub fn new(
        invoice_date: NaiveDate,
        invoice_number: Uuid,
        line_items: Vec<InvoiceLine>,
    ) -> Self {
        Invoice {
            invoice_date,
            invoice_number,
            line_items,
        }
    }

    pub fn copy_line_items(&self) -> Vec<InvoiceLine> {
        self.line_items
            .iter()
            .map(|item| item.clone())
            .collect::<Vec<InvoiceLine>>()
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl InvoiceOperations for Invoice {
    fn add_line_item(&mut self, item: InvoiceLine) {
        self.line_items.push(item);
    }

    fn remove_invoice_line(&mut self, id: &Uuid) -> Result<(), InvoiceError> {
        match self
            .line_items
            .iter()
            .position(|item| &item.invoice_line_id == id)
        {
            Some(found_item_index) => {
                self.line_items.remove(found_item_index);
                Ok(())
            }
            None => Err(InvoiceError::InvoiceError(format!(
                "Cannot find line item with id {id}"
            ))),
        }
    }

    fn get_total(&self) -> f64 {
        let item_totals = self
            .line_items
            .iter()
            .map(|item| item.count_total())
            .collect::<Vec<f64>>();

        item_totals.iter().sum()
    }

    fn merge_invoices(&mut self, other: &Invoice) -> Result<(), InvoiceError> {
        let mut new_line_items = self.copy_line_items();

        other.line_items.iter().for_each(|item| {
            new_line_items.push(item.clone());
        });

        self.line_items = new_line_items;

        Ok(())
    }

    fn clone(&self) -> Result<Invoice, InvoiceError> {
        Ok(Invoice {
            invoice_date: self.invoice_date.clone(),
            invoice_number: self.invoice_number.clone(),
            line_items: self.copy_line_items(),
        })
    }
}
