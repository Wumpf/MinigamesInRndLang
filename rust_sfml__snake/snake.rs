extern crate native;
extern crate rsfml;

use rsfml::system::{Vector2f, Vector2i};
use rsfml::window::{ContextSettings, VideoMode, event, Close};
use rsfml::graphics::{RenderWindow, Shape, Color};
use rsfml::graphics::rc::{CircleShape};
use rsfml::traits::drawable::Drawable;

static resolution: (i32, i32) = (1024, 1024);
static tilePixelSize: i32 = 32;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn tileToPixelCoor(tileCoor: &Vector2i) -> Vector2f {
  Vector2f::new((tileCoor.x * tilePixelSize) as f32, (tileCoor.x * tilePixelSize) as f32)
}

trait Placeable {
  fn setPosition(&mut self, tilePosition: &Vector2i) -> ();
}
impl Placeable for CircleShape {
  fn setPosition(&mut self, tilePosition: &Vector2i) -> () {
     self.set_position(&tileToPixelCoor(tilePosition));
  }  
}

struct Snake {
  head: Vector2i,
  body: Vec<Vector2i>
}
impl Snake {
  fn draw<HeadShape: Placeable + Drawable>(&self, window: &mut RenderWindow, headShape: &mut HeadShape) {
    headShape.setPosition(&self.head);
    window.draw(headShape);
  }
}

fn main () -> () {
    // Create the window of the application
    let mut window = match RenderWindow::new(VideoMode::new_init(resolution.val0() as uint, resolution.val1() as uint, 32),
                                             "Rust Snake",
                                             Close,
                                             &ContextSettings::default()) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };

    let mapSize = Vector2i{ x: resolution.val0() / tilePixelSize, y: resolution.val1() / tilePixelSize };

    // Create a Snake
    let mut snake = Snake{ head: mapSize.div(&2i), body: vec![] };

    // Create body shapes
    let mut headShape = match CircleShape::new() {
        Some(circle) => circle,
        None         => fail!("Error, cannot create ball")
    };
    headShape.set_radius(tilePixelSize as f32 * 0.5f32);
    headShape.set_fill_color(&Color::red());

    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _             => {/* do nothing */}
            }
        }

        // Clear the window
        window.clear(&Color::new_RGB(0, 200, 200));
        


        snake.draw(&mut window, &mut headShape);



        // Display things on screen
        window.display()
    }
}