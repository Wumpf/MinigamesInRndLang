use sdl2;
use rand;
use rand::Rng;

/// Definition of a tetris block.
#[derive(Clone, Copy, PartialEq)]
pub enum Block {
	Occupied(sdl2::pixels::Color),
	Empty,
}

/// Size of piece definition.
pub const PIECE_SIZE: usize = 4;
/// Piece consists of PIECE_SIZE by PIECE_SIZE blocks.
pub type Piece = [[Block; PIECE_SIZE]; PIECE_SIZE];


/// The shape is the same but without color - true means occupied, false is empty
type PieceShape = [[bool; PIECE_SIZE]; PIECE_SIZE];
/// Standard piece shapes.
const PIECE_SHAPES: [PieceShape; 6] = [
	// The block.
	[[false, false, false, false],
	 [false, true , true , false],
	 [false, true , true , false],
	 [false, false, false, false]],
	
	// The small z. 
    [[false, false, false, false],
	 [true , true , false, false],
	 [false, true , true , false],
	 [false, false, false, false]],
	
	// The big Z. 
    [[true , true , false, false],
	 [false, true , false, false],
	 [false, true , true , false],
	 [false, false, false, false]],
	 
	 // The L. 
    [[false, true , false, false],
	 [false, true , false, false],
	 [false, true , true , false],
	 [false, false, false, false]],
	 
	  // The mirrored L. 
    [[false, false, true, false],
	 [false, false, true, false],
	 [false, true , true , false],
	 [false, false, false, false]],
	 
	  // The long one. 
    [[false, true , false, false],
	 [false, true , false, false],
	 [false, true , false, false],
	 [false, true , false, false]],
];

pub fn generate_random_piece() -> Piece {
	let ref shape = PIECE_SHAPES[rand::thread_rng().gen_range(0, PIECE_SHAPES.len())];
	let mut piece = [[Block::Empty; PIECE_SIZE]; PIECE_SIZE];
	let color: sdl2::pixels::Color = rand::thread_rng().gen();
	
	for y in 0..PIECE_SIZE {
		for x in 0..PIECE_SIZE {
			if shape[y][x] == true {
				piece[y][x] = Block::Occupied(color);
			}
		}
	}
	
	// Zero to three rotations.
	let num_rots = rand::thread_rng().gen_range(0, 4); 
	for _ in 0..num_rots {
		piece = rotate_piece(&piece);
	}
	
	return piece;
}

pub fn rotate_piece(piece: &Piece) -> Piece {
	let mut new_piece = [[Block::Empty; PIECE_SIZE]; PIECE_SIZE];
	
	for y in 0..PIECE_SIZE {
		for x in 0..PIECE_SIZE {
			new_piece[PIECE_SIZE-1-x][y] = piece[y][x];			
		}
	}
	
	return new_piece;
}
	