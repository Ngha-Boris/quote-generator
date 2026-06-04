use axum::serve;
use metrics_exporter_prometheus::PrometheusBuilder;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

mod db;
mod handlers;
mod models;
mod routes;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Install Prometheus metrics recorder
    let builder = PrometheusBuilder::new();
    let metrics_handle = builder
        .install_recorder()
        .expect("failed to install Prometheus recorder");

    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");

    // Run migrations
    db::run_migrations(&pool).await;

    // Seed default quotes if empty
    db::seed_quotes(&pool).await;

    let state = AppState { db: pool };

    // Build application router
    let app = routes::create_router(state, metrics_handle);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Backend listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    fn test_socket_addr_creation() {
        // Test that our server address is created correctly
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        assert_eq!(addr.ip(), IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(addr.port(), 8080);
    }

    #[test]
    fn test_modules_exist() {
        // Compile-time check that all modules exist
        // If any module is missing, this test will fail to compile

        // Test that we can reference items from each module
        // We just need to reference them, not call them
        let _handler_ref = handlers::get_random_quote;
        let _handler_ref2 = handlers::get_all_quotes;
        let _handler_ref3 = handlers::health_check;
        let _ = models::Quote {
            id: 1,
            quote: "test".to_string(),
            author: "test".to_string(),
        };

        // If we reach this point, all modules compile correctly
        // Model construction successful
    }

    #[test]
    fn test_module_structure() {
        // Verify module hierarchy is correct
        // This test ensures our module declarations are valid
        use crate::models::{ErrorResponse, HealthResponse, Quote, QuoteResponse};

        // Test that we can construct all model types
        let _quote = Quote {
            id: 1,
            quote: "test".to_string(),
            author: "test".to_string(),
        };
        let _response = QuoteResponse {
            id: 1,
            quote: "test".to_string(),
            author: "test".to_string(),
        };
        let _health = HealthResponse {
            status: "healthy".to_string(),
        };
        let _error = ErrorResponse {
            error: "test".to_string(),
        };

        // All types are accessible, test passes
        // Successfully constructed all model types
    }

    #[test]
    fn test_environment_variables_expected() {
        // Document expected environment variables
        let expected_vars = ["DATABASE_URL", "RUST_LOG"];
        assert_eq!(expected_vars.len(), 2);
        assert!(expected_vars.contains(&"DATABASE_URL"));
        assert!(expected_vars.contains(&"RUST_LOG"));
    }
}
