use std::io::prelude::*;
use std::io::{BufReader, stderr};
use std::net::{TcpListener, TcpStream, SocketAddr};

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    println!("Called handle_client");
    println!("{:?}", stream);
    let mut buffer: Vec<u8> = vec![];
    let mut buf_reader = BufReader::new(&mut stream);
    buf_reader.read_to_end(&mut buffer)?;
    println!("content: {:?}", buffer);
    stream.write_all(buffer.as_slice())?;
    Ok(())
}


fn main() -> std::io::Result<()> {
    println!("Starting echo.");
    let addrs = [
        SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0x0], 3000)),
        SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0x0], 0)),
        SocketAddr::from(([0, 0, 0, 0], 3000)),
        SocketAddr::from(([0, 0, 0, 0], 0)),
    ];
    let listener = TcpListener::bind(&addrs[..]).unwrap();

    let sock = listener.local_addr().unwrap(); // SocketAddr
    let port = sock.port();
    let port_s = port.to_string();
    
    println!("Listening on {port_s}");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }
            Err(e) => { stderr().write(format!("Connection failed: {e}").as_bytes())?; }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert!(true)
    }
}