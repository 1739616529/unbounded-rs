use axum::{Router, routing, response::Html};


pub fn get_user_router() -> Router {
    let router = Router::new().route("/", routing::get(home));
    Router::new().nest("/user", router)
}

async fn home() -> Html<&'static str> {
    Html("<h1>hello world</h1>")
}
