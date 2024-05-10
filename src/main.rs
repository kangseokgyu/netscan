use axum::{response::Html, routing::get, Router};
use ip_mon::parser::Device;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let task1 = async move {
        web().await;
    };

    let task2 = async move {
        parse().await;
    };

    // Task 1과 Task 2를 동시에 실행
    tokio::join!(task1, task2);
}

async fn web() {
    // build our application with a single route
    let app = Router::new().route("/", get(handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(std::include_str!("index.html"))
}

async fn parse() {
    loop {
        let devices = Device::parse(std::fs::read_to_string("nmap.data").unwrap().as_str());
        println!("{}", serde_json::to_string_pretty(&devices).unwrap());

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
