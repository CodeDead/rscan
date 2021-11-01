/// Indicates the status of a port. Can be either open or closed
pub enum PortStatus {
    Open,
    Closed,
}

/// Represents the result of a port scan
pub struct ScanResult {
    pub host: String,
    pub port: u16,
    pub port_status: PortStatus,
}

impl ScanResult {
    /// Initialize a new ScanResult
    ///
    /// # Arguments
    ///
    /// * `host` - The host that was scanned
    /// * `port` - The port number
    /// * `port_status` - The `PortStatus` enum that indicates whether the port is open or closed
    pub fn new(host: &str, port: u16, port_status: PortStatus) -> ScanResult {
        ScanResult {
            host: String::from(host),
            port,
            port_status,
        }
    }
}
