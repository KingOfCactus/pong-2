use std::net::UdpSocket;
use std::time::Duration;
use std::io::stdin;

// yeah, not commiting hard-coded ip values today 👍
pub const PC_1_IP: &str = "";
pub const PC_2_IP: &str = "";

pub struct NetworkManager {
    remote_addr: String
}

impl NetworkManager {
    pub fn new(remote: String) -> NetworkManager {
        return Self { remote_addr: remote+":25565" };
    }
    
    pub fn punch_hole(self: &mut Self) {
        let socket = UdpSocket::bind("0.0.0.0:25565").unwrap();
        socket.set_write_timeout(Some(Duration::from_secs(1))).expect("Write timeout overwrite failed");

        let result = socket.send_to(&[1;10], self.remote_addr.clone()).expect("Sending failed -> ");
        println!("Message sended (code {})", result);
    }

    pub fn listen(self: &mut Self) {
        let socket = UdpSocket::bind("0.0.0.0:25565").expect("Bind failed");
        let result = socket.set_read_timeout(Some(Duration::from_secs(10))).expect("Read timeout overwrite failed");

        let mut msg = &mut [0 as u8; 4];
        socket.recv(msg).expect("Could not receive message");

        println!("Message received: {}", msg[0]);
    }

    // ugly test code, proceed with caution
    pub fn test() {
        let mut remote = "".to_string();
        println!("1) Pc 1");
        println!("2) Pc 2");

        let mut input = "".to_string();
        let mut remote = "";
        stdin().read_line(&mut input);
        input = input.trim().to_string();
        
        if input == "1" {
            remote = PC_1_IP
        }
        else {
            remote = PC_2_IP;
        }

        let mut net = NetworkManager::new(remote.to_string());
        print!("{}[2J", 27 as char);

        while (true) {
            println!("1) Punch hole");
            println!("2) Listen for 10 sec");

            let mut input = "".to_string();
            stdin().read_line(&mut input);
            input = input.trim().to_string();
            
            if input == "1" {
                net.punch_hole()
            }
            else if input == "2" {
                net.listen();
            }
            
            println!("");println!("");println!("");println!("");println!("");
        }
    }
}