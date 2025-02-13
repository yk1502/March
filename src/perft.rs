use std::time::Instant;
use crate::board::Board;


const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
const POSITION3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
const POSITION4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
const POSITION5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ";
const POSITION6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ";


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

    //perft_pos.add_pos("position5".to_string(), POSITION5.to_string(), vec![1, 44, 1486, 62379, 2103487, 89941194]);
    //perft_pos.add_pos("position4".to_string(), POSITION4.to_string(), vec![1, 6, 264, 9467, 422333, 15833292, 706045033]);
    //perft_pos.add_pos("kiwipete".to_string(), KIWIPETE.to_string(), vec![1, 48, 2039, 97862, 4085603, 193690690, 8031647685]);
    //perft_pos.add_pos("position6".to_string(), POSITION6.to_string(), vec![1, 46, 2079, 89890, 3894594, 164075551, 6923051137]);
    //perft_pos.add_pos("position3".to_string(), POSITION3.to_string(), vec![1, 14, 191, 2812, 43238, 674624, 11030083, 178633661]);
    // perft_pos.add_pos("startpos".to_string(), STARTPOS.to_string(), vec![1, 20, 400, 8902, 197281, 4865609, 119060324, 3195901860, 84998978956]);


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