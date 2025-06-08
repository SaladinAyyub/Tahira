use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};

mod db;
mod handlers;

mod models;

#[tokio::main]
async fn main() {
    // tracing
    tracing_subscriber::fmt::init();

    // Add Cors
    let cors = CorsLayer::new().allow_origin(Any);

    //add Postgres
    dotenv::dotenv().ok();
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool: Pool<Postgres> = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Error making pool connection");

    db::setup(&pool).await;

    let app: axum::Router = Router::new()
        .route("/tahira/api", get(root))
        .route(
            "/tahira/api/places",
            get(handlers::get_places).post(handlers::add_place),
        )
        .route(
            "/tahira/api/localities",
            get(handlers::get_localities).post(handlers::add_locality),
        )
        .route("/tahira/api/places/:id", get(handlers::get_place_from_id))
        .route(
            "/tahira/api/places/:locality_id",
            get(handlers::get_places_from_locality_id),
        )
        .layer(cors)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    String::from("Assalamu Alaykum")
}
