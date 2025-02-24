use crate::attack::{gen_bishop_attacks, gen_rook_attacks, BISHOP_MASK, ROOK_MASK};
use crate::board::Bitboard;
use crate::types::Square;
use crate::util::XorShift;




pub const BISHOP_MAGICS: [u64; 64] = [
    0x2843503000850040, 0xa00282440c018840, 0x8110078081000040, 0x400430520000a008, 
    0x0214042000080004, 0x2042050421420008, 0x0402010128400448, 0x0082060904020200, 
    0x1018080208020400, 0x0005092220820200, 0x8430100404404010, 0x1000044040804100, 
    0x0000020210044040, 0x9215009010080229, 0x0040004410041100, 0x1104210088440200, 
    0x000a802008012800, 0x0c91601230150100, 0x00300c8200220220, 0xa842000120820000, 
    0x0001000090400020, 0x000a000122100208, 0x041040042c020800, 0x0050a04109180201, 
    0x0044101860a00190, 0x404108210c100408, 0x8001030450040021, 0x00e10c0008040810, 
    0x0420020040c05000, 0x000144108a010400, 0x880141080a08012c, 0x0010430100430800, 
    0x0030100820100204, 0x0214010402200444, 0x141014020010098a, 0x0242004040040100, 
    0x80500200100a0900, 0x0010100040402409, 0x01130124021a1204, 0x0000810a06110082, 
    0x8020980440001105, 0xc4120a062020a201, 0xd001840048000300, 0x0000004010402600, 
    0x0000011122004402, 0x410200b00c800102, 0x010483080b000203, 0x00101c00408001c1, 
    0xc000840108408808, 0x00a060aa10100600, 0x9180330401040000, 0x0080084a42120024, 
    0x2000002004240000, 0x0a00480a28060000, 0x2008200441a20800, 0x0408100402404000, 
    0x0000804804900800, 0x0000810080908800, 0x30010001a2011090, 0x8000440024840410, 
    0x0410112070020200, 0x00a00020820a0210, 0x40201004f80800c0, 0x000850010a441180, 
];

pub const ROOK_MAGICS: [u64; 64] = [
    0x0080002290400280, 0x014010004000a009, 0x008008100280a000, 0x010008100063000c, 
    0x0480040080023800, 0x2c0088202c001006, 0x0100050000945200, 0x0480014264800100, 
    0x0042800180204000, 0x0003004000210080, 0x4000801004200080, 0x0009003820100100, 
    0xb010800400800800, 0x0806000510880200, 0x0002000804010200, 0x04010002028a6700, 
    0x20022180018c4000, 0x8020004005245001, 0x080a898020001000, 0x0008808008045001, 
    0x0002050011000800, 0x100a008002800400, 0x0020040041100288, 0x800882000488510c, 
    0x4000982080014005, 0x1000410200208601, 0x0010500080600180, 0x0040080080100080, 
    0x0004000480800800, 0x000d040801204030, 0x0043000100040a00, 0x0010010200004094, 
    0x0020204010800280, 0x1810042000404004, 0x0030100080806000, 0x8842100080800806, 
    0x4000800800800400, 0x0102001002000904, 0x0228081004000102, 0x0020008402000041, 
    0x8404804000208000, 0x0032201004404000, 0xc000200041010052, 0xc0202042008a0011, 
    0x0003001108010004, 0x0a02000810020084, 0x200e000409420028, 0x0002040044860001, 
    0x3005210c80064300, 0x0008204001008100, 0x2200803000200080, 0x4040200903900100, 
    0x4063040008008080, 0x0002020080040080, 0x00010002000c8100, 0x0002010044048200, 
    0xa201079680002041, 0x0041084080102202, 0x0042018028209042, 0x2c0100500004a049, 
    0x4002002014481002, 0x041100080a040001, 0x020900088200040b, 0xa55010240080c102, 
];






pub const BISHOP_REL: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    6, 5, 5, 5, 5, 5, 5, 6, 
];
 

pub const ROOK_REL: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12, 
];




 





pub fn gen_occ(index: u64, mut mask: Bitboard) -> Bitboard {
    
    let mut occ = Bitboard::new();
    let index_bb = Bitboard::create(index);
    let mut index_bit = 0;

    while !mask.is_empty() {
        let lsb = mask.pop_lsb();
        
        if index_bb.get_bit(index_bit) {
            occ.set_bit(lsb);
        }

        index_bit += 1;
    }

    occ
}


pub fn gen_magic() {
    let mut bishop_attacks: [[Bitboard; 512]; 64] = [[Bitboard::new(); 512]; 64];
    let mut rook_attacks: [[Bitboard; 4096]; 64] = [[Bitboard::new(); 4096]; 64];

    let mut state = XorShift::new();

    println!("Generating bishop magics...");
    let mut sq = 0;
    while sq != 64 {
        bishop_attacks[sq as usize] = [Bitboard::new(); 512];
        let magic = state.next() & state.next() & state.next();
        let mut success = true;
        for i in 0..512 {
            let occ = gen_occ(i as u64, BISHOP_MASK[sq]);
            let attack = gen_bishop_attacks(Square::from_u8(sq as u8), occ);
            let magic_index = occ.get_raw().wrapping_mul(magic) >> (64 - BISHOP_REL[sq as usize]);
            
            if bishop_attacks[sq as usize][magic_index as usize].is_empty() {
                bishop_attacks[sq as usize][magic_index as usize] = attack;
            } else if bishop_attacks[sq as usize][magic_index as usize] != attack {
                success = false;
                break;
            }
        }

        if success {
            print!("0x{:016x}, ", magic);

            if (sq + 1) % 4 == 0 {
                println!();
            }

            sq += 1;
        }
    }
    
        
    
    
    println!("\nGenerating rook magics...");
    let mut sq = 0;
    while sq != 64 {
        rook_attacks[sq as usize] = [Bitboard::new(); 4096];
        let magic = state.next() & state.next() & state.next();
        let mut success = true;
        for i in 0..4096 {
            let occ = gen_occ(i as u64, ROOK_MASK[sq]);
            let attack = gen_rook_attacks(Square::from_u8(sq as u8), occ);
            let magic_index = occ.get_raw().wrapping_mul(magic) >> (64 - ROOK_REL[sq as usize]);
            
            if rook_attacks[sq as usize][magic_index as usize].is_empty(){
                rook_attacks[sq as usize][magic_index as usize] = attack;
            } else if rook_attacks[sq as usize][magic_index as usize] != attack {
                success = false;
                break;
            }
        }

        if success {
            print!("0x{:016x}, ", magic);

            if (sq + 1) % 4 == 0 {
                println!();
            }

            sq += 1;
        }
    }

    
}