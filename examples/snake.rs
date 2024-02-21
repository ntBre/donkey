use donkey::{
    colors::{color, GREEN, RED},
    keys::Key,
    window_ex::with_window,
};
use raylib_sys::Color;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const TITLE: &str = "snake";
const SQR_SIZE: usize = 40;
const SNAKE_COLOR: Color = GREEN;

struct Snake {
    x: usize,
    y: usize,
}

fn main() {
    let background = color(0x181818AA);
    let mut s = Snake { x: 0, y: 0 };
    // TODO builder here for window settings like fps
    with_window(WIDTH, HEIGHT, TITLE, |win| {
        win.draw(|canvas| {
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
            if win.is_key_pressed(Key::W) && s.y > 0 {
                s.y -= SQR_SIZE;
            }
            if win.is_key_pressed(Key::S) && s.y < HEIGHT as usize - SQR_SIZE {
                s.y += SQR_SIZE;
            }
            if win.is_key_pressed(Key::A) && s.x > 0 {
                s.x -= SQR_SIZE;
            }
            if win.is_key_pressed(Key::D) && s.x < WIDTH as usize - SQR_SIZE {
                s.x += SQR_SIZE;
            }
        });
    });
}
