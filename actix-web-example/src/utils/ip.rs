use std::net::UdpSocket;

//#![windows_subsystem = "windows"]
pub fn what_is_my_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };
    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };
    match socket.local_addr() {
        Ok(addr) => Some(addr.ip().to_string()),
        Err(_) => return None,
    }
}
