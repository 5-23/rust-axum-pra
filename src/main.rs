use axum::{*, routing::{get, post}, response::Html, http::uri::PathAndQuery, extract::Path};
use serde::{Serialize};
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
        .route("/", get(index))
        .route("/json", get(json))
        .route("/hello/:name", get(hello))
        ;

    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str>{
    Html::from("<h1>hello</h1>")
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

async fn hello(Path(name): Path<String>) -> Html<String>{
    println!("{name:?}");
    Html::from(format!("hello <b>{name:?}</b>!"))
}