use data_seeker::app;
use data_seeker::CONFIGS;
use dotenvy::dotenv;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    let settings = CONFIGS.read().unwrap();
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], settings.server.data_seeker.port));

    dotenv().ok();
    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    let db = Database::connect(database_url).await;
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}
