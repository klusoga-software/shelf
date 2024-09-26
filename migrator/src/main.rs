use sqlx::postgres::PgPoolOptions;
use sqlx::sqlx_macros::migrate;

#[tokio::main]
async fn main() {
    println!("Starting sql migrations");
    
    let connection_url = std::env::var("DATABASE_URL").unwrap_or("postgres://postgres:password@localhost/postgres".to_string());
    
    let pool = PgPoolOptions::new().connect(connection_url.as_str()).await.expect("Failed to connect to database");
    
    migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");
}
