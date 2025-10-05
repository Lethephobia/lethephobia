use axum::{Router, routing::get};
use tokio::net::TcpListener;

use internal_api::states::AppConfig;
use internal_api::states::AppState;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(AppState {
            app_config: AppConfig,
        });

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
