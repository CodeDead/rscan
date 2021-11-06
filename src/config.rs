use clap::{App, Arg};

pub struct Config {
    pub host: Option<String>,
    pub start_port: Option<u16>,
    pub end_port: Option<u16>,
    pub threads: Option<u32>,
    pub timeout: Option<u64>,
    pub no_closed: bool,
    pub sort: bool,
    pub interactive: bool,
}

pub struct ConfigError {
    pub message: String,
}

impl ConfigError {
    /// Initialize a new `ConfigError`
    fn new(message: &str) -> ConfigError {
        if message.is_empty() {
            panic!("Error message cannot be empty!");
        }

        ConfigError {
            message: String::from(message)
        }
    }
}

impl Config {
    /// Initialize a new `Config`
    ///
    /// # Arguments
    ///
    /// * `host` - The host (or IP address) that needs to be scanned
    /// * `start_port` - The initial port that needs to be scanned
    /// * `end_port` - The final port that needs to be scanned
    /// * `threads` - The amount of threads that need to be used to perform the scan
    /// * `timeout` - The connection timeout before a connection is marked as closed (in milliseconds)
    /// * `no_closed` - Indicates whether closed ports should be displayed or not
    /// * `sort` - Indicates whether the status of ports should be sorted depending on the port number
    /// * `interactive` - Indicates whether the scan should display results during the scan
    pub fn new(host: Option<String>, start_port: Option<u16>, end_port: Option<u16>, threads: Option<u32>, timeout: Option<u64>, no_closed: bool, sort: bool, interactive: bool) -> Result<Config, ConfigError> {
        match &end_port {
            None => {}
            Some(end) => {
                match &start_port {
                    None => {}
                    Some(start) => {
                        if end < start {
                            return Err(ConfigError::new("End port cannot be smaller than start port!"));
                        }
                    }
                }
            }
        };

        match &threads {
            None => {}
            Some(d) => {
                if d < &1 {
                    return Err(ConfigError::new("Threads cannot be smaller than 1!"));
                }
            }
        };

        match &timeout {
            None => {}
            Some(d) => {
                if d < &1 {
                    return Err(ConfigError::new("Timeout cannot be smaller than 1!"));
                }
            }
        };

        Ok(Config {
            host,
            start_port,
            end_port,
            threads,
            timeout,
            no_closed,
            sort,
            interactive,
        })
    }

    /// Read the `Config` struct using the application arguments
    pub fn read_from_args() -> Result<Config, ConfigError> {
        let matches = App::new("rscan")
            .version("1.0.1")
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
                .help("Sets whether closed ports should be outputted or not")
                .required(false)
                .takes_value(false))
            .arg(Arg::with_name("unsorted")
                .short("u")
                .long("unsorted")
                .help("Sets whether the output should be sorted by port number or not")
                .required(false)
                .takes_value(false))
            .arg(Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .help("Sets whether the output should be displayed while scanning or whether to wait until the scan has completed")
                .required(false)
                .takes_value(false))
            .get_matches();

        let host = matches.value_of("host");
        let threads = matches.value_of("threads");
        let start_port = matches.value_of("startport");
        let end_port = matches.value_of("endport");
        let timeout = matches.value_of("timeout");
        let no_closed = matches.is_present("noclosed");
        let sort = if matches.is_present("unsorted") { false } else { true };
        let interactive = matches.is_present("interactive");

        let host_value = match host {
            None => None,
            Some(d) => Some(String::from(d))
        };

        let threads_value: Option<u32> = match threads {
            None => Some(1),
            Some(d) => {
                let t: u32 = d.parse().expect("Threads is not a valid integer!");
                Some(t)
            }
        };

        let start_port_value: Option<u16> = match start_port {
            None => Some(0),
            Some(d) => {
                let t: u16 = d.parse().expect("Start port is not a valid port number!");
                Some(t)
            }
        };

        let end_port_value: Option<u16> = match end_port {
            None => Some(u16::MAX),
            Some(d) => {
                let t: u16 = d.parse().expect("End port is not a valid port number!");
                Some(t)
            }
        };

        let timeout_value: Option<u64> = match timeout {
            None => Some(250),
            Some(d) => {
                let t: u64 = d.parse().expect("Timeout is not a valid integer!");
                Some(t)
            }
        };

        Config::new(host_value, start_port_value, end_port_value, threads_value, timeout_value, no_closed, sort, interactive)
    }
}
