use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow, PartialEq)]
pub struct Quote {
    pub id: i32,
    pub quote: String,
    pub author: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct QuoteResponse {
    pub id: i32,
    pub quote: String,
    pub author: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ErrorResponse {
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_creation() {
        let quote = Quote {
            id: 1,
            quote: "Test quote".to_string(),
            author: "Test Author".to_string(),
        };
        assert_eq!(quote.id, 1);
        assert_eq!(quote.quote, "Test quote");
        assert_eq!(quote.author, "Test Author");
    }

    #[test]
    fn test_quote_response_creation() {
        let response = QuoteResponse {
            id: 2,
            quote: "Response quote".to_string(),
            author: "Response Author".to_string(),
        };
        assert_eq!(response.id, 2);
        assert_eq!(response.quote, "Response quote");
        assert_eq!(response.author, "Response Author");
    }

    #[test]
    fn test_health_response_creation() {
        let health = HealthResponse {
            status: "healthy".to_string(),
        };
        assert_eq!(health.status, "healthy");
    }

    #[test]
    fn test_error_response_creation() {
        let error = ErrorResponse {
            error: "Something went wrong".to_string(),
        };
        assert_eq!(error.error, "Something went wrong");
    }

    #[test]
    fn test_quote_serialization() {
        let quote = Quote {
            id: 1,
            quote: "Hello World".to_string(),
            author: "Test".to_string(),
        };
        let json = serde_json::to_string(&quote).unwrap();
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"quote\":\"Hello World\""));
        assert!(json.contains("\"author\":\"Test\""));
    }

    #[test]
    fn test_quote_equality() {
        let quote1 = Quote {
            id: 1,
            quote: "Same".to_string(),
            author: "Author".to_string(),
        };
        let quote2 = Quote {
            id: 1,
            quote: "Same".to_string(),
            author: "Author".to_string(),
        };
        let quote3 = Quote {
            id: 2,
            quote: "Different".to_string(),
            author: "Author".to_string(),
        };
        assert_eq!(quote1, quote2);
        assert_ne!(quote1, quote3);
    }
}
