use crate::service;



pub async fn start() {

    service::run().await;

}
