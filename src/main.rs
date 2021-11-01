use std::net::{Shutdown, ToSocketAddrs};
use std::net::TcpStream;
use std::time::Duration;
use clap::{Arg, App};
use crate::result::{PortStatus, ScanResult};

mod result;

fn main() {
    let matches = App::new("rscan")
        .version("1.0")
        .author("CodeDead <admin@codedead.com>")
        .about("TCP Network scanning utility")
        .arg(Arg::with_name("threads")
            .short("c")
            .long("threads")
            .value_name("COUNT")
            .help("Sets the number of threads to use")
            .takes_value(true))
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .value_name("HOST")
            .help("Sets the host (or IP address) to scan")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("startport")
            .short("s")
            .long("start")
            .value_name("STARTPORT")
            .help("Sets the initial port that needs to be scanned")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("endport")
            .short("e")
            .long("end")
            .value_name("ENDPORT")
            .help("Sets the last port that needs to be scanned")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("timeout")
            .short("t")
            .long("timeout")
            .value_name("TIMEOUT")
            .help("Sets the connection timeout (in milliseconds) before a port is marked as closed")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("noclosed")
            .short("n")
            .long("noclosed")
            .value_name("TRUE|FALSE")
            .help("Sets whether closed ports should be outputted or not")
            .required(false)
            .takes_value(true))
        .get_matches();

    let host = matches.value_of("host").expect("Host is a required parameter!");
    let _threads: u32 = matches.value_of("threads").unwrap_or("1").parse().expect("Threads is not a valid integer!");
    let mut start_port: u16 = matches.value_of("startport").unwrap_or("0").parse().expect("Start port is not a port number!");
    let mut end_port: u16 = matches.value_of("endport").unwrap_or("0").parse().expect("End port is not a valid port number!");
    let timeout: u64 = matches.value_of("timeout").unwrap_or("500").parse().expect("Timeout is not a valid integer!");
    let no_closed: bool = matches.value_of("noclosed").unwrap_or("false").parse().expect("No closed argument can only be true or false!");

    if start_port > end_port && end_port != 0 {
        panic!("Start port cannot be bigger than end port!");
    } else if end_port == 0 {
        end_port = u16::MAX;
    }

    if start_port == 0 && end_port == 0 {
        start_port = 0;
        end_port = u16::MAX;
    } else if start_port > 0 && end_port == 0 {
        end_port = u16::MAX;
    }

    let mut result = scan_range(host, start_port, end_port, timeout);
    // Sort by port
    result.sort_by(|a, b| a.port.cmp(&b.port));

    for s in result {
        match s.port_status {
            PortStatus::Open => {
                println!("{}:{} | {}", s.host, s.port, "OPEN");
            }
            PortStatus::Closed => {
                if !no_closed {
                    println!("{}:{} | {}", s.host, s.port, "CLOSED");
                }
            }
        }
    }
}

/// Scan a range of ports for a specified host
///
/// # Arguments
///
/// * `host` - The host that needs to be scanned
/// * `start` - The initial port that needs to be scanned
/// * `end` - The final port that needs to be scanned
/// * `timeout` - The connection timeout (in milliseconds) before a port is marked as closed
fn scan_range(host: &str, start: u16, end: u16, timeout: u64) -> Vec<ScanResult> {
    let mut scan_result = vec![];

    for n in start..=end {
        let mut address = format!("{}:{}", host, n).to_socket_addrs().unwrap();
        let socket_address = address.next().unwrap();

        if let Ok(stream) = TcpStream::connect_timeout(&socket_address, Duration::from_millis(timeout)) {
            scan_result.push(ScanResult::new(host, n, PortStatus::Open));
            let res = stream.shutdown(Shutdown::Both);
            match res {
                Ok(_) => {},
                Err(e) => {panic!("Unable to shut down TcpStream: {}", e)}
            }
        } else {
            scan_result.push(ScanResult::new(host,n, PortStatus::Closed));
        }
    }

    scan_result
}
