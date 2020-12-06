use std::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};

use docopt::Docopt;

const USAGE: &'static str = "\
Simle tool for receive UDP datagrams.

Usage: zudp help
       zudp listen <hostport>
       zudp send <hostport> <data>
";

const BUFFER_SIZE: usize = 2048;
const MAX_PRINT_SIZE: usize = 256;
const ALL_INTERFACER: &str = "0:0";


fn listen(addr: &SocketAddr) -> std::io::Result<()> {
    let socket = UdpSocket::bind(addr)?;

    loop {
        let mut buf = [0; BUFFER_SIZE];
        let (size, src) = socket.recv_from(&mut buf)?;
        let print_size = core::cmp::min(size, MAX_PRINT_SIZE);

        match std::str::from_utf8(&buf[..print_size]) {
            Ok(data) => println!("{} {} {:?}", src, size, data),
            Err(_) => println!("{} {}", src, size),
        }
    }
}


fn send(addr: &SocketAddr, data: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind(ALL_INTERFACER)?;
    socket.send_to(data.as_bytes(), &addr)?;
    Ok(())
}


fn main() -> std::io::Result<()> {
    match Docopt::new(USAGE).unwrap().parse() {
        Ok(args) => {
            if args.get_bool("help") {
                println!("{}", USAGE);

            } else if args.get_bool("listen") {
                let addr = args.get_str("<hostport>")
                               .to_socket_addrs()?
                               .next()
                               .unwrap();
                listen(&addr)?;

            } else if args.get_bool("send") {
                let addr = args.get_str("<hostport>")
                               .to_socket_addrs()?
                               .next()
                               .unwrap();
                send(&addr, args.get_str("<data>"))?;

            } else {
                panic!("{:?}", args);
            }
        },
        Err(e) => e.exit(),
    };
    Ok(())
}
