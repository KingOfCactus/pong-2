use std::net::UdpSocket;
use std::time::Duration;

pub struct NetworkManager {
    remote_addr: String
}

impl NetworkManager {
    pub fn new(remote: String) -> NetworkManager {
        return Self { remote_addr: remote+":26655" };
    }
    
    pub fn punch_hole(self: &mut Self) {
        let socket = UdpSocket::bind("0.0.0.0:26655").unwrap();
        socket.set_write_timeout(Some(Duration::from_secs(1))).unwrap();

        let result = socket.send_to(&[1;10], self.remote_addr.clone()).expect("Sending failed -> ");
        println!("Message sended (code {})", result);
    }

    pub fn listen(self: &mut Self) {
        let socket = UdpSocket::bind("0.0.0.0:26655").unwrap();
        let result = socket.set_read_timeout(Some(Duration::from_secs(1))).unwrap();

        let mut msg = &mut [0 as u8; 4];
        match socket.recv(msg) {
            Ok(_) => println!("Message received: {}{}{}{}", msg[0], msg[1], msg[2], msg[3]),
            Err(_) => println!("Message not received"),
        }

    }
}