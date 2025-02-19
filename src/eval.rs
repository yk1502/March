use crate::board::Board;
use crate::types::{Piece, Colour};



pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0;

    score += board.get_piece_count(Piece::WP) as i32 * 100;
    score += board.get_piece_count(Piece::WN) as i32 * 300;
    score += board.get_piece_count(Piece::WB) as i32 * 350;
    score += board.get_piece_count(Piece::WR) as i32 * 500;
    score += board.get_piece_count(Piece::WQ) as i32 * 900;

    score -= board.get_piece_count(Piece::BP) as i32 * 100;
    score -= board.get_piece_count(Piece::BN) as i32 * 300;
    score -= board.get_piece_count(Piece::BB) as i32 * 350;
    score -= board.get_piece_count(Piece::BR) as i32 * 500;
    score -= board.get_piece_count(Piece::BQ) as i32 * 900;

    if board.get_side() == Colour::White {
        score 
    } else {
        -score
    }
}