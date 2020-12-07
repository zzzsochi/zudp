use std::cmp;
use std::io;
use std::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};
use std::str;

use clap::{App, AppSettings, Arg};

const BUFFER_SIZE: usize = 2048;
const MAX_PRINT_SIZE: usize = 256;
const ALL_INTERFACER: &str = "0:0";

fn listen(addr: &SocketAddr) -> io::Result<()> {
    let socket = UdpSocket::bind(addr)?;

    loop {
        let mut buf = [0; BUFFER_SIZE];
        let (size, src) = socket.recv_from(&mut buf)?;
        let print_size = cmp::min(size, MAX_PRINT_SIZE);

        match str::from_utf8(&buf[..print_size]) {
            Ok(data) => println!("{} {} {:?}", src, size, data),
            Err(_) => println!("{} {}", src, size),
        }
    }
}

fn send(addr: &SocketAddr, data: &str) -> io::Result<()> {
    let socket = UdpSocket::bind(ALL_INTERFACER)?;
    socket.send_to(data.as_bytes(), &addr)?;
    Ok(())
}

fn get_addr(value: Option<&str>) -> io::Result<SocketAddr> {
    Ok(value.unwrap().to_socket_addrs()?.next().unwrap())
}

fn main() -> io::Result<()> {
    let matches = App::new("zudp")
        .version("0.1.0")
        .author("Alexander Zelenyak <zzz.sochi@gmail.com>")
        .about("Simple tool for receive UDP datagrams.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(App::new("listen").arg(Arg::new("hostport").takes_value(true)))
        .subcommand(
            App::new("send")
                .arg(Arg::new("hostport").takes_value(true))
                .arg(Arg::new("data").takes_value(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("listen", sub_matches)) => {
            let addr = get_addr(sub_matches.value_of("hostport"))?;
            listen(&addr)?
        }
        Some(("send", sub_matches)) => {
            let addr = get_addr(sub_matches.value_of("hostport"))?;
            let data = sub_matches.value_of("data").unwrap();
            send(&addr, data)?
        }
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }
    Ok(())
}
