mod cnt_db;
use axum::{*, routing::get, response::Html};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/{name}", get(index))
        .route("/up", get(count_up))
        ;

    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(name: String) -> Html<&'static str>{
    println!("log: {name}");
    Html::from("<h1>hello</h1>")
}

async fn count_up() -> &'static str{
    let n = cnt_db::db::Db::new();
    "ok"
}