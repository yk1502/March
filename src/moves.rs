use crate::types::{Square, Piece};



#[derive(Clone, Copy)]
pub struct Move {
    source: Square,
    target: Square,
    piece: Piece,
    promote: Piece,
    castle: u8,
    capture: Piece,
    double: bool,
    ep: bool,
}

impl Move {
    pub fn new() -> Self {
        Move {
            source: Square::None,
            target: Square::None,
            piece: Piece::None,
            promote: Piece::None,
            capture: Piece::None,
            castle: 0b0000,
            double: false,
            ep: false,
        }
    }

    pub fn create(source: Square, 
        target: Square, 
        piece: Piece, 
        promote: Piece, 
        capture: Piece, 
        castle: u8, 
        double: bool, 
        ep: bool) -> Self {
            Move {
                source: source,
                target: target,
                piece: piece,
                promote: promote,
                capture: capture,
                castle: castle,
                double: double,
                ep: ep,
            }
    }

    pub fn get_source(&self) -> Square {
        self.source 
    }

    pub fn get_target(&self) -> Square {
        self.target
    }

    pub fn get_piece(&self) -> Piece {
        self.piece
    }

    pub fn get_promote(&self) -> Piece {
        self.promote
    }

    pub fn get_capture(&self) -> Piece {
        self.capture
    }

    pub fn get_castle(&self) -> u8 {
        self.castle
    }

    pub fn is_castle(&self) -> bool {
        self.castle != 0
    }

    pub fn is_promote(&self) -> bool {
        self.promote != Piece::None
    }

    pub fn is_double(&self) -> bool {
        self.double
    }

    pub fn is_capture(&self) -> bool {
        self.capture != Piece::None
    }

    pub fn is_ep(&self) -> bool {
        self.ep
    }

    pub fn display(&self) {
        let source = self.source.to_str();
        let target = self.target.to_str();
        
        if self.is_promote() {
            print!("{}{}{}", source, target, self.get_promote().to_char().to_ascii_lowercase());
        } else {
            print!("{}{}", source, target);
        }
    }
  
}


#[derive(Clone, Copy)]
pub struct MoveList {
    pub moves: [Move; 256],
    count: usize,
}

impl MoveList {

    pub fn new() -> Self {
        MoveList {
            moves: [Move::new(); 256],
            count: 0,
        }
    }

    pub fn add_move(&mut self, mv: Move) {
        self.moves[self.count] = mv;
        self.count += 1;
    }

    pub fn pop_move(&mut self) {
        self.count -= 1;
    }

    pub fn move_count(&self) -> usize {
        self.count
    }

    pub fn get_move(&self, index: usize) -> Move {
        self.moves[index]
    }

    pub fn display(&self) {
        for i in 0..self.count {
            self.moves[i].display();
            println!();
        }
    }


}

