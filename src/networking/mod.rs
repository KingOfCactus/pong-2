use std::net::UdpSocket;
use std::time::Duration;
use bincode::serialized_size;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MessageContentType { I8, I16, I32, I64, ISIZE,
                              U8, U16, U32, U64, USIZE,
                              F32, F64, BOOL, CHAR,
                              STRING, STR 
                            }

#[derive(Serialize, Deserialize)]
pub struct NetworkMessage<T> {
    pub size: u8,
    pub content_type: MessageContentType,
    pub content: Box<T>
}

impl<T: Serialize + 'static> NetworkMessage<T> {
    pub fn new(content: T) -> NetworkMessage<T> {    
        let mut msg = NetworkMessage {
            content_type: Self::get_content_type(),
            content: Box::new(content),
            size: 0
        };
        
        msg.size = serialized_size(&msg).unwrap() as u8;
        return msg;
    }

    fn get_content_type() -> MessageContentType{
        let type_name = std::any::type_name::<T>();
        match type_name {
            "i8" => MessageContentType::I8,
            "i16" => MessageContentType::I16,
            "i32" => MessageContentType::I32,
            "i64" => MessageContentType::I64,
            "isize" => MessageContentType::ISIZE,

            "u8" => MessageContentType::U8,
            "u16" => MessageContentType::U16,
            "u32" => MessageContentType::U32,
            "u64" => MessageContentType::U64,
            "usize" => MessageContentType::USIZE,
            
            "f32" => MessageContentType::F32,
            "f64" => MessageContentType::F64,

            "&str" => MessageContentType::STR,
            "char" => MessageContentType::CHAR,            
            "alloc::string::String" => MessageContentType::STRING,
            
            "bool" => MessageContentType::BOOL,
            _ => panic!("Tried to use unknown type as NetworkMessage content: {}", type_name)
        }
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