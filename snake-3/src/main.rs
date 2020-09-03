use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

mod game;

use game::{Game};
use game::snake::direction::{Direction};
use game::snake::{Snake, SnakePiece};
use game::food::{Food};

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
        gl: GlGraphics::new(opengl),
        rows: ROWS,
        cols: COLS,
        square_width: SQUARE_WIDTH,
        just_eaten: false,
        food: Food { x: 1, y: 1 },
        score: 0,
        snake: Snake {
            gl: GlGraphics::new(opengl),
            snake_parts: LinkedList::from_iter((vec![SnakePiece(COLS / 2, ROWS / 2)]).into_iter()),
            width: SQUARE_WIDTH,
            d: Direction::DOWN,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
    println!("Congratulations, your score was: {}", game.score);
}
