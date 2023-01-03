//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::net::{SocketAddr, TcpListener};
use tower::Service; // for `call`
use tower::ServiceExt; // for `oneshot` and `ready`

#[tokio::test]
async fn health_check_works() {
    let app = data_seeker::app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    // Assert
    assert!(response.status().is_success());
    // assert!(response.body().is_empty());
}

#[tokio::test]
async fn real_deal_health_check_works() {
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(data_seeker::app().into_make_service())
            .await
            .unwrap();
    });

    let client = hyper::Client::new();
    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{}/health_check", addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(response.status().is_success());
}
