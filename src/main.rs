use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::io::{Read, Write};
use std::thread::spawn;
use std::time::Duration;
use std::env::args;

const PORT: u16 = 7070;
const BUF: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

fn main() {
    let args = args().collect::<Vec<String>>();
    println!("{args:?}");
    let first = &args[1];
    if first == "server" {
        server();
    }
    if first == "client" {
        let addr = &args[2];
        client((addr.clone(), PORT));
    }
}

fn server() {
    let server = TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), PORT)).unwrap();
    
    let mut clients: Vec<TcpStream> = vec![];

    loop {
        // Accept clients
        let incoming = server.incoming();

        // if let Ok((mut socket, addr)) = server.accept() {
        for client in incoming {
            let mut socket = match client {
                Ok(c) => c,
                Err(_) => continue,
            };
            println!("[conn] new connection");

            println!("[server] sending testing payload");
            let result = socket.write(BUF.as_slice());
            println!("[server] send result: {result:?}");

            println!("[server] client functional");
            clients.push(socket);
            // Read
            std::thread::sleep(Duration::from_secs(5));
            for client in clients.iter_mut() {
                println!("[server] attempting recv");
                let mut buf = [0;8];
                let result = client.read(&mut buf);
                if result.is_ok() {
                    println!("[server] > {buf:?}");
                } else {
                    println!("failed")
                }
            }
        }
    }
}

fn client(addr: impl ToSocketAddrs) {
    let mut socket = TcpStream::connect(addr).unwrap();
    std::thread::sleep(Duration::from_secs(2));

    println!(" client] sending");
    match socket.write_all(&BUF) {
        Ok(_) => println!(" client] send successful"),
        Err(_) => println!(" client] send failed"),
    };

    loop {
        let mut buf = vec![];
        let result = socket.read_to_end(&mut buf);
        if result.is_ok() {
            println!("[client] received > {buf:?}");
        }
    }
}
