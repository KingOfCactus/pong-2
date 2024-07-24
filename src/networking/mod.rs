use std::any::{Any, TypeId};
use std::net::UdpSocket;
use std::time::Duration;

pub struct NetMessage {
    pub type_id: TypeId,
    data: Vec<u8>,
    size: u8
}   

impl NetMessage {
    pub fn new<T: serde::Serialize + 'static>(msg: T) -> NetMessage {
        let encoded_msg = bincode::serialize(&msg).expect("Could not serialize packet data");
        
        return Self {
            type_id: msg.type_id(),
            size: encoded_msg.len() as u8,
            data: encoded_msg
        }
    }

    pub fn get_message<T: for<'a> serde::Deserialize<'a>>(&self) -> T {
        return bincode::deserialize::<T>(&self.data).expect("Could not deserialize packet data");
    }

}

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