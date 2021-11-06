use std::time::Duration;
use std::net::{Shutdown, ToSocketAddrs};
use std::net::TcpStream;
use std::thread;
use std::sync::{Arc, Mutex};
use crate::result::{PortStatus, ScanResult};

mod result;
mod config;

fn main() {
    let optional_config = crate::config::Config::read_from_args();
    let config = match optional_config {
        Ok(d) => d,
        Err(e) => {
            panic!("{}", e.message);
        }
    };

    let all_results: Arc<Mutex<Vec<ScanResult>>> = Arc::new(Mutex::new(vec![]));

    let mut threads = config.threads;

    if threads > 1 {
        let total_ports = u32::from(config.end_port) - u32::from(config.start_port) + 1;

        if threads > total_ports {
            threads = total_ports;
        }

        let range = (total_ports / threads) as u16;
        let remainder = (total_ports % threads) as u16;

        let mut current_start = config.start_port;
        let mut current_end = range - 1;

        let mut handles = vec![];
        for n in 0..threads {
            let local_start = current_start;
            let local_end = current_end;
            let local_host = config.host.clone();

            let all_results = Arc::clone(&all_results);
            let handle = thread::spawn(move || {
                let res = scan_range(&local_host, local_start, local_end, config.timeout, config.interactive, config.no_closed);

                let mut results = all_results.lock().unwrap();
                for l in res {
                    results.push(l);
                }
            });
            handles.push(handle);

            if current_end != u16::MAX {
                current_start = current_end + 1;
            }

            match current_end.checked_add(range) {
                Some(v) => { current_end = v; }
                None => { current_end = u16::MAX; }
            };

            if remainder > 0 && n == threads - 2 {
                match current_end.checked_add(remainder) {
                    None => { current_end = u16::MAX; }
                    Some(v) => { current_end = v; }
                }
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }
    } else {
        let res = scan_range(&config.host, config.start_port, config.end_port, config.timeout, config.interactive, config.no_closed);
        let mut all = all_results.lock().unwrap();
        for l in res {
            all.push(l);
        }
    }

    let mut res = all_results.lock().unwrap();

    if config.sort {
        // Sort by port number
        res.sort_by(|a, b| a.port.cmp(&b.port));
    }

    if !config.interactive {
        for s in res.iter() {
            to_display(&s);
        }
    }
}

/// Print a `ScanResult` to the stdout
///
/// # Arguments
///
/// * `s` - The reference to the `ScanResult` struct
/// * `no_closed` - Boolean that indicates whether closed ports should be printed out or not
fn to_display(s: &ScanResult) {
    match s.port_status {
        PortStatus::Open => {
            println!("{}:{} | {}", s.host, s.port, "OPEN");
        }
        PortStatus::Closed => {
            println!("{}:{} | {}", s.host, s.port, "CLOSED");
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
/// * `interactive` - Sets whether the output should be displayed while scanning or not
/// * `no_closed` - Sets whether closed ports should be added to the return list or not
fn scan_range(host: &str, start: u16, end: u16, timeout: u64, interactive: bool, no_closed: bool) -> Vec<ScanResult> {
    let mut scan_result = vec![];

    for n in start..=end {
        let mut address = format!("{}:{}", host, n).to_socket_addrs().unwrap();
        let socket_address = address.next().unwrap();

        if let Ok(stream) = TcpStream::connect_timeout(&socket_address, Duration::from_millis(timeout)) {
            let sr = ScanResult::new(host, n, PortStatus::Open);
            if interactive {
                to_display(&sr);
            }
            scan_result.push(sr);

            let res = stream.shutdown(Shutdown::Both);
            match res {
                Ok(_) => {}
                Err(e) => { panic!("Unable to shut down TcpStream: {}", e) }
            }
        } else if !no_closed {
            let sr = ScanResult::new(host, n, PortStatus::Closed);
            if interactive {
                to_display(&sr)
            }
            scan_result.push(sr);
        }
    }

    scan_result
}
