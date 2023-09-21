use std::net::{Ipv4Addr, SocketAddr};

use tokio::{
    net::{TcpListener, UdpSocket},
    spawn,
};

mod router;

pub async fn run() {


    let addr = SocketAddr::from(([0,0,0,0], 8956));
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();
    let udp_socket = UdpSocket::bind(&addr).await.unwrap();

    let tcp_spawn_handle = spawn(async move {
        router::tcp_process(tcp_listener).await;
    });

    let udp_spawn_handle = spawn(async move {
        router::udp_process(udp_socket).await;
    });

    futures::future::join_all(vec![tcp_spawn_handle, udp_spawn_handle,]).await;
}
