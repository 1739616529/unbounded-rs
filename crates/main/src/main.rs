
use root;
mod process;
#[tokio::main]
async fn main() {
    root::init().await;
}
