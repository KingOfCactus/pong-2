use rand;
use rand::Rng;
use rand::thread_rng;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::GamepadAxis::*;
use raylib::consts::GamepadButton::*;

struct PlayerInput {
    on_gamepad: bool,

    smoothness : f32,
    raw_dir: Vector2,
    dir: Vector2,

    is_right_down: bool,
    is_left_down: bool,
    is_down_down: bool,
    is_up_down: bool
}

impl PlayerInput {
    fn new() -> Self {
        return Self {
            on_gamepad: false,
            smoothness: 0.0,
            raw_dir: Vector2::zero(),
            dir: Vector2::zero(),
            is_right_down: false,
            is_left_down: false,
            is_down_down: false,
            is_up_down: false,
        };
    }
}

struct Ball {
    position: Vector2,
    color: Color,
    radius: f32,
    speed: f32
}

const SCREEN_SIZE: Vector2 = Vector2{x: 640.0, y: 480.0};
const GAMEPAD_DEADZONE: f32 = 0.7;

fn main() {
    let (mut rl, _thread) = raylib::init()
        .size(SCREEN_SIZE.x as i32, SCREEN_SIZE.y as i32)
        .title("Pong 2").vsync().build();
   
    rl.set_target_fps(60);
    let mut show_stats = true;

    let mut move_direction =Vector2::one();
    let mut player_velocity;
    let mut player = Ball {
        position: Vector2 { x: SCREEN_SIZE.x * 0.5, y: SCREEN_SIZE.y * 0.5 },
        color: Color::WHITE,
        speed: 540.0,
        radius: 10.0
    };
    
    let mut input = PlayerInput::new();
    input.smoothness = 3.0;
    
    // Each frame
    while !rl.window_should_close() {
        // <-- GAME LOGIC -->
        update_player_input(&mut input, &rl);
        player_velocity = input.dir * player.speed / 2.25;

        // If on horizontal screen edge
        if player.position.x == player.radius || player.position.x == SCREEN_SIZE.x - player.radius {
            move_direction.x *= -1.0;

            let y: f32 = thread_rng().gen();
            if move_direction.y >= 0.0 { move_direction.y = y; }
            else { move_direction.y = -y; }

            input.dir = Vector2 { x: 0.0, y: 0.0 };
        }

        // If on vertical screen edge
        if player.position.y == player.radius || player.position.y == SCREEN_SIZE.y - player.radius {
            move_direction.y *= -1.0;
            input.dir = Vector2 { x: 0.0, y: 0.0 };
            /* // Get a new y direction keeping the same signal
            let new_y: f32 = thread_rng().gen();
            if move_direction.y >= 0.0 { move_direction.y = new_y; }
            else { move_direction.y = -new_y; }

            // move_direction.y *= -1.0;
            input.dir = Vector2 { x: 0.0, y: 0.0 }; */
        }

        // Move player
        player_velocity += move_direction * player.speed;
        player.position += player_velocity * rl.get_frame_time();
        player.position.x = player.position.x.clamp(player.radius, SCREEN_SIZE.x - player.radius);
        player.position.y = player.position.y.clamp(player.radius, SCREEN_SIZE.y - player.radius);


        // <-- DRAW SCREEN -->
        if rl.is_key_pressed(KEY_TAB) || rl.is_gamepad_button_pressed(0, GAMEPAD_BUTTON_MIDDLE_RIGHT) { show_stats = !show_stats; }
        let mut stats;
        stats = "(".to_owned() + format!("{:.2}", input.raw_dir.x).as_str() + ", " + format!("{:.2}", input.raw_dir.y).as_str() + ")\n";
        stats = stats + "(" + format!("{:.2}", input.dir.x).as_str() + ", " + format!("{:.2}", input.dir.y).as_str() + ")\n";
        if input.on_gamepad {
            stats = stats + &rl.get_gamepad_name(0).expect("UNKNOWN") + " Connected \n";
            stats = stats + "(" + format!("{:.2}", rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X)).as_str() + ", " + format!("{:.2}", rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y)).as_str() + ")\n";
        }
        else {
            let mut keys_down = "".to_string();
            if input.is_up_down { keys_down += "w "; }
            if input.is_down_down { keys_down += "s "; }
            if input.is_left_down { keys_down += "a "; }
            if input.is_right_down { keys_down += "d "; }
            stats += &keys_down;
        }

        let mut draw_handle = rl.begin_drawing(&_thread);
        draw_handle.clear_background(Color::BLACK);

        if show_stats {
            draw_handle.draw_fps(0, 0);
            draw_handle.draw_text(&stats, 0, (SCREEN_SIZE.y * 0.05) as i32, 18, Color::GREEN);
        }

        draw_handle.draw_text("Pong 2", (SCREEN_SIZE.x * 0.465) as i32, (SCREEN_SIZE.y * 0.01) as i32, 18, Color::RED);
        draw_handle.draw_circle_v(player.position, player.radius, player.color);
        
    }

    fn update_player_input(input: &mut PlayerInput, rl: &RaylibHandle) {
        input.on_gamepad = rl.is_gamepad_available(0);
        let gamepad_axis = Vector2 {
            x: rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X),
            y: rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y)
        };

        // Get buttons state
        if input.on_gamepad {
            input.is_right_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_RIGHT) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X) > GAMEPAD_DEADZONE;
            input.is_left_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_LEFT) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X) < -GAMEPAD_DEADZONE;
            input.is_down_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_UP) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y) < -GAMEPAD_DEADZONE;
            input.is_up_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_DOWN) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y) > GAMEPAD_DEADZONE;
        }
        else {
            input.is_right_down = rl.is_key_down(KEY_D) || rl.is_key_down(KEY_RIGHT);
            input.is_left_down = rl.is_key_down(KEY_A) || rl.is_key_down(KEY_LEFT);
            input.is_down_down = rl.is_key_down(KEY_S) || rl.is_key_down(KEY_DOWN);
            input.is_up_down = rl.is_key_down(KEY_W) || rl.is_key_down(KEY_UP);
        }

        // Get raw input direction
        if input.on_gamepad && gamepad_axis != Vector2::zero() {
            input.raw_dir.x =  rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X);
            input.raw_dir.y =  rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y);
        }
        else {
            if !input.is_right_down && !input.is_left_down { input.raw_dir.x = 0.0; }
            else if input.is_right_down && !input.is_left_down { input.raw_dir.x = 1.0; }
            else if !input.is_right_down { input.raw_dir.x = -1.0; }
            
            if !input.is_down_down && !input.is_up_down { input.raw_dir.y = 0.0; }
            else if input.is_down_down && !input.is_up_down { input.raw_dir.y = 1.0; }
            else if !input.is_down_down {input.raw_dir.y = -1.0; }
        }

        // Smooth raw_dir into input direction
        input.dir = input.dir.lerp(input.raw_dir, input.smoothness * rl.get_frame_time());

    }
}