mod cnt_db;
use axum::{*, routing::{get, post}, response::Html};
use serde::{Deserialize, Serialize};
#[derive(Serialize)]
struct Js{
    name: &'static str,
    info: Info
}
#[derive(Serialize)]
struct Info{
    mail: &'static str,
    tell: (u16, u16, u16),
    star: usize
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/{name}", get(index))
        .route("/up", get(count_up))
        .route("/info", get(json))
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
async fn json() -> (http::StatusCode, Json<Js>){
    let j = Json::from(Js{
        name: "5-23",
        info: Info {
            mail: "yhanbyeol6@gmail.com",
            tell: (010, 1234, 5678),
            star: 0
        }
    });
    (http::StatusCode::CREATED, j)
}