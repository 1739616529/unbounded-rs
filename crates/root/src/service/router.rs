use std::{env, path::PathBuf};

use axum::{self, Router};
use tower_http::services::ServeDir;

use super::user::get_user_router;

pub fn get_route() -> Router {
    let merge_route = Router::new().merge(get_user_router());
    let static_path = get_static_dir();

    println!("{:?}", static_path.display());

    let static_server = ServeDir::new(static_path).append_index_html_on_directories(true);
    let app = Router::new()
        .nest("/api", merge_route)
        .nest_service("/", static_server);
    return app;
}

#[cfg(debug_assertions)]
fn get_static_dir() -> PathBuf {
    let mut path = env::current_dir().unwrap();

    path.push("crates/web/dist");
    path
}

#[cfg(not(debug_assertions))]
fn get_static_dir() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    path.pop();
    path.push("static");
    path
}
