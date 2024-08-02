mod utils;

#[cfg(test)]
mod tests {
    use rust_invoice_app::traits::invoice_operations::InvoiceOperations;

    use crate::utils::setup;

    #[test]
    fn test_invoice_line_count_total_success() {
        let (cost, quantity) = setup::create_random_cost_quantity();

        let invoice_line = setup::create_invoice_line(cost, quantity);

        assert_eq!(invoice_line.count_total(), cost * (quantity as f64));
    }

    #[test]
    fn test_create_invoice_add_item_success() {
        let mut invoice = setup::setup_invoice();
        let invoice_line = setup::setup_invoice_line();

        invoice.add_line_item(invoice_line);

        assert_eq!(2, invoice.line_items.len());
    }

    #[test]
    fn test_create_invoice_remove_item_success() {
        let mut invoice = setup::setup_invoice();
        let invoice_line = setup::setup_invoice_line();

        invoice.add_line_item(invoice_line.clone());

        assert_eq!(2, invoice.line_items.len());

        let remove_result = invoice.remove_invoice_line(&invoice_line.invoice_line_id);
        assert!(remove_result.is_ok());

        assert_eq!(1, invoice.line_items.len());
    }

    #[test]
    fn test_create_invoice_remove_item_fail() {
        let mut invoice = setup::setup_invoice();
        let invoice_line = setup::setup_invoice_line();

        let remove_result = invoice.remove_invoice_line(&invoice_line.invoice_line_id);
        // cannot find the uuid
        assert!(remove_result.is_err());
    }

    #[test]
    fn test_create_invoice_get_total_success() {
        let (cost, quantity) = setup::create_random_cost_quantity();
        let invoice_line = setup::create_invoice_line(cost, quantity);
        let mut invoice = setup::create_invoice(cost, quantity);

        invoice.add_line_item(invoice_line);

        assert_eq!(2, invoice.line_items.len());

        let target_total_cost = 2.0 * cost * (quantity as f64);

        assert_eq!(target_total_cost, invoice.get_total());
    }
}
