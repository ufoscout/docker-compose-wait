use std::net::TcpStream;

pub fn is_reachable(address: &String) -> bool {
    match TcpStream::connect(address) {
        Ok(_stream) => true,
        Err(_e) => false,
    }
} // the stream is closed here

#[cfg(test)]
mod test {

    use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
    use super::*;
    use std::{thread, time};

    #[test]
    fn port_should_be_closed() {
        let available_port = available_port().to_string();
        let mut address = String::from("127.0.0.1:");
        address.push_str(&available_port);
        println!("Check for available connections on {}" , &address);
        assert!(!is_reachable(&address));
    }

    fn available_port() -> u16 {
        let loopback = Ipv4Addr::new(127, 0, 0, 1);
        let socket = SocketAddrV4::new(loopback, 0);
        let listener = TcpListener::bind(socket);
        let port = listener.unwrap().local_addr().unwrap();
        port.port()
    }

    #[test]
    fn port_should_be_open() {

        let loopback = Ipv4Addr::new(127, 0, 0, 1);
        let socket = SocketAddrV4::new(loopback, 0);
        let listener = TcpListener::bind(socket).unwrap();
        let listener_port = listener.local_addr().unwrap().to_string();

        thread::spawn(move || {
                loop {
                    match listener.accept() {
                        Ok(_) => {  println!("Connection received!"); }
                        Err(_) => { println!("Error in received connection!"); }
                }
                }
        });
        
        thread::sleep(time::Duration::from_millis(250));
        println!("Check for available connections on {}", &listener_port);
        assert!(is_reachable(&listener_port));
    }

}