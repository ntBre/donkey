use std::time::UNIX_EPOCH;

use donkey::{
    colors::{color, GREEN, RED, WHITE},
    keys::Key,
    window_ex::with_window,
};
use raylib_sys::Color;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const TITLE: &str = "snake";
const SQR_SIZE: usize = 40;
const SNAKE_COLOR: Color = GREEN;

enum Direction {
    North,
    South,
    East,
    West,
}

struct Snake {
    x: usize,
    y: usize,
    dir: Direction,
}

fn rand() -> u32 {
    UNIX_EPOCH.elapsed().unwrap().as_nanos() as u32
}

fn rand_float() -> f32 {
    rand() as f32 / u32::MAX as f32
}

fn main() {
    let background = color(0x181818AA);
    let mut s = Snake { x: 0, y: 0, dir: Direction::East };
    let mut apple = None;
    // TODO builder here for window settings like fps
    with_window(WIDTH, HEIGHT, TITLE, |win| {
        win.set_target_fps(60);
        win.draw(|canvas| {
            let dt = win.get_frame_time();
            canvas.clear_background(background);
            // tile the background like tsoding
            for x in (0..WIDTH as usize).step_by(SQR_SIZE) {
                for y in (0..HEIGHT as usize).step_by(SQR_SIZE) {
                    if (x + y) / SQR_SIZE % 2 == 0 {
                        canvas.draw_rectangle(x, y, SQR_SIZE, SQR_SIZE, RED);
                    }
                }
            }
            // draw the snake
            canvas.draw_rectangle(s.x, s.y, SQR_SIZE, SQR_SIZE, SNAKE_COLOR);
            if win.is_key_pressed(Key::W) {
                s.dir = Direction::North;
            }
            if win.is_key_pressed(Key::S) {
                s.dir = Direction::South;
            }
            if win.is_key_pressed(Key::A) {
                s.dir = Direction::West;
            }
            if win.is_key_pressed(Key::D) {
                s.dir = Direction::East;
            }
            let delta = (5.0 * dt * SQR_SIZE as f32) as usize;
            match s.dir {
                Direction::North => match s.y.checked_sub(delta) {
                    Some(v) => s.y = v,
                    None => s.y = HEIGHT as usize,
                },
                Direction::South => {
                    s.y += delta;
                    if s.y > HEIGHT as usize {
                        s.y = 0;
                    }
                }
                Direction::East => {
                    s.x += delta;
                    if s.x > WIDTH as usize {
                        s.x = 0;
                    }
                }
                Direction::West => match s.x.checked_sub(delta) {
                    Some(v) => s.x = v,
                    None => s.x = WIDTH as usize,
                },
            };
            // draw apples
            if let Some((x, y)) = apple {
                canvas.draw_rectangle(
                    x as usize, y as usize, SQR_SIZE, SQR_SIZE, WHITE,
                );
            } else if rand_float() < 0.5 {
                let x = rand() % WIDTH as u32;
                let y = rand() % HEIGHT as u32;
                apple = Some((x as usize, y as usize));
            }
        });
    });
}
