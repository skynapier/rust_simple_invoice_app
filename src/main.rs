use rust_invoice_app::{
    services::accounting_services::AccountingServices,
    traits::accounting_operations::AccoungtingOperations,
};

fn main() {
    let accounting_services = AccountingServices::default();

    println!(
        "running: {}",
        stringify!(accounting_services.create_invoice_with_one_item())
    );
    accounting_services.create_invoice_with_one_item();

    println!(
        " running: {}",
        stringify!(accounting_services.create_invoice_with_multiple_items_and_quantities())
    );
    accounting_services.create_invoice_with_multiple_items_and_quantities();

    println!("running: {}", stringify!(accounting_services.remove_item()));
    accounting_services.remove_item();

    println!(
        "running: {}",
        stringify!(accounting_services.merge_invoices())
    );
    accounting_services.merge_invoices();

    println!(
        "running: {}",
        stringify!(accounting_services.clone_invoice())
    );
    accounting_services.clone_invoice();

    println!(
        "running: {}",
        stringify!(accounting_services.invoice_to_string())
    );
    accounting_services.invoice_to_string();
}
