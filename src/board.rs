use crate::attack::{get_pawn_attack, get_knight_attack, get_bishop_attack, get_rook_attack, get_queen_attack, get_king_attack};
use crate::types::{Colour, Direction, Piece, PieceType, Square};
use crate::moves::{Move, MoveList};
use std::ops::{BitAnd, BitOr, Not};


#[derive(Copy, Clone, PartialEq)]
pub struct Bitboard {
    bitboard: u64,
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Bitboard::create(self.bitboard & rhs.bitboard)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Bitboard::create(self.bitboard | rhs.bitboard)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self {
        Bitboard::create(!self.bitboard)
    }
}

impl Bitboard {

    pub fn new() -> Self {
        Bitboard {bitboard: 0}
    }

    pub fn create(bb: u64) -> Self {
        Bitboard {bitboard: bb}
    }

    pub const fn create_const(bb: u64) -> Self {
        Bitboard {bitboard: bb}
    }

    pub fn display(&self) {
        for r in 0..8 {
            for f in 0..8 {

                if f == 0 {
                    print!("    ");
                }

                let square_index = r * 8 + f;

                let bit = if self.get_bit(Square::from_u8(square_index)) {
                    1
                } else {
                    0
                };

                print!("{}  ", bit);
            }
            println!();
        }
    }

    pub fn set_bit<T: Into<u8>>(&mut self, sq: T) {
        self.bitboard |= 1 << sq.into();
    }

    pub fn get_bit<T: Into<u8>>(&self, sq: T) -> bool {
        (self.bitboard & (1 << sq.into())) != 0  
    }

    pub fn pop_bit<T: Into<u8>>(&mut self, sq: T) {
        self.bitboard &= !(1 << sq.into())
    }

    pub fn get_lsb_index(&self) -> u8 {
        self.bitboard.trailing_zeros() as u8
    }

    pub fn pop_lsb(&mut self) -> u8 {
        let lsb = self.get_lsb_index();
        self.pop_bit(lsb);
        lsb
    }

    pub fn get_bit_count(&self) -> u8 {
        self.bitboard.count_ones() as u8
    }

    pub fn get_raw(&self) -> u64 {
        self.bitboard
    }

    pub fn is_empty(&self) -> bool {
        self.bitboard == 0
    }
}


#[derive(Clone, Copy)]
pub struct Board {
    pub bitboards: [Bitboard; 12], // DEBUG
    pub occupancies: [Bitboard; 3], 
    mailbox: [Piece; 64],
    castling: u8,
    ep_square: Square,
    side: Colour,
    opp_side: Colour,
}

impl Board {

    pub fn new() -> Self {
        Board {
            bitboards: [Bitboard::new(); 12],
            occupancies: [Bitboard::new(); 3],
            mailbox: [Piece::None; 64],
            castling: 0b0000,
            ep_square: Square::None,
            side: Colour::White,
            opp_side: Colour::Black,
        }
    }

    pub fn display_board(&self) {
        for sq in 0..64 {
            let file_index = sq % 8;
            let rank_index = 7 - (sq / 8);

            if file_index == 0 {
                print!("{}   ", rank_index + 1);
            }

            print!("{}  ", self.mailbox[sq].to_char());

            if (sq + 1) % 8 == 0 {
                println!();
            }
        }
        println!("\n    a  b  c  d  e  f  g  h\n");
        println!("   Castling Rights : {:04b}", self.castling);
        println!("   EP Square       : {}", self.ep_square.value());
        println!("   Side            : {}", self.side.to_str());
    }

    pub fn flip_side(&mut self) {
        (self.side, self.opp_side) = (self.opp_side, self.side);
    }

    pub fn parse_fen(&mut self, fen: &str) {
        let fen_parts: Vec<&str> = fen.split(' ').collect();

        // Parse side
        if fen_parts[1] == "w" {
            self.side = Colour::White;
            self.opp_side = Colour::Black;
        } else {
            self.side = Colour::Black;
            self.opp_side = Colour::White;
        }

        // Parse EP Square
        self.ep_square = Square::from_str(fen_parts[3]);

        // Parse castling rights
        for c in fen_parts[2].chars() {
            match c {
                'K' => self.castling |= 0b1000,
                'Q' => self.castling |= 0b0100,
                'k' => self.castling |= 0b0010,
                'q' => self.castling |= 0b0001,
                 _  => {},
            }
        }

        // Parse position
        let mut square_index: usize = 0;
        for c in fen_parts[0].chars() {
            
            if c.is_ascii_digit() {
                square_index += c.to_digit(10).unwrap() as usize;
                continue;
            }

            if c == '/' {
                continue;
            }

            let piece = Piece::from_char(c);
            let square = Square::from_u8(square_index as u8);

            self.mailbox[square_index] = piece;
            self.bitboards[piece.as_index()].set_bit(square);
            self.occupancies[piece.colour().as_index()].set_bit(square);

            square_index += 1;
        }

        self.occupancies[2] = self.occupancies[0] | self.occupancies[1];
    }

    pub fn is_square_occupied(&self, sq: Square) -> bool {
        self.occupancies[2].get_bit(sq)
    }

    pub fn can_king_castle(&self, colour: Colour) -> bool {
        if colour == Colour::White {
            (self.castling & 0b1000) != 0
        } else {
            (self.castling & 0b0010) != 0
        }
    }


    pub fn can_queen_castle(&self, colour: Colour) -> bool {
        if colour == Colour::White {
            (self.castling & 0b0100) != 0
        } else {
            (self.castling & 0b0001) != 0
        }
    }

    pub fn is_square_attacked(&self, sq: Square) -> bool {
        let opp_bb_offset = self.opp_side.as_index() * 6;

        !((get_pawn_attack(sq, self.side) & self.bitboards[opp_bb_offset]) |
        (get_knight_attack(sq) & self.bitboards[opp_bb_offset + PieceType::Knight.as_index()]) |
        (get_bishop_attack(sq, self.occupancies[2]) & self.bitboards[opp_bb_offset + PieceType::Bishop.as_index()]) |
        (get_rook_attack(sq, self.occupancies[2]) & self.bitboards[opp_bb_offset + PieceType::Rook.as_index()]) | 
        (get_queen_attack(sq, self.occupancies[2]) & self.bitboards[opp_bb_offset + PieceType::Queen.as_index()]) | 
        (get_king_attack(sq) & self.bitboards[opp_bb_offset + PieceType::King.as_index()])).is_empty()
    }

    pub fn in_check(&self) -> bool {
        let sq_index = self.bitboards[self.side.as_index() * 6 + 5].get_lsb_index();
        self.is_square_attacked(Square::from_u8(sq_index))
    }

    pub fn parse_move(&self, move_str: &str) -> Move {

        let source = Square::from_str(move_str.get(0..2).unwrap());
        let target = Square::from_str(move_str.get(2..4).unwrap());
        let mut promote = PieceType::None;

        let mut mv = Move::new();

        let ml = self.gen_moves();
        
        if move_str.len() == 5 {
            promote = Piece::from_char(move_str.chars().last().unwrap()).of_type();
            
            for i in 0..ml.move_count() {
                if ml.get_move(i).get_source() == source && ml.get_move(i).get_target() == target && ml.get_move(i).get_promote().of_type() == promote {
                    mv = ml.get_move(i);
                }
            }
        } else {
            for i in 0..ml.move_count() {
                if ml.get_move(i).get_source() == source && ml.get_move(i).get_target() == target {
                    mv = ml.get_move(i)
                }
            }
        }
        
        mv
    }

    pub fn gen_moves(&self) -> MoveList {

        let mut ml = MoveList::new();
        
        // Simple moves
        for (piece_idx, bb_iter) in self.bitboards.iter().enumerate() {

            let mut bb = *bb_iter;
            let piece = Piece::from_u8(piece_idx as u8);

            if self.side == Colour::White && piece.colour() == Colour::Black {
                break;
            } else if self.side == Colour::Black && piece.colour() == Colour::White {
                continue;
            }

            
            // Castling
            if piece.of_type() == PieceType::King {
                if self.side == Colour::White && !self.is_square_attacked(Square::E1) {
                    // White
                    let king_side_blockers = !self.is_square_occupied(Square::F1) && !self.is_square_occupied(Square::G1);
                    let queen_side_blockers = !self.is_square_occupied(Square::B1) && !self.is_square_occupied(Square::C1) && !self.is_square_occupied(Square::D1);
                    let king_path = !self.is_square_attacked(Square::F1);
                    let queen_path = !self.is_square_attacked(Square::C1) && !self.is_square_attacked(Square::D1);
                    
                    // King side
                    if self.can_king_castle(Colour::White) && king_side_blockers && king_path && self.bitboards[Piece::WR.as_index()].get_bit(Square::H1) {
                        ml.add_move(Move::create(Square::E1, Square::G1, Piece::WK, Piece::None, Piece::None, 0b1000, false, false));
                    }

                    // Queen side
                    if self.can_queen_castle(Colour::White) && queen_side_blockers && queen_path && self.bitboards[Piece::WR.as_index()].get_bit(Square::A1) {
                        ml.add_move(Move::create(Square::E1, Square::C1, Piece::WK, Piece::None, Piece::None, 0b0100, false, false));
                    }

                } else if self.side == Colour::Black && !self.is_square_attacked(Square::E8) {
                    // Black
                    let king_side_blockers = !self.is_square_occupied(Square::F8) && !self.is_square_occupied(Square::G8);
                    let queen_side_blockers = !self.is_square_occupied(Square::B8) && !self.is_square_occupied(Square::C8) && !self.is_square_occupied(Square::D8);
                    let king_path = !self.is_square_attacked(Square::F8);
                    let queen_path = !self.is_square_attacked(Square::C8) && !self.is_square_attacked(Square::D8);
                    
                    // King side
                    if self.can_king_castle(Colour::Black) && king_side_blockers && king_path && self.bitboards[Piece::BR.as_index()].get_bit(Square::H8) {
                        ml.add_move(Move::create(Square::E8, Square::G8, Piece::BK, Piece::None, Piece::None, 0b0010, false, false));
                    }

                    // Queen side
                    if self.can_queen_castle(Colour::Black) && queen_side_blockers && queen_path && self.bitboards[Piece::BR.as_index()].get_bit(Square::A8) {
                        ml.add_move(Move::create(Square::E8, Square::C8, Piece::BK, Piece::None, Piece::None, 0b0001, false, false));
                    }
                }
            }

            while !bb.is_empty() {
                let lsb = bb.pop_lsb();
                let lsb_sq = Square::from_u8(lsb);

                let mut attack_bb = match piece.of_type() {
                    PieceType::Pawn   => get_pawn_attack(lsb_sq, self.side) & self.occupancies[self.opp_side.as_index()],
                    PieceType::Knight => get_knight_attack(lsb_sq) & !self.occupancies[self.side.as_index()],
                    PieceType::Bishop => get_bishop_attack(lsb_sq, self.occupancies[2]) & !self.occupancies[self.side.as_index()],
                    PieceType::Rook   => get_rook_attack(lsb_sq, self.occupancies[2]) & !self.occupancies[self.side.as_index()],
                    PieceType::Queen  => get_queen_attack(lsb_sq, self.occupancies[2]) & !self.occupancies[self.side.as_index()],
                    PieceType::King   => get_king_attack(lsb_sq) & !self.occupancies[self.side.as_index()],
                    _ => Bitboard::new(),
                };

                if piece.of_type() == PieceType::Pawn {
                    // Pawn pushes
                    if !(lsb_sq.on_north_edge() || lsb_sq.on_south_edge() || piece.on_promote_square(lsb_sq)) {
                        let mut target_sq = lsb_sq.to_front(self.side);

                        // Single pawn push
                        if !self.is_square_occupied(target_sq) {
                            ml.add_move(Move::create(lsb_sq, target_sq, piece, Piece::None, Piece::None, 0, false, false));

                            // Double pawn push
                            target_sq = target_sq.to_front(self.side);
                            if piece.on_double_square(lsb_sq) && !self.is_square_occupied(target_sq) {
                                ml.add_move(Move::create(lsb_sq, target_sq, piece, Piece::None, Piece::None, 0, true, false));
                            }
                        }
                    }

                    // Pawn promotions
                    if piece.on_promote_square(lsb_sq) {
                        let target_sq = lsb_sq.to_front(self.side);

                        if !self.is_square_occupied(target_sq) {

                            let (queen, rook, bishop, knight) = if piece.colour() == Colour::White {
                                (Piece::WQ, Piece::WR, Piece::WB, Piece::WN)
                            } else {
                                (Piece::BQ, Piece::BR, Piece::BB, Piece::BN)
                            };

                            ml.add_move(Move::create(lsb_sq, target_sq, piece, queen, Piece::None, 0, false, false));
                            ml.add_move(Move::create(lsb_sq, target_sq, piece, rook, Piece::None, 0, false, false));
                            ml.add_move(Move::create(lsb_sq, target_sq, piece, bishop, Piece::None, 0, false, false));
                            ml.add_move(Move::create(lsb_sq, target_sq, piece, knight, Piece::None, 0, false, false));
                        }
                    }

                    // En Passant
                    if self.ep_square != Square::None && get_pawn_attack(lsb_sq, self.side).get_bit(self.ep_square) {
                        let capture_piece = self.mailbox[self.ep_square.to_front(self.opp_side).as_index()];
                        ml.add_move(Move::create(lsb_sq, self.ep_square, piece, Piece::None, capture_piece, 0, false, true));
                    }
                } 

                // Simple moves
                while !attack_bb.is_empty() {
                    let target_sq = Square::from_u8(attack_bb.pop_lsb());
                    let capture_piece = self.mailbox[target_sq.as_index()];
                    
                    if piece.of_type() == PieceType::Pawn {

                        // Pawn captures and pawn promotion captures
                        if piece.on_promote_square(lsb_sq) {
                            let (queen, rook, bishop, knight) = if piece.colour() == Colour::White {
                                (Piece::WQ, Piece::WR, Piece::WB, Piece::WN)
                            } else {
                                (Piece::BQ, Piece::BR, Piece::BB, Piece::BN)
                            };

                            ml.add_move(Move::create(lsb_sq, target_sq, piece, queen, capture_piece, 0, false, false));
                            ml.add_move(Move::create(lsb_sq, target_sq, piece, rook, capture_piece, 0, false, false));
                            ml.add_move(Move::create(lsb_sq, target_sq, piece, bishop, capture_piece, 0, false, false));
                            ml.add_move(Move::create(lsb_sq, target_sq, piece, knight, capture_piece, 0, false, false));
                            
                        } else {
                            ml.add_move(Move::create(lsb_sq, target_sq, piece, Piece::None, capture_piece, 0, false, false));
                        }
                        
                    } else {
                        ml.add_move(Move::create(lsb_sq, target_sq, piece, Piece::None, capture_piece, 0, false, false));
                    };                    
                }   
                
            }
        }

        ml
    }

    pub fn make_move(&mut self, mv: Move) -> bool {
        
        let source = mv.get_source();
        let target = mv.get_target();
        let piece = mv.get_piece();
        let promote = mv.get_promote();
        let castle = mv.get_castle();
        let capture_piece = mv.get_capture();

        self.bitboards[piece.as_index()].pop_bit(source);
        self.bitboards[piece.as_index()].set_bit(target);
        
        self.occupancies[self.side.as_index()].pop_bit(source);
        self.occupancies[self.side.as_index()].set_bit(target);

        self.mailbox[source.as_index()] = Piece::None;
        self.mailbox[target.as_index()] = piece;

        self.occupancies[2].pop_bit(source);
        self.occupancies[2].set_bit(target);

        if piece == Piece::WK {
            self.castling &= 0b0011;
        } else if piece == Piece::BK {
            self.castling &= 0b1100;
        }

        if piece == Piece::WR && source == Square::A1 {
            self.castling &= 0b1011;
        } else if piece == Piece::WR && source == Square::H1 {
            self.castling &= 0b0111;
        } else if piece == Piece::BR && source == Square::A8 {
            self.castling &= 0b1110;
        } else if piece == Piece::BR && source == Square::H8 {
            self.castling &= 0b1101;
        }

        if mv.is_capture() && !mv.is_ep() {
            self.bitboards[capture_piece.as_index()].pop_bit(target);
            self.occupancies[capture_piece.colour().as_index()].pop_bit(target);
        }

        if mv.is_ep() {
            let ep_captured_pawn = target.to_front(self.opp_side);
            self.bitboards[capture_piece.as_index()].pop_bit(ep_captured_pawn);
            self.occupancies[capture_piece.colour().as_index()].pop_bit(ep_captured_pawn);
            
            self.mailbox[ep_captured_pawn.as_index()] = Piece::None;
            self.occupancies[2].pop_bit(ep_captured_pawn);
        } 

        if mv.is_promote() {
            self.bitboards[piece.as_index()].pop_bit(target);
            self.bitboards[promote.as_index()].set_bit(target);
            
            self.mailbox[target.as_index()] = promote;
        }

        if mv.is_castle() {
            if castle == 0b1000 {
                self.bitboards[Piece::WR.as_index()].pop_bit(Square::H1);
                self.bitboards[Piece::WR.as_index()].set_bit(Square::F1);
                self.occupancies[0].pop_bit(Square::H1);
                self.occupancies[0].set_bit(Square::F1);

                self.mailbox[Square::H1.as_index()] = Piece::None;
                self.mailbox[Square::F1.as_index()] = Piece::WR;
                self.occupancies[2].pop_bit(Square::H1);
                self.occupancies[2].set_bit(Square::F1);
            } else if castle == 0b0100 {
                self.bitboards[Piece::WR.as_index()].pop_bit(Square::A1);
                self.bitboards[Piece::WR.as_index()].set_bit(Square::D1);
                self.occupancies[0].pop_bit(Square::A1);
                self.occupancies[0].set_bit(Square::D1);

                self.mailbox[Square::A1.as_index()] = Piece::None;
                self.mailbox[Square::D1.as_index()] = Piece::WR;
                self.occupancies[2].pop_bit(Square::A1);
                self.occupancies[2].set_bit(Square::D1);
            } else if castle == 0b0010 {
                self.bitboards[Piece::BR.as_index()].pop_bit(Square::H8);
                self.bitboards[Piece::BR.as_index()].set_bit(Square::F8);
                self.occupancies[1].pop_bit(Square::H8);
                self.occupancies[1].set_bit(Square::F8);

                self.mailbox[Square::H8.as_index()] = Piece::None;
                self.mailbox[Square::F8.as_index()] = Piece::BR;
                self.occupancies[2].pop_bit(Square::H8);
                self.occupancies[2].set_bit(Square::F8);
            } else if castle == 0b0001 {
                self.bitboards[Piece::BR.as_index()].pop_bit(Square::A8);
                self.bitboards[Piece::BR.as_index()].set_bit(Square::D8);
                self.occupancies[1].pop_bit(Square::A8);
                self.occupancies[1].set_bit(Square::D8);

                self.mailbox[Square::A8.as_index()] = Piece::None;
                self.mailbox[Square::D8.as_index()] = Piece::BR;
                self.occupancies[2].pop_bit(Square::A8);
                self.occupancies[2].set_bit(Square::D8);
            }
        }

        self.ep_square = Square::None;

        if mv.is_double() {
            if !target.on_west_edge() {
                let mut side_square = target.clone();
                side_square.shift(Direction::West);
                if self.bitboards[self.opp_side.as_index() * 6].get_bit(side_square) {
                    self.ep_square = source.to_front(self.side);
                }
            }

            if !target.on_east_edge() {
                let mut side_square = target.clone();
                side_square.shift(Direction::East);
                if self.bitboards[self.opp_side.as_index() * 6].get_bit(side_square) {
                    self.ep_square = source.to_front(self.side);
                }
            }
        }



        if self.in_check() {
            false
        } else {
            (self.side, self.opp_side) = (self.opp_side, self.side);
            true
        }

    }
}