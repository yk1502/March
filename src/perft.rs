use std::time::Instant;

use crate::board::Board;
use crate::util::{STARTPOS, KIWIPETE, POSITION3, POSITION4, POSITION5, POSITION6};

struct PerftPos {
    fens: Vec<String>,
    depths: Vec<Vec<u128>>,
    names: Vec<String>,
    count: usize,
}

impl PerftPos {

    pub fn new() -> Self {
        PerftPos {
            fens: Vec::new(),
            depths: Vec::new(),
            names: Vec::new(), 
            count: 0
        }
    }

    pub fn add_pos(&mut self, name: String, fen: String, depths: Vec<u128>) {
        self.names.push(name);
        self.fens.push(fen);
        self.depths.push(depths);
        self.count += 1;
    }
}




pub fn perft(board: Board, depth: u32, ply: u32, nodes: &mut u128, move_nodes: bool) {

    if depth == 0 {
        *nodes += 1;
        return;
    }

    let ml = board.gen_moves();
    let mut cumulative = 0;

    for i in 0..ml.move_count() {
        let mv = ml.get_move(i);
        let mut new_board = board.clone();

        if !new_board.make_move(mv) {
            continue;
        }

        perft(new_board, depth - 1, ply + 1, nodes, move_nodes);

        if ply == 0 && move_nodes {
            cumulative = *nodes - cumulative;
            mv.display();
            println!(" - {}", cumulative);
            cumulative = *nodes;
        }
        
    }
}



pub fn run_perft(fen: &str, depth: u32, debug: bool) {
    let mut board = Board::new();
    board.parse_fen(fen);

    if debug {
        let mut nodes = 0;
    
        let start = Instant::now();
        perft(board, depth, 0, &mut nodes, true);
        let duration = start.elapsed();

        let nps = ((nodes as f64) / (duration.as_secs_f64())) / 1_000_000 as f64;

        println!("Depth : {} | Nodes : {} | NPS : {} mnps", depth, nodes, nps);
    } else {
        for d in 0..=depth {
            let mut nodes = 0;
    
            let start = Instant::now();
            perft(board, d, 0, &mut nodes, false);
            let duration = start.elapsed();
    
            let nps = ((nodes as f64) / (duration.as_secs_f64())) / 1_000_000 as f64;
    
            println!("Depth : {} | Nodes : {} | NPS : {} mnps", d, nodes, nps);
        }
    }
    
    
}




pub fn perft_suite() {
    let mut perft_pos = PerftPos::new();

    perft_pos.add_pos("position5".to_string(), POSITION5.to_string(), vec![1, 44, 1486, 62379, 2103487, 89941194]);
    perft_pos.add_pos("position4".to_string(), POSITION4.to_string(), vec![1, 6, 264, 9467, 422333, 15833292, 706045033]);
    perft_pos.add_pos("kiwipete".to_string(), KIWIPETE.to_string(), vec![1, 48, 2039, 97862, 4085603, 193690690, 8031647685]);
    perft_pos.add_pos("position6".to_string(), POSITION6.to_string(), vec![1, 46, 2079, 89890, 3894594, 164075551, 6923051137]);
    perft_pos.add_pos("position3".to_string(), POSITION3.to_string(), vec![1, 14, 191, 2812, 43238, 674624, 11030083, 178633661]);
    perft_pos.add_pos("startpos".to_string(), STARTPOS.to_string(), vec![1, 20, 400, 8902, 197281, 4865609, 119060324, 3195901860, 84998978956]);


    for i in 0..perft_pos.count {

        println!("\nRunning test {} to depth {}", perft_pos.names[i], perft_pos.depths[i].len());

        let mut board = Board::new();
        board.parse_fen(perft_pos.fens[i].as_str());
        let mut passer = 0;

        for d in 0..perft_pos.depths[i].len() {
            let mut nodes = 0;
    
            perft(board, d as u32, 0, &mut nodes, false);
            
            if nodes != perft_pos.depths[i][d] {
                break;
            }

            passer = d
        }

        if passer != perft_pos.depths[i].len() - 1 {
            println!("Test {} failed at depth {}.", perft_pos.names[i], passer);
        } else {
            println!("Test {} passed", perft_pos.names[i]);
        }
    }
}