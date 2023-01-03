use data_seeker::app;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}
