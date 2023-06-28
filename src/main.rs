use data_seeker::app;
use data_seeker::CONFIGS;

#[tokio::main]
async fn main() {
    let settings = CONFIGS.read().unwrap();
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], settings.server.data_seeker.port));

    axum::Server::bind(&addr)
        .serve(app().await.into_make_service())
        .await
        .unwrap();
}
