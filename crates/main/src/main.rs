
use root;
#[tokio::main]
async fn main() {
    root::init::init().await;
    root::start::start().await;
}
