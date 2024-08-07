use std::any::Any;
use std::any::TypeId;
use std::fs;
use std::env;
use std::io::Write;
use bincode::deserialize;
use bincode::serialize;
use raylib::prelude::*;
use raylib::prelude::Vector2;

use crate::input_system::*;
use crate::networking::*;

pub struct MiscUtils; 
pub struct DebugUtils;
pub struct InputUtils;
pub struct NetworkUtils;

pub const SCREEN_SIZE: Vector2 = Vector2 { x: 640.0, y: 480.0 };
const MAX_CONNECTED_GAMEPADS: usize = 4;


impl InputUtils {
    pub fn get_device_by_id(id: i32) -> Box<dyn InputDevice>{
        match id {
            0 => return Box::new(KeyboardInput::new(true)),
            1 => return Box::new(KeyboardInput::new(false)),
            _ => return Box::new(GamepadInput::new(id - 2, true))
        }
    }
    
    pub fn get_connected_devices(rl: &RaylibHandle) -> Vec<Box<dyn InputDevice>> {
        let mut devices: Vec<Box<dyn InputDevice>> = Vec::with_capacity(MAX_CONNECTED_GAMEPADS + 2);
        devices.insert(0, Box::new(KeyboardInput::new(true)));
        devices.insert(1, Box::new(KeyboardInput::new(false)));
    
        for i in 0..MAX_CONNECTED_GAMEPADS{
            if rl.is_gamepad_available(i as i32) {
                devices.insert(i + 2, Box::new(GamepadInput::new(i as i32, true)));
            }
        }
    
        return devices;
    }
}

impl NetworkUtils {
    pub fn decode_msg_header(msg: &[u8]) -> (u8, MessageContentType) {
        return (msg[0], unsafe { std::mem::transmute(msg[1]) });
    }
}

impl MiscUtils {
    pub fn init_window() -> (RaylibHandle, RaylibThread) {
        let (mut rl_handle, _thread) = raylib::init()
            .size(SCREEN_SIZE.x as i32, SCREEN_SIZE.y as i32)
            .title("Pong 2").vsync().build();
    
        rl_handle.set_target_fps(60);
        return (rl_handle, _thread);
    }
    
    pub fn get_highscore() -> i32 {
        match fs::read_to_string("highscore.txt") {
            Ok(s) => return s.parse::<i32>().unwrap(),
            // Create file if doesn't exist
            _ => { 
                println!("File 'highscore.txt' doesn't exist, creating...");
                let mut file = fs::File::create("highscore.txt").unwrap();
                file.write_all(b"0").unwrap();
                return 0;
             }
        }
    }
    
    pub fn save_highscore(i: i32) {
        let mut file = fs::OpenOptions::new().write(true).open("highscore.txt").unwrap();
        let buffer: String = i.to_string();
        file.write_all(buffer.as_bytes()).unwrap();
    }
}

impl DebugUtils {
    pub fn is_debug_session() -> bool {
        let debug = env::var("DEBUG");
        match debug {
            Ok(_) => if debug.unwrap().eq("1") { true } else { false} 
            _ => false
        }
    }

    // very ugly test code, proceed with caution
    pub fn debug() {
        let msg = NetworkMessage::new("Big string jumpscare: 1231234012347033481-230984902840-293");
        let b = serialize(&msg).unwrap();
        let header = NetworkUtils::decode_msg_header(&b);

        println!("[Header] {}, {:?}", header.0 ,header.1);

        match header.1 {
            MessageContentType::STR => {
                let msg: NetworkMessage<String> = deserialize(&b).unwrap();
                println!("[Content] {}", msg.content);
            },
            MessageContentType::I32 => {
                let msg: NetworkMessage<i32> = deserialize(&b).unwrap();
                println!("[Content] {}", msg.content);
            },
            MessageContentType::F32 => {
                let msg: NetworkMessage<f32> = deserialize(&b).unwrap();
                println!("[Content] {}", msg.content);
            },
            MessageContentType::CHAR => {
                let msg: NetworkMessage<char> = deserialize(&b).unwrap();
                println!("[Content] {}", msg.content);
            },
            MessageContentType::BOOL => {
                let msg: NetworkMessage<bool> = deserialize(&b).unwrap();
                println!("[Content] {}", msg.content);
            },
            _ => println!("Unknown type")
        }

        print!("[Encoded ({})] ", b.len());
        for a in b { print!("{}, ", a); }
        println!();
        panic!();

        //let dm: NetworkMessage = deserialize(&b).expect("Could not deserialize");
    // println!("{}", dm);



        
        let remote = env::var("REMOTE").expect("REMOTE variable not set");
        let mut net = NetworkManager::new(remote.to_string());
        print!("{}[2J", 27 as char);

        while (true) {
            println!("1) Punch hole");
            println!("2) Listen");

            let mut input = "".to_string();
            std::io::stdin().read_line(&mut input);
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
 





