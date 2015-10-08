use sdl2;
use sdl2::pixels::Color;
use gameboard::*;
use blocks::*;
use time;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 1024;

const TIME_STEP: f64 = 0.4;
const MOVEMENT_STEP: f64 = TIME_STEP * 0.5;
const DOWN_SPEEDUP: f64 = 5.0;

pub struct Application<'a> {
	sdl_context: sdl2::Sdl,
	renderer: sdl2::render::Renderer<'a>,
	running: bool,
	gameboard: Gameboard,
	last_timestep: f64,
	last_move_time: f64,
	move_down_fast: bool,
}

impl<'a> Application<'a> {
	/// Initializes the application.
	pub fn new() -> Application<'a> {
		let sdl_context = sdl2::init()
						.ok()
						.expect("Failed to init SDL2.");
						
    	let video_subsystem = sdl_context.video()
    								  .ok()
    								  .expect("Failed to create video context.");
		
		let window = video_subsystem.window("Testris, written in Rust & SDL2", WINDOW_WIDTH, WINDOW_HEIGHT).build()
									    .ok()
    								    .expect("Failed to create window.");
		
		let renderer = window.renderer().build()
    								  .ok()
    								  .expect("Failed to create renderer.");
		
		Application {
			sdl_context: sdl_context,
			renderer: renderer,
			running: true,
			gameboard: Gameboard::new(),
			last_timestep: time::precise_time_s(),
			last_move_time: time::precise_time_s(),
			move_down_fast: false,
		}
	}
	
	pub fn run(&mut self) {
		while self.running {
			self.update();
			self.render();
		}
	}
	
	fn update(&mut self) {
		let mut event_pump = self.sdl_context.event_pump().unwrap();
		
		// Handle all sdl events.
		for event in event_pump.poll_iter() {
			self.handle_event(event);
		}
		
		// Step world.
		let mut step_time = TIME_STEP;
		if self.move_down_fast == true {
			step_time /= DOWN_SPEEDUP;
		}
		let current_time = time::precise_time_s();
		if current_time - self.last_timestep > step_time {
			self.last_timestep = current_time;
			self.gameboard.fall_down();
		}
		
		// Generate new piece if there is none.
		if self.gameboard.has_active_piece() == false {
			self.gameboard.insert_piece(generate_random_piece());
		}
	}
	
	fn render(&mut self) {
		self.renderer.set_draw_color(Color::RGB(100, 149, 237));
		self.renderer.clear();
		
		self.gameboard.draw(&mut self.renderer, WINDOW_WIDTH, WINDOW_HEIGHT);
		
		self.renderer.present();
	}
	
	fn handle_event(&mut self, event: sdl2::event::Event) {
		use sdl2::event::Event;
		use sdl2::keyboard::Keycode;
		
		let current_time = time::precise_time_s();
		match event {
			Event::KeyDown { keycode: Some(Keycode::Up), .. } |
			Event::KeyDown { keycode: Some(Keycode::Left), .. } |
			Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
				if current_time - self.last_move_time > MOVEMENT_STEP {
					self.last_move_time = current_time;

					match event {
						Event::KeyDown { keycode: Some(Keycode::Up), .. } => self.gameboard.rotate_piece(),
						 Event::KeyDown { keycode: Some(Keycode::Left), .. }=> self.gameboard.move_piece_left(),
						 Event::KeyDown { keycode: Some(Keycode::Right), .. } => self.gameboard.move_piece_right(),
						 _ => {}
					}
				}
			},
			
			Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
				self.move_down_fast = true;
			}
			Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
				self.move_down_fast = false;
			}
			
			Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
				self.running = false;
			},
			_ => {}
		}
	}
}