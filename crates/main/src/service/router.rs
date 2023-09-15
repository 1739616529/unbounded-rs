use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, UdpSocket},
};

pub async fn tcp_process(listener: TcpListener) {
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buf = [0; 4096];

        stream.read(&mut buf).await.unwrap();
        stream.write_all("HTTP/1.1 200 OK \r\n\r\n".as_bytes()).await.unwrap();
    }
}

pub async fn udp_process(socket: UdpSocket) {
    loop {
        let mut buf = [0; 10];
        socket.recv_from(&mut buf).await.unwrap();
        let str = std::str::from_utf8(&buf).unwrap();
        println!("udp: {:#?}", str);
    }
}
