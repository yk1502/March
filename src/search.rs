use std::thread;
use std::time::Duration;

use crate::board::Board;
use crate::moves::{Move, MoveList};
use crate::types::SearchInfo;
use crate::util::{display_uci_move, gen_rand, MAX_SCORE, MATE_SCORE};
use crate::eval::evaluate;




pub fn get_rand_move(board: &Board) {
    let ml = board.gen_moves();
    let mut valid_ml = MoveList::new();

    for i in 0..ml.move_count() {
        let mv = ml.get_move(i);
        let mut clone_board = board.clone();

        if clone_board.make_move(mv) {
            valid_ml.add_move(mv);
        }
    }

    println!("info score 0");
    thread::sleep(Duration::from_secs_f32(0.00));
    display_uci_move(valid_ml.get_move(gen_rand() as usize % valid_ml.move_count()));
}


pub fn negamax(board: &Board, depth: i32, mut alpha: i32, mut beta: i32, si: &mut SearchInfo) -> i32 {

    if depth == 0 {
        return evaluate(board);
    }

    if si.should_stop() {
        si.stop_early = true;
        return 0;
    }
    
    let ml = board.gen_moves();
    let mut best_score = -MAX_SCORE;

    for i in 0..ml.move_count() {

        let mv = ml.get_move(i);
        let mut new_board = board.clone();

        if !new_board.make_move(mv) {
            continue;
        }

        si.update();
        let score = -negamax(&new_board, depth - 1, -beta, -alpha, si);
        si.revert();

        if score > best_score {
            best_score = score;

            if si.is_root() {
                si.best_move = mv;
            }

            if score > alpha {
                alpha = score;

                if alpha >= beta {
                    break;
                }
            }
        }
    }

    if best_score == -MAX_SCORE {
        if board.in_check() {
            return -MATE_SCORE + si.ply;
        } else {
            return 0;
        }
    }

    best_score
}


pub fn search_pos(board: Board, stm_time: u32, stm_inc: u32) {
    let time_left = (stm_time / 20) + (stm_inc / 2);
    let mut si = SearchInfo::new(time_left as u128);
    let mut best_move = Move::new();

    for d in 1..256 {
        let score = negamax(&board, d, -MAX_SCORE, MAX_SCORE, &mut si);

        if si.stop_early {
            break;
        }

        best_move = si.best_move;
        println!("info depth {} nodes {} time {} score cp {}", d, si.nodes, si.get_time(), score);
    }

    display_uci_move(best_move);
}