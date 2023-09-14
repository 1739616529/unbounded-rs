use std::time::Duration;

use tokio::{net::TcpStream, time};

pub async fn get_intranet_ipv4(route: &str, port: usize, delay: Option<u64> ) -> Vec<String> {
    let _delay: u64 = match delay {
        Some(val) => val,
        None => 500
    };
    let mut online_ip: Vec<String> = Vec::new();
    let mut handles = Vec::with_capacity(255);
    for i in 1..256 {
        let ip = format!("{}.{}", route, i);
        let addr = format!("{}:{}", ip, port);
        let ret = time::timeout(Duration::from_millis(_delay), TcpStream::connect(addr));
        handles.push(ret);
    }

    let ret = futures::future::join_all(handles).await;

    for (i, v) in ret.iter().enumerate() {
        if let Ok(_) = v {
            let ip = format!("{}.{}", route, i + 1);
            online_ip.push(ip)
        }
    }

    online_ip
}
