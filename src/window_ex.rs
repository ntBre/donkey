//! This is an experimental Window and drawing API based on closures rather than
//! begin and end pairs

use std::ffi::c_int;

use raylib_sys::DrawLine;

use crate::{colors::IntoColor, Window};

pub fn with_window(
    width: i32,
    height: i32,
    title: &str,
    mut f: impl FnMut(&Window),
) {
    let win = Window::init(width, height, title);
    while !win.should_close() {
        f(&win);
    }
}

pub struct Canvas;

impl Canvas {
    pub fn clear_background(&self, color: impl IntoColor) {
        // TODO if I move to this API, move the Window version here
        Window.clear_background(color);
    }

    // TODO you actually want i32 on the positions because then it's okay for
    // them to be slightly negative. otherwise my snake, for example, teleports
    // before the square is visually to the edge of the screen
    pub fn draw_rectangle(
        &self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: impl IntoColor,
    ) {
        Window.draw_rectangle(x, y, w, h, color)
    }

    pub fn draw_line(
        &self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        color: impl IntoColor,
    ) {
        unsafe {
            DrawLine(
                x1 as c_int,
                y1 as c_int,
                x2 as c_int,
                y2 as c_int,
                color.into(),
            )
        }
    }
}

impl Window {
    pub fn draw(&self, mut f: impl FnMut(&Canvas)) {
        self.begin_drawing();
        f(&Canvas);
        self.end_drawing();
    }
}
