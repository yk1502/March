use std::{mem::transmute, time::Instant};

use crate::moves::Move;




#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8, 
    A7, B7, C7, D7, E7, F7, G7, H7, 
    A6, B6, C6, D6, E6, F6, G6, H6, 
    A5, B5, C5, D5, E5, F5, G5, H5, 
    A4, B4, C4, D4, E4, F4, G4, H4, 
    A3, B3, C3, D3, E3, F3, G3, H3, 
    A2, B2, C2, D2, E2, F2, G2, H2, 
    A1, B1, C1, D1, E1, F1, G1, H1, 
    None
}

impl Into<u8> for Square {
    fn into(self) -> u8 {
        self.value()
    }
}

impl Square {
    pub fn as_index(&self) -> usize {
        *self as usize
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }

    pub fn from_u8(square_num: u8) -> Self {
        unsafe {transmute(square_num)}
    }

    pub fn shift(&mut self, step: Direction) {
        *self = Square::from_u8((*self as i16 + step.value()) as u8);
    }

    pub fn to_front(&self, side: Colour) -> Self {
        if side == Colour::White {
            Square::from_u8((*self as i16 + Direction::North.value()) as u8)
        } else {
            Square::from_u8((*self as i16 + Direction::South.value()) as u8)
        }
    }

    pub fn is_edge(&self) -> bool {
        let f = self.value() % 8;
        let r = self.value() / 8;

        f == 0 || f == 7 || r == 0 || r == 7
    }

    pub fn on_north_edge(&self) -> bool {
        self.value() / 8 == 0
    }

    pub fn on_south_edge(&self) -> bool {
        self.value() / 8 == 7
    }

    pub fn on_east_edge(&self) -> bool {
        self.value() % 8 == 7
    }

    pub fn on_west_edge(&self) -> bool {
        self.value() % 8 == 0
    }

    pub fn from_str(square_str: &str) -> Self {

        if square_str == "-" {
            Self::None
        } else {
            let mut square_chars = square_str.chars();
            let file_index = square_chars.next().unwrap_or('a') as u8 - b'a';
            let rank_index = 7 - (square_chars.next().unwrap_or('1') as u8 - b'1');
            let square_index = rank_index * 8 + file_index;

            unsafe {transmute(square_index)}
        }
        
    }

    pub fn to_str(&self) -> String {

        if *self == Self::None {
            String::from("None")
        } else {
            let square_index = *self as u8;
            let file_index = square_index % 8;
            let rank_index = 7 - (square_index / 8);
            
            let file = (file_index + b'a') as char;
            let rank = (rank_index + b'1') as char;

            format!("{}{}", file, rank)
        }
        
    }
}


#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Colour {
    White, Black, None
}

impl Colour {
    pub fn as_index(&self) -> usize {
        *self as usize
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }

    pub fn to_str(&self) -> String {
        if *self == Self::White {
            String::from("White")
        } else {
            String::from("Black")
        }
    }
}


#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn, Knight, Bishop, Rook, Queen, King, 
    None,
}

impl PieceType {
    pub fn from_u8(piece_num: u8) -> Self {
        unsafe {transmute(piece_num % 6)}
    }

    pub fn as_index(&self) -> usize {
        *self as usize
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}



#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    WP, WN, WB, WR, WQ, WK,
    BP, BN, BB, BR, BQ, BK,
    None
} 


impl Piece {

    pub fn as_index(&self) -> usize {
        *self as usize
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }

    pub fn from_u8(pc_index: u8) -> Self {
        unsafe {transmute(pc_index)}
    }

    pub fn of_type(&self) -> PieceType {
        PieceType::from_u8((*self as u8) % 6)
    }

    pub fn colour(&self) -> Colour {
        if (*self as u8) < 6 {
            Colour::White
        } else {
            Colour::Black
        }
    }

    pub fn on_promote_square(&self, sq: Square) -> bool {
        let colour = self.colour();
        let rank = sq.value() / 8;
        ((rank == 1) && (colour == Colour::White)) || ((rank == 6) && (colour == Colour::Black))
    }

    pub fn on_double_square(&self, sq: Square) -> bool {
        let colour = self.colour();
        let rank = sq.value() / 8;
        ((rank == 1) && (colour == Colour::Black)) || ((rank == 6) && (colour == Colour::White))
    }

    pub fn from_char(c: char) -> Self {
        match c {
            'P' => Self::WP,
            'N' => Self::WN,
            'B' => Self::WB,
            'R' => Self::WR,
            'Q' => Self::WQ,
            'K' => Self::WK,

            'p' => Self::BP,
            'n' => Self::BN,
            'b' => Self::BB,
            'r' => Self::BR,
            'q' => Self::BQ,
            'k' => Self::BK,

            _ => Self::None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::WP => 'P',
            Self::WN => 'N',
            Self::WB => 'B',
            Self::WR => 'R',
            Self::WQ => 'Q',
            Self::WK => 'K',

            Self::BP => 'p',
            Self::BN => 'n',
            Self::BB => 'b',
            Self::BR => 'r',
            Self::BQ => 'q',
            Self::BK => 'k',
            _ => '-',
        }
    }

}


#[repr(i16)]
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North = -8,
    East  =  1,
    South =  8,
    West  = -1,

    NE = -8 + 1,
    NW = -8 - 1,
    SE =  8 + 1,
    SW =  8 - 1,
}

impl Direction {
    pub fn as_index(&self) -> usize {
        *self as usize
    }

    pub fn value(&self) -> i16 {
        *self as i16
    }
}


#[derive(Copy, Clone)]
pub struct SearchInfo {
    start: Instant,
    time_left: u128,
    pub best_move: Move,
    pub ply: i32,
    pub nodes: i32,
    pub stop_early: bool,
}

impl SearchInfo {

    pub fn new(time_left: u128) -> Self {
        SearchInfo {
            ply: 0,
            best_move: Move::new(),
            start: Instant::now(),
            time_left: time_left,
            nodes: 0,
            stop_early: false,
        }
    }

    pub fn get_time(&self) -> u128 {
        self.start.elapsed().as_millis()
    }

    pub fn should_stop(&self) -> bool {
        self.get_time() > self.time_left
    }

    pub fn is_root(&self) -> bool {
        self.ply == 0
    }

    pub fn update(&mut self) {
        self.ply += 1;
        self.nodes += 1;
    }

    pub fn revert(&mut self) {
        self.ply -= 1;
    }
}

