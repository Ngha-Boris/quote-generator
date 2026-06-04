use sqlx::PgPool;

/// Application shared state
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that AppState can be cloned (compile-time check)
    fn _assert_clone_impl() {
        fn assert_clone<T: Clone>() {}
        assert_clone::<AppState>();
    }

    /// Test that AppState is Send + Sync (required for axum handlers)
    fn _assert_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<AppState>();
        assert_sync::<AppState>();
    }

    #[test]
    fn test_app_state_traits() {
        // This test verifies AppState has required traits at compile time
        // The functions above would fail to compile if traits were missing
        assert!(true);
    }
}
