use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use metrics_exporter_prometheus::PrometheusBuilder;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Quote {
    id: i32,
    quote: String,
    author: String,
}

#[derive(Debug, Serialize)]
struct QuoteResponse {
    id: i32,
    quote: String,
    author: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

/// Application shared state
#[derive(Clone)]
struct AppState {
    db: PgPool,
}

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
    let handle = builder
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
    run_migrations(&pool).await;

    // Seed default quotes if empty
    seed_quotes(&pool).await;

    let state = AppState { db: pool };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application routes
    let app = Router::new()
        .route("/api/quotes/random", get(get_random_quote))
        .route("/api/quotes", get(get_all_quotes))
        .route("/api/health", get(health_check))
        .route(
            "/metrics",
            get(move || {
                let handle = handle.clone();
                async move { handle.render() }
            }),
        )
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Backend listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Run database migrations (create quotes table)
async fn run_migrations(pool: &PgPool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS quotes (
            id SERIAL PRIMARY KEY,
            quote TEXT NOT NULL,
            author TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to run migrations");

    tracing::info!("Database migrations complete");
}

/// Seed the database with motivational quotes if the table is empty
async fn seed_quotes(pool: &PgPool) {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM quotes")
        .fetch_one(pool)
        .await
        .expect("Failed to count quotes");

    if count.0 > 0 {
        tracing::info!("Database already seeded with {} quotes", count.0);
        return;
    }

    let quotes = vec![
        ("The only way to do great work is to love what you do.", "Steve Jobs"),
        ("Discipline beats motivation.", "Unknown"),
        ("Success is not final, failure is not fatal: it is the courage to continue that counts.", "Winston Churchill"),
        ("Believe you can and you're halfway there.", "Theodore Roosevelt"),
        ("It does not matter how slowly you go as long as you do not stop.", "Confucius"),
        ("The future belongs to those who believe in the beauty of their dreams.", "Eleanor Roosevelt"),
        ("In the middle of every difficulty lies opportunity.", "Albert Einstein"),
        ("Strive not to be a success, but rather to be of value.", "Albert Einstein"),
        ("The best time to plant a tree was 20 years ago. The second best time is now.", "Chinese Proverb"),
        ("Your limitation—it's only your imagination.", "Unknown"),
        ("Push yourself, because no one else is going to do it for you.", "Unknown"),
        ("Great things never come from comfort zones.", "Unknown"),
        ("Dream it. Wish it. Do it.", "Unknown"),
        ("Success doesn't just find you. You have to go out and get it.", "Unknown"),
        ("The harder you work for something, the greater you'll feel when you achieve it.", "Unknown"),
        ("Dream bigger. Do bigger.", "Unknown"),
        ("Don't stop when you're tired. Stop when you're done.", "Unknown"),
        ("Wake up with determination. Go to bed with satisfaction.", "Unknown"),
        ("Do something today that your future self will thank you for.", "Sean Patrick Flanery"),
        ("Little things make big days.", "Unknown"),
        ("It's going to be hard, but hard does not mean impossible.", "Unknown"),
        ("Don't wait for opportunity. Create it.", "Unknown"),
        ("Sometimes we're tested not to show our weaknesses, but to discover our strengths.", "Unknown"),
        ("The key to success is to focus on goals, not obstacles.", "Unknown"),
        ("Dream it. Believe it. Build it.", "Unknown"),
    ];

    for (text, author) in &quotes {
        sqlx::query("INSERT INTO quotes (quote, author) VALUES ($1, $2)")
            .bind(text)
            .bind(author)
            .execute(pool)
            .await
            .expect("Failed to seed quote");
    }

    tracing::info!("Seeded {} quotes into the database", quotes.len());
}

/// GET /api/quotes/random — Return a random quote
async fn get_random_quote(
    State(state): State<AppState>,
) -> Result<Json<QuoteResponse>, (StatusCode, Json<ErrorResponse>)> {
    metrics::counter!("api_requests_total", "endpoint" => "random_quote").increment(1);

    let quote = sqlx::query_as::<_, Quote>(
        "SELECT id, quote, author FROM quotes ORDER BY RANDOM() LIMIT 1",
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
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
async fn get_all_quotes(
    State(state): State<AppState>,
) -> Result<Json<Vec<QuoteResponse>>, (StatusCode, Json<ErrorResponse>)> {
    metrics::counter!("api_requests_total", "endpoint" => "all_quotes").increment(1);

    let quotes = sqlx::query_as::<_, Quote>("SELECT id, quote, author FROM quotes ORDER BY id")
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
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
async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, (StatusCode, Json<ErrorResponse>)> {
    metrics::counter!("api_requests_total", "endpoint" => "health").increment(1);

    // Check database connectivity
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .map_err(|e| {
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
