// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;

use std::sync::Mutex;

pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let db_path = db::get_db_path(app.handle());
            let conn = db::initialize_db(&db_path)
                .expect("Failed to initialize database");
            app.manage(AppState { db: Mutex::new(conn) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Auth
            commands::auth::login,
            commands::auth::logout,
            commands::auth::get_current_user,
            // Users
            commands::users::get_users,
            commands::users::create_user,
            commands::users::update_user,
            commands::users::delete_user,
            commands::users::change_password,
            // Inventory - Categories
            commands::inventory::get_categories,
            commands::inventory::create_category,
            commands::inventory::update_category,
            // Inventory - Products
            commands::inventory::get_products,
            commands::inventory::get_product,
            commands::inventory::create_product,
            commands::inventory::update_product,
            commands::inventory::delete_product,
            commands::inventory::adjust_stock,
            commands::inventory::get_stock_adjustments,
            // Sales
            commands::sales::create_sale,
            commands::sales::get_sale,
            commands::sales::get_sales,
            commands::sales::void_sale,
            // Invoices
            commands::invoices::get_invoices,
            commands::invoices::get_invoice,
            // Customers
            commands::customers::get_customers,
            commands::customers::create_customer,
            commands::customers::update_customer,
            commands::customers::delete_customer,
            // Suppliers
            commands::suppliers::get_suppliers,
            commands::suppliers::create_supplier,
            commands::suppliers::update_supplier,
            commands::suppliers::delete_supplier,
            // Reports
            commands::reports::get_dashboard_stats,
            commands::reports::get_sales_report,
            commands::reports::get_inventory_report,
            commands::reports::get_profit_loss_report,
            commands::reports::get_vat_report,
            // Settings
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
