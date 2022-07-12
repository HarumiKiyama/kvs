use std::net::{SocketAddr, IpAddr, Ipv4Addr};

pub static DEFAULT_IP_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4000);

