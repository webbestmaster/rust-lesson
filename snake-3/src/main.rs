extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

pub struct Game {
    gl: GlGraphics,
}


impl Game {
    fn render(&mut self, args: &RenderArgs) {
        const green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(green, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 30;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    let width = COLS * SQUARE_WIDTH;
    let height = ROWS * SQUARE_WIDTH;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [width, height])
        // .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
            // println!("{:?}", r);
        }
    }
}
