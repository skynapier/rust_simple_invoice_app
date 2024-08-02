pub trait AccoungtingOperations {
    fn create_invoice_with_one_item(&self);
    fn create_invoice_with_multiple_items_and_quantities(&self);
    fn remove_item(&self);
    fn merge_invoices(&self);
    fn clone_invoice(&self);
    fn invoice_to_string(&self);
}
