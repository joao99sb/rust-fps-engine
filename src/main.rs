use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;

const SCREEN_WIDTH: usize = 1024;
const SCREEN_HEIGHT: usize = 720;

const PLAYER_X: f32 = 8.0;
const PLAYER_Y: f32 = 8.0;
const PLAYER_A: f32 = 0.0; // angle

const MAP_HEIGHT: i32 = 16;
const MAP_WIDTH: i32 = 16;

const FOV: f32 = PI / 4.0;

const DEPTH: f32 = 8.0;

const FRAMES_PER_SECOND: f64 = 60.0; //hz
const SECONS_PER_FRAME: f64 = 1.0 / FRAMES_PER_SECOND;
const MICROSECONDS_PER_FRAME: u64 = (SECONS_PER_FRAME * 10e6) as u64;

fn main() {
    let mut map: String = String::new();

    map += "################";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "################";

    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(
        MICROSECONDS_PER_FRAME,
    )));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for x in 0..SCREEN_WIDTH {
            let ray_angle: f32 =
                (PLAYER_A - FOV / 2.0) + ((x as f32) / (SCREEN_WIDTH as f32)) * FOV;

            let mut distance_to_wall: f32 = 0.0;

            let mut hit_wall: bool = false;

            let eye_x = ray_angle.sin();
            let eye_y = ray_angle.cos();

            while !hit_wall && distance_to_wall < DEPTH {
                distance_to_wall += 0.1;

                let test_x: i32 = (PLAYER_X + eye_x * distance_to_wall) as i32;
                let test_y: i32 = (PLAYER_Y + eye_y * distance_to_wall) as i32;

                if test_x < 0 || test_x >= MAP_WIDTH || test_y < 0 || test_y >= MAP_HEIGHT {
                    hit_wall = true;
                    distance_to_wall = DEPTH;
                } else {
                    let character = map
                        .chars()
                        .nth((test_y * MAP_WIDTH + test_x) as usize)
                        .unwrap();
                    
                    if character == '#' {
                        hit_wall = true;
                    }
                }
            }

            let ceiling =
                (SCREEN_HEIGHT as f32 / 2.0 - SCREEN_HEIGHT as f32 / distance_to_wall) as usize;
            let flor = SCREEN_HEIGHT - ceiling;

            for y in 0..SCREEN_HEIGHT {
                if y < ceiling {
                    buffer[y * SCREEN_WIDTH + x] = 0x00_AA_0F_00;
                } else if y > ceiling && y < flor {
                    buffer[y * SCREEN_WIDTH + x] = 0xFF_FF_FF_FF;
                } else {
                    buffer[y * SCREEN_WIDTH + x] = 0x3B_20_40_0A;
                }
            }
        }

        window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
