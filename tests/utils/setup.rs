use chrono::Utc;
use rand::Rng;
use rust_invoice_app::models::invoice::Invoice;
use rust_invoice_app::models::invoice_line::InvoiceLine;
use uuid::Uuid;

pub fn create_random_cost_quantity() -> (f64, i32) {
    let mut rng = rand::thread_rng();
    let random_cost: f64 = rng.gen_range(0.0..10.0);
    let random_quantity: i32 = rng.gen_range(0..10);

    (random_cost, random_quantity)
}

pub fn create_invoice_line(cost: f64, quantity: i32) -> InvoiceLine {
    InvoiceLine {
        invoice_line_id: Uuid::new_v4(),
        cost: cost,
        quantity: quantity,
        description: String::from("test random invoice line item"),
    }
}

pub fn create_invoice(cost: f64, quantity: i32) -> Invoice {
    Invoice {
        invoice_date: Utc::now().date_naive(),
        invoice_number: Uuid::new_v4(),
        line_items: vec![create_invoice_line(cost, quantity)],
    }
}

pub fn setup_invoice() -> Invoice {
    let (cost, quantity) = create_random_cost_quantity();
    create_invoice(cost, quantity)
}

pub fn setup_invoice_line() -> InvoiceLine {
    let (cost, quantity) = create_random_cost_quantity();
    create_invoice_line(cost, quantity)
}
