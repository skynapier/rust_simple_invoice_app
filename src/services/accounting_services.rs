use chrono::Utc;
use uuid::Uuid;

use crate::models::invoice::Invoice;
use crate::models::invoice_line::InvoiceLine;
use crate::traits::accounting_operations::AccoungtingOperations;
use crate::traits::invoice_operations::InvoiceOperations;

#[derive(Default)]
pub struct AccountingServices {}

impl AccountingServices {
    pub fn create_new_invoice(&self) -> Invoice {
        Invoice {
            invoice_date: Utc::now().date_naive(),
            invoice_number: Uuid::new_v4(),
            line_items: vec![],
        }
    }
}

impl AccoungtingOperations for AccountingServices {
    fn create_invoice_with_one_item(&self) {
        let mut invoice = self.create_new_invoice();
        let invoice_line = InvoiceLine::new(Uuid::new_v4(), 6.99, 1, String::from("Apple"));
        invoice.add_line_item(invoice_line);
        println!("{:?}", invoice.line_items);
    }

    fn create_invoice_with_multiple_items_and_quantities(&self) {
        let mut invoice = self.create_new_invoice();

        invoice.add_line_item(InvoiceLine::new(
            Uuid::new_v4(),
            10.21,
            4,
            String::from("Banana"),
        ));
        invoice.add_line_item(InvoiceLine::new(
            Uuid::new_v4(),
            5.21,
            1,
            String::from("Orange"),
        ));
        invoice.add_line_item(InvoiceLine::new(
            Uuid::new_v4(),
            6.21,
            5,
            String::from("Pineapple"),
        ));

        println!("{:?}", invoice.get_total());
    }

    fn remove_item(&self) {
        let mut invoice = self.create_new_invoice();
        let line_item_id_1 = Uuid::new_v4();
        let line_item_id_2 = Uuid::new_v4();
        invoice.add_line_item(InvoiceLine::new(
            line_item_id_1,
            10.21,
            1,
            String::from("Orange"),
        ));
        invoice.add_line_item(InvoiceLine::new(
            line_item_id_2,
            10.99,
            5,
            String::from("Banana"),
        ));

        invoice
            .remove_invoice_line(&line_item_id_1)
            .map_err(|err| println!("Error {err}"))
            .unwrap();

        println!("{:?}", invoice.get_total());
    }

    fn merge_invoices(&self) {
        let mut invoice1 = self.create_new_invoice();
        invoice1.add_line_item(InvoiceLine::new(
            Uuid::new_v4(),
            10.21,
            1,
            String::from("Blueberries"),
        ));

        let mut invoice2 = self.create_new_invoice();
        invoice2.add_line_item(InvoiceLine::new(
            Uuid::new_v4(),
            5.29,
            4,
            String::from("Orange"),
        ));
        invoice2.add_line_item(InvoiceLine::new(
            Uuid::new_v4(),
            9.99,
            1,
            String::from("Banana"),
        ));

        invoice1
            .merge_invoices(&invoice2)
            .map_err(|err| println!("Error {err}"))
            .unwrap();
        println!("invoices {:?}", invoice1);
        println!("{:?}", invoice1.get_total());
    }

    fn clone_invoice(&self) {
        let mut invoice = self.create_new_invoice();

        invoice.add_line_item(InvoiceLine::new(
            Uuid::new_v4(),
            0.99,
            5,
            String::from("Onion"),
        ));
        invoice.add_line_item(InvoiceLine::new(
            Uuid::new_v4(),
            10.49,
            2,
            String::from("Watermelon"),
        ));

        let cloned_invoice = invoice
            .clone()
            .map_err(|err| println!("Cloned Error {err}"))
            .unwrap();

        println!(
            "original invoice total {:?}, get cloned total {:?}",
            invoice.get_total(),
            cloned_invoice.get_total()
        );
    }

    fn invoice_to_string(&self) {
        let invoice = Invoice::new(
            Utc::now().date_naive(),
            Uuid::new_v4(),
            vec![InvoiceLine::new(
                Uuid::new_v4(),
                1.99,
                20,
                String::from("Peer"),
            )],
        );

        println!("{}", invoice.to_string());
    }
}
