use std::net::SocketAddr;

use tokio::spawn;

use crate::{
    config::{self},
    init::INITED,
};

use self::router::get_route;

mod router;
mod user;

pub async fn run() {
    if !unsafe { INITED } {
        panic!("unrs: not inited")
    };

    let http_handle = spawn(async {
        let config = config::get().await;
        let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
        let router = get_route();
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .unwrap();
    });

    futures::future::join_all([http_handle]).await;
}
