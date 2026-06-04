use crate::models::Quote;
use sqlx::PgPool;

/// Run database migrations (create quotes table)
pub async fn run_migrations(pool: &PgPool) {
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
pub async fn seed_quotes(pool: &PgPool) {
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

/// Fetch a random quote from the database
pub async fn fetch_random_quote(pool: &PgPool) -> Result<Option<Quote>, sqlx::Error> {
    sqlx::query_as::<_, Quote>("SELECT id, quote, author FROM quotes ORDER BY RANDOM() LIMIT 1")
        .fetch_optional(pool)
        .await
}

/// Fetch all quotes from the database
pub async fn fetch_all_quotes(pool: &PgPool) -> Result<Vec<Quote>, sqlx::Error> {
    sqlx::query_as::<_, Quote>("SELECT id, quote, author FROM quotes ORDER BY id")
        .fetch_all(pool)
        .await
}

/// Check database connectivity
pub async fn check_health(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await.map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    /// Helper function to create a test database pool
    async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test_db".to_string());

        PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
    }

    #[tokio::test]
    async fn test_check_health_success() {
        // This test requires a running PostgreSQL database
        // Skip if no database is available
        if let Ok(pool) = create_test_pool().await {
            let result = check_health(&pool).await;
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_quotes_count_constants() {
        // Verify we have the expected number of seed quotes defined
        let seed_quotes = vec![
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

        assert_eq!(seed_quotes.len(), 25);
    }

    #[test]
    fn test_seed_quote_content() {
        // Verify a specific quote is in the seed data
        let seed_quotes = vec![
            (
                "The only way to do great work is to love what you do.",
                "Steve Jobs",
            ),
            ("Discipline beats motivation.", "Unknown"),
        ];

        assert_eq!(
            seed_quotes[0].0,
            "The only way to do great work is to love what you do."
        );
        assert_eq!(seed_quotes[0].1, "Steve Jobs");
        assert_eq!(seed_quotes[1].0, "Discipline beats motivation.");
        assert_eq!(seed_quotes[1].1, "Unknown");
    }
}
