use crate::config;

// 是否已初始化
pub static mut INITED: bool = false;

pub async fn init() {
    futures::future::join_all([config_init()]).await;
    unsafe {
        INITED = true;
    }
}

async fn config_init() {
    let config = config::Config { port: 8956 };
    config::init(config).await;
}


