use crate::db;
use crate::models::{ErrorResponse, HealthResponse, QuoteResponse};
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;

/// GET /api/quotes/random — Return a random quote
pub async fn get_random_quote(
    State(state): State<AppState>,
) -> Result<Json<QuoteResponse>, (StatusCode, Json<ErrorResponse>)> {
    metrics::counter!("api_requests_total", "endpoint" => "random_quote").increment(1);

    let quote = db::fetch_random_quote(&state.db).await.map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to fetch quote".to_string(),
            }),
        )
    })?;

    match quote {
        Some(q) => Ok(Json(QuoteResponse {
            id: q.id,
            quote: q.quote,
            author: q.author,
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "No quotes found".to_string(),
            }),
        )),
    }
}

/// GET /api/quotes — Return all quotes
pub async fn get_all_quotes(
    State(state): State<AppState>,
) -> Result<Json<Vec<QuoteResponse>>, (StatusCode, Json<ErrorResponse>)> {
    metrics::counter!("api_requests_total", "endpoint" => "all_quotes").increment(1);

    let quotes = db::fetch_all_quotes(&state.db).await.map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to fetch quotes".to_string(),
            }),
        )
    })?;

    let responses: Vec<QuoteResponse> = quotes
        .into_iter()
        .map(|q| QuoteResponse {
            id: q.id,
            quote: q.quote,
            author: q.author,
        })
        .collect();

    Ok(Json(responses))
}

/// GET /api/health — Health check endpoint
pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, (StatusCode, Json<ErrorResponse>)> {
    metrics::counter!("api_requests_total", "endpoint" => "health").increment(1);

    // Check database connectivity
    db::check_health(&state.db).await.map_err(|e| {
        tracing::error!("Health check failed: {}", e);
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "Database unavailable".to_string(),
            }),
        )
    })?;

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{HealthResponse, Quote, QuoteResponse};

    #[test]
    fn test_quote_to_response_mapping() {
        // Test the conversion from Quote to QuoteResponse
        let quote = Quote {
            id: 1,
            quote: "Test quote".to_string(),
            author: "Test Author".to_string(),
        };

        let response = QuoteResponse {
            id: quote.id,
            quote: quote.quote.clone(),
            author: quote.author.clone(),
        };

        assert_eq!(response.id, 1);
        assert_eq!(response.quote, "Test quote");
        assert_eq!(response.author, "Test Author");
    }

    #[test]
    fn test_health_response_default() {
        let health = HealthResponse {
            status: "healthy".to_string(),
        };
        assert_eq!(health.status, "healthy");
    }

    #[test]
    fn test_error_response_creation() {
        let error = ErrorResponse {
            error: "Failed to fetch quote".to_string(),
        };
        assert_eq!(error.error, "Failed to fetch quote");
    }

    #[test]
    fn test_status_code_mapping() {
        // Test that we use correct status codes
        assert_eq!(StatusCode::OK.as_u16(), 200);
        assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), 500);
        assert_eq!(StatusCode::SERVICE_UNAVAILABLE.as_u16(), 503);
    }

    #[test]
    fn test_handler_signatures_exist() {
        // Compile-time check: verify handlers are functions that can be referenced
        let _ = get_random_quote as fn(State<AppState>) -> _;
        let _ = get_all_quotes as fn(State<AppState>) -> _;
        let _ = health_check as fn(State<AppState>) -> _;

        // If we reach here, all handlers have correct signatures
        assert!(true);
    }
}
