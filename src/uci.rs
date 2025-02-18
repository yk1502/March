use std::io;

use crate::util::STARTPOS;
use crate::board::Board;
use crate::search::get_rand_move;



fn isready(cmd: &str) {
    if cmd.trim() == "isready" {
        println!("readyok");
    }
}


fn uci(cmd: &str) {
    if cmd.trim() == "uci" {
        println!("option name Hash type spin default 1 min 1 max 1");
        println!("option name Threads type spin default 1 min 1 max 1");
        println!("uciok");
    }
}


fn ucinewgame(cmd: &str, board: &mut Board) {
    if cmd.trim() == "ucinewgame" {
        board.parse_fen(STARTPOS);
    }
}


fn position(cmd: &str, board: &mut Board) {
    let split_cmd: Vec<&str> = cmd.trim().split(" ").collect();
    let mut fen = String::new();

    // Set startpos
    if cmd.contains("startpos") {
        board.parse_fen(STARTPOS);
    }

    // Collect fen
    if let Some(fen_idx) = split_cmd.iter().position(|&x| x == "fen") {
        
        for i in (fen_idx + 1)..split_cmd.len() {

            if split_cmd[i] == "moves" {
                break;
            }

            fen.push_str(split_cmd[i]);
            fen.push_str(" ");
        }

        board.parse_fen(fen.as_str().trim());
    }

    
    if let Some(moves_idx) = split_cmd.iter().position(|&x| x == "moves") {

        // Push moves on board
        for i in (moves_idx + 1)..split_cmd.len() {
            board.make_move(board.parse_move(split_cmd[i].trim()));
        }
    }
}


fn go(cmd: &str, board: Board) {
    let split_cmd: Vec<&str> = cmd.trim().split(" ").collect();

    if split_cmd[0] == "go" {
        get_rand_move(&board);
    }
}


fn display(cmd: &str, board: &Board) {
    if cmd.trim() == "display" {
        board.display_board();
    }
}






pub fn uci_loop() {

    println!("March by yk1502");
    let mut board = Board::new();
    
    loop {
        let mut raw_cmd = String::new();
        io::stdin().read_line(&mut raw_cmd).expect("Failed");
        let cmd = raw_cmd.as_str().trim();

        if cmd == "quit" {
            break;
        }

        isready(cmd);
        uci(cmd);
        ucinewgame(cmd, &mut board);
        position(cmd, &mut board);
        go(cmd, board);
        display(cmd, &board);
    }

}