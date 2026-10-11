use socket2::{Socket, Domain, Type, SockAddr};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn do_something(conn: &mut Socket) -> std::io::Result<()> {
    let mut buffer: [u8; 64] = [0; 64];
    let bytes_read = match std::io::Read::read(conn, &mut buffer) {
        Ok(bytes_read) => bytes_read,
        Err(err) => {
            return Err(err);
        }
    };

    if bytes_read == 0 {
        println!("client closed connection");
        return Ok(());
    }

    println!("client says {}: {}", bytes_read, String::from_utf8_lossy(&buffer[..bytes_read]));

    let wbuf = b"world";
    std::io::Write::write(conn, wbuf)?;
    Ok(())
}

fn main() {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
    let _ = socket.set_reuse_address(true).unwrap();

    let s_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 1234);

    // bind the socket to addr
    socket.bind(&SockAddr::from(s_addr)).unwrap();

    // listen for incoming connections
    socket.listen(128).unwrap();

    loop {
        // accept 
        let (mut conn, _) = match socket.accept() {
            Ok(conn) => conn,
            Err(err) => {
                eprintln!("accept() error {}", err);
                continue;
            }
        };

        if let Err(err) = do_something(&mut conn) {
            eprintln!("do_something() error {}", err);
        }

        // close the connection
        drop(conn);
    }
}