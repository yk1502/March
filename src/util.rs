use crate::moves::Move;


pub const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
pub const POSITION3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
pub const POSITION4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
pub const POSITION5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ";
pub const POSITION6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ";


pub const MAX_SCORE: i32 = 50000;
pub const MATE_SCORE: i32 = 49000;


pub struct XorShift {
    state: u64,
}

impl XorShift {
    pub const fn new() -> Self {
        XorShift {
            state: 123456789,
        }
    }

    pub const fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state 
    }
}


pub fn display_uci_move(mv: Move) {
    print!("bestmove ");
    mv.display();
    println!();
}