use socket2::{Domain, SockAddr, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};


fn main() {
    let mut socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();

    let s_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1234);
    socket.connect(&SockAddr::from(s_addr)).unwrap();

    let msg = "hello".as_bytes();
    std::io::Write::write(&mut socket, msg).unwrap();

    // read the response
    let mut buffer: [u8; 64] = [0; 64];
    let bytes_read = std::io::Read::read(&mut socket, &mut buffer).unwrap();
    println!("server says {}: {}", bytes_read, String::from_utf8_lossy(&buffer[..bytes_read]));
}