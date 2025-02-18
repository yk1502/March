use std::thread;
use std::time::Duration;

use crate::board::Board;
use crate::moves::MoveList;
use crate::util::gen_rand;


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
    thread::sleep(Duration::from_secs_f32(0.01));
    print!("bestmove ");
    valid_ml.get_move(gen_rand() as usize % valid_ml.move_count()).display();
    println!();
}