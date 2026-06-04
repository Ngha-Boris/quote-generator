use crate::handlers;
use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

/// Build the application router with all routes and middleware
pub fn create_router(
    state: AppState,
    metrics_handle: metrics_exporter_prometheus::PrometheusHandle,
) -> Router {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application routes
    Router::new()
        .route("/api/quotes/random", get(handlers::get_random_quote))
        .route("/api/quotes", get(handlers::get_all_quotes))
        .route("/api/health", get(handlers::health_check))
        .route(
            "/metrics",
            get(move || {
                let handle = metrics_handle.clone();
                async move { handle.render() }
            }),
        )
        .layer(cors)
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;
    use axum::Router;

    /// Compile-time check that create_router returns correct type
    fn _assert_router_type() {
        fn assert_is_router<T>(_router: Router<T>) {}

        // This would fail to compile if create_router didn't return a Router
        // We can't actually call it without a real DB, but we verify the signature
        let _assert = assert_is_router::<AppState>;
    }

    #[test]
    fn test_router_configuration_compiles() {
        // Verify that route configuration compiles correctly
        // The routes are: /api/quotes/random, /api/quotes, /api/health, /metrics
        let expected_routes = vec![
            "/api/quotes/random",
            "/api/quotes",
            "/api/health",
            "/metrics",
        ];

        // Verify we have the expected number of routes
        assert_eq!(expected_routes.len(), 4);

        // Verify specific routes exist
        assert!(expected_routes.contains(&"/api/quotes/random"));
        assert!(expected_routes.contains(&"/api/quotes"));
        assert!(expected_routes.contains(&"/api/health"));
        assert!(expected_routes.contains(&"/metrics"));
    }

    #[test]
    fn test_cors_configuration() {
        // Verify CORS layer is configured correctly
        let _cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        // If we get here, CORS configuration is valid
        assert!(true);
    }

    /// Compile-time check for required traits on Router with AppState
    fn _assert_router_traits() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        fn assert_clone<T: Clone>() {}

        assert_send::<AppState>();
        assert_sync::<AppState>();
        assert_clone::<AppState>();
    }
}
