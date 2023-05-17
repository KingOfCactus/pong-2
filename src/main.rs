use rand;
use rand::Rng;
use rand::thread_rng;
use raylib::prelude::*;
use raylib::prelude::Vector2;
use raylib::consts::KeyboardKey::*;
use raylib::consts::GamepadAxis::*;
use raylib::consts::GamepadButton::*;

const VECTOR_ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
//const VECTOR_ONE: Vector2 = Vector2 { x: 1.0, y: 1.0 };

const SCREEN_SIZE: Vector2 = Vector2 { x: 640.0, y: 480.0 };
const GAMEPAD_DEADZONE: f32 = 0.7;
const PADDLE_PADDING: f32 = 20.0;
const PADDLE_SIZE: Vector2 = Vector2 { x: 11.0, y: 65.0 };
const PADDLE_VELOCITY: f32 = 500.0;

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
            raw_dir: VECTOR_ZERO,
            dir: VECTOR_ZERO,
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
fn main() {
    let (mut rl, _thread) = raylib::init()
        .size(SCREEN_SIZE.x as i32, SCREEN_SIZE.y as i32)
        .title("Pong 2").vsync().build();
   
    rl.set_target_fps(60);
    let mut show_stats = true;

    let mut move_direction = Vector2 { x: -1.0, y: 0.0 };
    let mut player_velocity;
    let mut player = Ball {
        position: Vector2 { x: SCREEN_SIZE.x * 0.5, y: SCREEN_SIZE.y * 0.5 },
        color: Color::WHITE,
        speed: 500.0,
        radius: 10.0
    };
    
    let mut input = PlayerInput::new();
    let mut score = 0;
    input.smoothness = 3.0;
    
    let mut left_paddle = Rectangle::new( 
        PADDLE_PADDING, 
        SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0,
        PADDLE_SIZE.x,
        PADDLE_SIZE.y
    );

    let mut right_paddle = Rectangle::new(
        SCREEN_SIZE.x - PADDLE_SIZE.x - PADDLE_PADDING,
        SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0,
        PADDLE_SIZE.x,
        PADDLE_SIZE.y
    );

    let input_multiplier = Vector2 { x: player.speed / 2.25, y: player.speed / 1.70 };
    let mut alpha = 185.0;

    // Each frame
    while !rl.window_should_close() {
        // <-- GAME LOGIC -->
        update_player_input(&mut input, &rl);

        // Set player color
        if input.raw_dir != VECTOR_ZERO { 
            alpha += 340.0 * ((input.dir.x.abs() + input.dir.y.abs())) * rl.get_frame_time(); 
        }
        else { 
            alpha -= 500.0 * (1.0 - (input.dir.x.abs() + input.dir.y.abs()) / 2.0) * rl.get_frame_time(); 
        }

        alpha = alpha.clamp(185.0, 255.0);
        player.color.a = alpha as u8;

        // Restart if player is outside of the screen
        if player.position.x > SCREEN_SIZE.x || player.position.x < 0.0 {
            // Reset stats
            score = 0;
            input.dir = VECTOR_ZERO;
            move_direction = Vector2 { x: -1.0, y: 0.0 };

            // Reset paddles
            player.position = SCREEN_SIZE / 2.0;
            left_paddle.y =  SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0;
            right_paddle.y =  SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0;

        }

        // Bounce when hit a paddle
        let hit_left_paddle = left_paddle.check_collision_circle_rec(player.position, player.radius + 5.0);
        let hit_right_paddle = right_paddle.check_collision_circle_rec(player.position, player.radius + 5.0);
        
        if hit_left_paddle || hit_right_paddle {
            let mut new_angle: f32 = thread_rng().gen();

            // Copy input direction or keep previous direction 
            if input.raw_dir.y == 0.0 { new_angle *= move_direction.y.signum(); }
            else { new_angle *= input.raw_dir.y.signum(); }
            
            // Keep player out of the paddles
            let min = left_paddle.x + PADDLE_SIZE.x + player.radius;
            let max = right_paddle.x - PADDLE_SIZE.x - player.radius;
            player.position = player.position.clamp(min, max);

            // Set new direction
            move_direction.x *= -1.0;
            move_direction.y = new_angle;
            input.dir = Vector2 { x: 0.0, y: 0.0 };

            // Set new player stats
            player_velocity = VECTOR_ZERO;          
            score += 1;
        }

        // Bounce when hit top or bottom screen
        if player.position.y == player.radius || player.position.y == SCREEN_SIZE.y - player.radius {
            let mut new_angle = move_direction.y.abs();
            new_angle = new_angle.clamp(0.6, 1.0);

            new_angle *= -move_direction.y.signum();
            move_direction.y = new_angle;
            
            input.dir.x *= 0.5;
            input.dir.y = 0.0;
        }

        // Set velocity
        player_velocity = input.dir * input_multiplier;
        player_velocity += move_direction * player.speed;

        // Apply velocity
        player.position += player_velocity * rl.get_frame_time();
        player.position.y = player.position.y.clamp(player.radius, SCREEN_SIZE.y - player.radius);


        // LEFT PADDLET
        let mut paddlet_speed: f32 = 500.0;
        let mut paddlet_view_range = 0.5;
        
        let player_distance = (1.0 - ((player.position.x + left_paddle.x) / (SCREEN_SIZE.x * paddlet_view_range))).powf(2.0);

        let mut left_paddlet_target = (player.position.y - left_paddle.y) * player_distance * 80.0;
        if left_paddlet_target.abs() < 20.0 { left_paddlet_target = 0.0 }
        else { left_paddlet_target = left_paddlet_target.clamp(-paddlet_speed, paddlet_speed) }

        if player.position.x - left_paddle.x > SCREEN_SIZE.x * paddlet_view_range { left_paddlet_target = 0.0 }

        left_paddle.y += left_paddlet_target * rl.get_frame_time();


        // RIGHT PADDLET
        let player_distance = ((player.position.x - SCREEN_SIZE.x * paddlet_view_range) / (right_paddle.x - SCREEN_SIZE.x * paddlet_view_range)).powf(2.0);

        let mut right_paddlet_target = (player.position.y - right_paddle.y) * player_distance * 80.0;
        if right_paddlet_target.abs() < 10.0 { right_paddlet_target = 0.0 }
        else { right_paddlet_target = right_paddlet_target.clamp(-paddlet_speed, paddlet_speed) }

        if (right_paddle.x - player.position.x > SCREEN_SIZE.x * paddlet_view_range) { right_paddlet_target = 0.0 }

        right_paddle.y += right_paddlet_target * rl.get_frame_time();


        // <-- DRAW SCREEN -->
        if rl.is_key_pressed(KEY_TAB) || rl.is_gamepad_button_pressed(0, GAMEPAD_BUTTON_MIDDLE_RIGHT) { show_stats = !show_stats; }
        let mut stats;
        stats = "(".to_owned() + format!("{:.2}", move_direction.x).as_str() + ", " + format!("{:.2}", move_direction.y).as_str() + ")\n";
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

        let text_X = SCREEN_SIZE.x / 2.0 - (measure_text(score.to_string().as_str(), 22) as f32 / 2.0);


        draw_handle.draw_text(score.to_string().as_str(), text_X as i32, (SCREEN_SIZE.y * 0.01) as i32, 22, Color::RED);
        draw_handle.draw_circle_v(player.position, player.radius, player.color);
        draw_handle.draw_rectangle_rec(&left_paddle, Color::GRAY);
        draw_handle.draw_rectangle_rec(&right_paddle, Color::GRAY);
    }
    










    fn update_player_input(input: &mut PlayerInput, rl: &RaylibHandle) {
        // Update gamepad data
        input.on_gamepad = rl.is_gamepad_available(0);
        let gamepad_axis = Vector2 {
            x: rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X),
            y: rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y)
        };

        // Update buttons state
        if input.on_gamepad {
            input.is_right_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_RIGHT) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X) > GAMEPAD_DEADZONE;
            input.is_left_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_LEFT) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X) < -GAMEPAD_DEADZONE;
            input.is_down_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_DOWN) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y) < -GAMEPAD_DEADZONE;
            input.is_up_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_UP) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y) > GAMEPAD_DEADZONE;
        }
        else {
            input.is_right_down = rl.is_key_down(KEY_D) || rl.is_key_down(KEY_RIGHT);
            input.is_left_down = rl.is_key_down(KEY_A) || rl.is_key_down(KEY_LEFT);
            input.is_down_down = rl.is_key_down(KEY_S) || rl.is_key_down(KEY_DOWN);
            input.is_up_down = rl.is_key_down(KEY_W) || rl.is_key_down(KEY_UP);
        }

        // Update raw direction
        if input.on_gamepad && gamepad_axis != VECTOR_ZERO {
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