use axum::{response::Html, routing::get, Router};
pub async fn index() -> Html<&'static str> {
    let html_content = include_str!("../index.html");
    Html(html_content)
}
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .nest("/video", axum_static::static_router("video"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // 나머지 서버 설정
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
