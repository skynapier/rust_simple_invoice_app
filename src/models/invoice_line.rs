use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceLine {
    pub invoice_line_id: Uuid,
    pub cost: f64,
    pub quantity: i32,
    pub description: String,
}

impl InvoiceLine {
    pub fn new(invoice_line_id: Uuid, cost: f64, quantity: i32, description: String) -> Self {
        InvoiceLine {
            invoice_line_id,
            cost,
            quantity,
            description,
        }
    }

    pub fn count_total(&self) -> f64 {
        self.cost * f64::from(self.quantity)
    }
}
