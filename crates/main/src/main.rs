

mod service;
mod process;
#[tokio::main]
async fn main() {
    // let ips = root::net::get_intranet_ipv4("192.168.0", 3000, None).await;
    // println!("{:?}", ips)
    service::run().await;
}
