use axum::{routing::post, Router};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth_guard;
mod db;
mod routes;

/// Shared database handle passed via axum State.
pub type Db = Arc<Mutex<rusqlite::Connection>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            ),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_path =
        std::env::var("DATABASE_PATH").unwrap_or_else(|_| "/data/lumisync.db".to_string());

    tracing::info!("Opening database at {}", db_path);
    let conn = db::initialize_db(&db_path).expect("Failed to initialize database");
    let db: Db = Arc::new(Mutex::new(conn));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Health check
        .route("/health", axum::routing::get(|| async { "OK" }))
        // ── Auth ──────────────────────────────────────────────────────────────
        .route("/api/login",            post(routes::auth::login))
        .route("/api/logout",           post(routes::auth::logout))
        .route("/api/get-current-user", post(routes::auth::get_current_user))
        // ── Users ─────────────────────────────────────────────────────────────
        .route("/api/get-users",       post(routes::users::get_users))
        .route("/api/create-user",     post(routes::users::create_user))
        .route("/api/update-user",     post(routes::users::update_user))
        .route("/api/delete-user",     post(routes::users::delete_user))
        .route("/api/change-password", post(routes::users::change_password))
        // ── Categories ────────────────────────────────────────────────────────
        .route("/api/get-categories",   post(routes::inventory::get_categories))
        .route("/api/create-category",  post(routes::inventory::create_category))
        .route("/api/update-category",  post(routes::inventory::update_category))
        // ── Products ──────────────────────────────────────────────────────────
        .route("/api/get-products",         post(routes::inventory::get_products))
        .route("/api/get-product",          post(routes::inventory::get_product))
        .route("/api/create-product",       post(routes::inventory::create_product))
        .route("/api/update-product",       post(routes::inventory::update_product))
        .route("/api/delete-product",       post(routes::inventory::delete_product))
        .route("/api/adjust-stock",         post(routes::inventory::adjust_stock))
        .route("/api/get-stock-adjustments",post(routes::inventory::get_stock_adjustments))
        // ── Sales ─────────────────────────────────────────────────────────────
        .route("/api/create-sale", post(routes::sales::create_sale))
        .route("/api/get-sale",    post(routes::sales::get_sale))
        .route("/api/get-sales",   post(routes::sales::get_sales))
        .route("/api/void-sale",   post(routes::sales::void_sale))
        // ── Invoices ──────────────────────────────────────────────────────────
        .route("/api/get-invoices", post(routes::invoices::get_invoices))
        .route("/api/get-invoice",  post(routes::invoices::get_invoice))
        // ── Customers ─────────────────────────────────────────────────────────
        .route("/api/get-customers",    post(routes::customers::get_customers))
        .route("/api/create-customer",  post(routes::customers::create_customer))
        .route("/api/update-customer",  post(routes::customers::update_customer))
        .route("/api/delete-customer",  post(routes::customers::delete_customer))
        // ── Suppliers ─────────────────────────────────────────────────────────
        .route("/api/get-suppliers",   post(routes::suppliers::get_suppliers))
        .route("/api/create-supplier", post(routes::suppliers::create_supplier))
        .route("/api/update-supplier", post(routes::suppliers::update_supplier))
        .route("/api/delete-supplier", post(routes::suppliers::delete_supplier))
        // ── Reports ───────────────────────────────────────────────────────────
        .route("/api/get-dashboard-stats",   post(routes::reports::get_dashboard_stats))
        .route("/api/get-sales-report",      post(routes::reports::get_sales_report))
        .route("/api/get-inventory-report",  post(routes::reports::get_inventory_report))
        .route("/api/get-profit-loss-report",post(routes::reports::get_profit_loss_report))
        .route("/api/get-vat-report",        post(routes::reports::get_vat_report))
        // ── Settings ──────────────────────────────────────────────────────────
        .route("/api/get-settings",    post(routes::settings::get_settings))
        .route("/api/update-settings", post(routes::settings::update_settings))
        .layer(cors)
        .with_state(db);

    tracing::info!("LumiSync API server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
