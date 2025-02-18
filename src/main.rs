
mod board;
mod types;
mod attack;
mod magic;
mod moves;
mod perft;
mod uci;
mod util;
mod search;


fn init() {
    attack::init_slider();    
}


fn start() {
    init();
    uci::uci_loop();
}


fn main() {
    start();
}




