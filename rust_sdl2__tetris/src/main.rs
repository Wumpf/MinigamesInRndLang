extern crate sdl2;
extern crate rand;
extern crate time;

mod application;
mod blocks;
mod gameboard;

use application::Application;

pub fn main() {
	let mut app = Application::new();
    app.run();
}
