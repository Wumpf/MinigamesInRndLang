use std::cmp;

use sdl2;
use sdl2::rect::Point;

use blocks::*;


const NUM_BLOCKS_X: usize = 10;
const NUM_BLOCKS_Y: usize = 20;

/// Padding for block rendering at the borders in percent.
const BLOCK_RENDER_PADDING: f32 = 0.1;

pub struct Gameboard {
	fields: [[Block; NUM_BLOCKS_X]; NUM_BLOCKS_Y],
	piece: Option<Piece>,
	piece_position: (i32, i32)
}

impl Gameboard {
	pub fn new() -> Gameboard{
		Gameboard {
			fields: [[Block::Empty; NUM_BLOCKS_X]; NUM_BLOCKS_Y],
			piece: Option::None,
			piece_position: (0, 0)
		}
	}
	
	// Draws the gameboard.
	pub fn draw(&self, renderer: &mut sdl2::render::Renderer, window_width: u32, window_height: u32) {
		// Compute field extents.
		let block_size_u = cmp::min(window_width / NUM_BLOCKS_X as u32, window_height / NUM_BLOCKS_Y as u32);
		let block_size_i = block_size_u as i32;
		let top_left = Point::new(cmp::max(0, (window_width - block_size_u * NUM_BLOCKS_X as u32) / 2) as i32,
								  cmp::max(0, (window_height - block_size_u * NUM_BLOCKS_Y as u32) / 2) as i32);
		let bottom_right = Point::new(top_left.x() + block_size_i * NUM_BLOCKS_X as i32, top_left.y() + block_size_i * NUM_BLOCKS_Y as i32);
		let padding = (block_size_u as f32 * BLOCK_RENDER_PADDING) as u32;
		
		// Draw field background.
		renderer.set_draw_color(sdl2::pixels::Color::RGB(112, 128, 144));
		renderer.fill_rect(sdl2::rect::Rect::new_unwrap(top_left.x(), top_left.y(), (bottom_right.x() - top_left.x()) as u32, (bottom_right.y() - top_left.y()) as u32));
		
		// Lines
		renderer.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100));
		for x in 0..NUM_BLOCKS_X+1 {
			let start = Point::new(top_left.x() + x as i32 * block_size_i, top_left.y());
			renderer.draw_line(start, Point::new(start.x(), bottom_right.y()));
		}
		for y in 0..NUM_BLOCKS_Y+1 {
			let start = Point::new(top_left.x(), top_left.y() + y as i32 * block_size_i);
			renderer.draw_line(start, Point::new(bottom_right.x(), start.y()));
		}
		
		// Draw the piece.
		if self.piece.is_some() == true {
			let ref piece = self.piece.unwrap();
			
			for y in 0..PIECE_SIZE {
				for x in 0..PIECE_SIZE {
					match piece[y][x] {
						Block::Occupied(color) => {
							let pos = (x as i32 + self.piece_position.0, y as i32 + self.piece_position.1);
							renderer.set_draw_color(color); // Changing the color with every draw is unnecessary since the piece is single colored, but 
							renderer.fill_rect(sdl2::rect::Rect::new_unwrap(top_left.x() + pos.0 * block_size_i + padding as i32, 
																			top_left.y() + pos.1 * block_size_i + padding as i32,
																			block_size_u - padding*2, block_size_u - padding*2));
						}
						_ => {}
					}
				}
			}
		}
		
		// Draw the rest of the gameboard.
		for y in 0..NUM_BLOCKS_Y {
			for x in 0..NUM_BLOCKS_X {
				match self.fields[y][x] {
					Block::Occupied(color) => {
						renderer.set_draw_color(color); // Changing the color with every draw is unnecessary since the piece is single colored, but 
						renderer.fill_rect(sdl2::rect::Rect::new_unwrap(top_left.x() + x as i32 * block_size_i + padding as i32, 
																		top_left.y() + y as i32 * block_size_i + padding as i32,
																		block_size_u - padding*2, block_size_u - padding*2));
					}
					_ => {}
				}
			}
		}
	}
	
	// Inserts a new piece. Overwrites old piece.
	pub fn insert_piece(&mut self, piece: Piece) {
		self.piece = Some(piece);
		self.piece_position = (NUM_BLOCKS_X as i32 / 2, -(PIECE_SIZE as i32 / 2));
	}
	// Moves the active piece to the left. Does nothing if there is no active piece.
	pub fn move_piece_left(&mut self) {
		self.piece_position.0 -= 1;
		if self.is_piece_valid() == false {
			self.piece_position.0 += 1;	
		}
	}
	/// Moves the active piece to the right. Does nothing if there is no active piece.
	pub fn move_piece_right(&mut self) {
		self.piece_position.0 += 1;
		if self.is_piece_valid() == false {
			self.piece_position.0 -= 1;	
		}
	}
	
	/// Removes all finished horizontal lines.
	pub fn erase_filled_lines(&mut self) {	
	}
	
	/// Current piece falls down by a single step. If it cant move further, its blocks will be added to the field.
	pub fn fall_down(&mut self) {
		if self.piece.is_some() == true {
			self.piece_position.1 += 1;
			if self.is_piece_valid() == false {
				self.piece_position.1 -= 1;
				
				// Insert into the static field list.
				let ref piece = self.piece.unwrap();
				for y in 0..PIECE_SIZE {
					for x in 0..PIECE_SIZE {
						if piece[y][x] != Block::Empty {
							let pos = (x as i32 + self.piece_position.0, y as i32 + self.piece_position.1);
							if pos.0 >= 0 && pos.1 >= 0 && pos.0 < NUM_BLOCKS_X as i32 && pos.1 < NUM_BLOCKS_Y as i32 {
								self.fields[pos.1 as usize][pos.0 as usize] = piece[y][x];
							}
						}
					}
				}
				
				self.piece = Option::None;
			}
		}
	}
	
	/// Checks weather the current piece configuration is valid or not.
	fn is_piece_valid(&self) -> bool {
		if self.piece.is_some() {
			let ref piece = self.piece.unwrap();
			for y in 0..PIECE_SIZE {
				for x in 0..PIECE_SIZE {
					if piece[y][x] != Block::Empty {
						let pos = Point::new(x as i32 + self.piece_position.0, y as i32 + self.piece_position.1);
						
						// Ignore outside upwards
						if pos.y() < 0 {
							continue;
						}
						// Outside the field.
						if pos.x() < 0 || pos.x() >= NUM_BLOCKS_X as i32 || pos.y() >= NUM_BLOCKS_Y as i32 {
							return false;
						}
						// Touching a solid block.
						if self.fields[pos.y() as usize][pos.x() as usize] != Block::Empty {
							return false;
						}
					}
				}
			}
		}
		return true;
	}
	
	pub fn rotate_piece(&self) {
		if self.piece.is_some() {
			// TODO
		}
	}
	
	pub fn has_active_piece(&self) -> bool {
		self.piece.is_some()
	}
}