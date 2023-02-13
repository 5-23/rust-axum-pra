use std::net::SocketAddr;

use axum::{*, routing::get, routing::post, response::Html};
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(||async {Html::from("<h1>hello</h1>")}));



    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

