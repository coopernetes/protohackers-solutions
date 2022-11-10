use std::net::{TcpListener, TcpStream, SocketAddr};

fn handle_client(stream: TcpStream) {
    println!("Called handle_client")
}


fn main() -> std::io::Result<()> {
    println!("Starting echo.");
    let addrs = [
       SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0x1], 0)),
       SocketAddr::from(([127, 0, 0, 1], 0)),
    ];
    let listener = TcpListener::bind(&addrs[..]).unwrap();

    let sock = listener.local_addr().unwrap(); // SocketAddr
    let port = sock.port();
    let port_s = port.to_string();
    
    println!("Listening on {port_s}");
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
