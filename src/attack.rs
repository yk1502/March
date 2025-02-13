use crate::magic::{gen_occ, BISHOP_MAGICS, BISHOP_REL, ROOK_MAGICS, ROOK_REL};
use crate::types::{Colour, Direction, Square};
use crate::board::Bitboard;




const PAWN: [[Bitboard; 64]; 2] = [
    [Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000), 
    Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000), 
    Bitboard::create_const(0x0000000000000002), Bitboard::create_const(0x0000000000000005), Bitboard::create_const(0x000000000000000a), Bitboard::create_const(0x0000000000000014), 
    Bitboard::create_const(0x0000000000000028), Bitboard::create_const(0x0000000000000050), Bitboard::create_const(0x00000000000000a0), Bitboard::create_const(0x0000000000000040), 
    Bitboard::create_const(0x0000000000000200), Bitboard::create_const(0x0000000000000500), Bitboard::create_const(0x0000000000000a00), Bitboard::create_const(0x0000000000001400), 
    Bitboard::create_const(0x0000000000002800), Bitboard::create_const(0x0000000000005000), Bitboard::create_const(0x000000000000a000), Bitboard::create_const(0x0000000000004000), 
    Bitboard::create_const(0x0000000000020000), Bitboard::create_const(0x0000000000050000), Bitboard::create_const(0x00000000000a0000), Bitboard::create_const(0x0000000000140000), 
    Bitboard::create_const(0x0000000000280000), Bitboard::create_const(0x0000000000500000), Bitboard::create_const(0x0000000000a00000), Bitboard::create_const(0x0000000000400000), 
    Bitboard::create_const(0x0000000002000000), Bitboard::create_const(0x0000000005000000), Bitboard::create_const(0x000000000a000000), Bitboard::create_const(0x0000000014000000), 
    Bitboard::create_const(0x0000000028000000), Bitboard::create_const(0x0000000050000000), Bitboard::create_const(0x00000000a0000000), Bitboard::create_const(0x0000000040000000), 
    Bitboard::create_const(0x0000000200000000), Bitboard::create_const(0x0000000500000000), Bitboard::create_const(0x0000000a00000000), Bitboard::create_const(0x0000001400000000), 
    Bitboard::create_const(0x0000002800000000), Bitboard::create_const(0x0000005000000000), Bitboard::create_const(0x000000a000000000), Bitboard::create_const(0x0000004000000000), 
    Bitboard::create_const(0x0000020000000000), Bitboard::create_const(0x0000050000000000), Bitboard::create_const(0x00000a0000000000), Bitboard::create_const(0x0000140000000000), 
    Bitboard::create_const(0x0000280000000000), Bitboard::create_const(0x0000500000000000), Bitboard::create_const(0x0000a00000000000), Bitboard::create_const(0x0000400000000000), 
    Bitboard::create_const(0x0002000000000000), Bitboard::create_const(0x0005000000000000), Bitboard::create_const(0x000a000000000000), Bitboard::create_const(0x0014000000000000), 
    Bitboard::create_const(0x0028000000000000), Bitboard::create_const(0x0050000000000000), Bitboard::create_const(0x00a0000000000000), Bitboard::create_const(0x0040000000000000),],

    [Bitboard::create_const(0x0000000000000200), Bitboard::create_const(0x0000000000000500), Bitboard::create_const(0x0000000000000a00), Bitboard::create_const(0x0000000000001400), 
    Bitboard::create_const(0x0000000000002800), Bitboard::create_const(0x0000000000005000), Bitboard::create_const(0x000000000000a000), Bitboard::create_const(0x0000000000004000), 
    Bitboard::create_const(0x0000000000020000), Bitboard::create_const(0x0000000000050000), Bitboard::create_const(0x00000000000a0000), Bitboard::create_const(0x0000000000140000), 
    Bitboard::create_const(0x0000000000280000), Bitboard::create_const(0x0000000000500000), Bitboard::create_const(0x0000000000a00000), Bitboard::create_const(0x0000000000400000), 
    Bitboard::create_const(0x0000000002000000), Bitboard::create_const(0x0000000005000000), Bitboard::create_const(0x000000000a000000), Bitboard::create_const(0x0000000014000000), 
    Bitboard::create_const(0x0000000028000000), Bitboard::create_const(0x0000000050000000), Bitboard::create_const(0x00000000a0000000), Bitboard::create_const(0x0000000040000000), 
    Bitboard::create_const(0x0000000200000000), Bitboard::create_const(0x0000000500000000), Bitboard::create_const(0x0000000a00000000), Bitboard::create_const(0x0000001400000000), 
    Bitboard::create_const(0x0000002800000000), Bitboard::create_const(0x0000005000000000), Bitboard::create_const(0x000000a000000000), Bitboard::create_const(0x0000004000000000), 
    Bitboard::create_const(0x0000020000000000), Bitboard::create_const(0x0000050000000000), Bitboard::create_const(0x00000a0000000000), Bitboard::create_const(0x0000140000000000), 
    Bitboard::create_const(0x0000280000000000), Bitboard::create_const(0x0000500000000000), Bitboard::create_const(0x0000a00000000000), Bitboard::create_const(0x0000400000000000), 
    Bitboard::create_const(0x0002000000000000), Bitboard::create_const(0x0005000000000000), Bitboard::create_const(0x000a000000000000), Bitboard::create_const(0x0014000000000000), 
    Bitboard::create_const(0x0028000000000000), Bitboard::create_const(0x0050000000000000), Bitboard::create_const(0x00a0000000000000), Bitboard::create_const(0x0040000000000000), 
    Bitboard::create_const(0x0200000000000000), Bitboard::create_const(0x0500000000000000), Bitboard::create_const(0x0a00000000000000), Bitboard::create_const(0x1400000000000000), 
    Bitboard::create_const(0x2800000000000000), Bitboard::create_const(0x5000000000000000), Bitboard::create_const(0xa000000000000000), Bitboard::create_const(0x4000000000000000), 
    Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000), 
    Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000), Bitboard::create_const(0x0000000000000000),]
];

const KNIGHT: [Bitboard; 64] = [
    Bitboard::create_const(0x0000000000020400), Bitboard::create_const(0x0000000000050800), Bitboard::create_const(0x00000000000a1100), Bitboard::create_const(0x0000000000142200), 
    Bitboard::create_const(0x0000000000284400), Bitboard::create_const(0x0000000000508800), Bitboard::create_const(0x0000000000a01000), Bitboard::create_const(0x0000000000402000), 
    Bitboard::create_const(0x0000000002040004), Bitboard::create_const(0x0000000005080008), Bitboard::create_const(0x000000000a110011), Bitboard::create_const(0x0000000014220022), 
    Bitboard::create_const(0x0000000028440044), Bitboard::create_const(0x0000000050880088), Bitboard::create_const(0x00000000a0100010), Bitboard::create_const(0x0000000040200020), 
    Bitboard::create_const(0x0000000204000402), Bitboard::create_const(0x0000000508000805), Bitboard::create_const(0x0000000a1100110a), Bitboard::create_const(0x0000001422002214), 
    Bitboard::create_const(0x0000002844004428), Bitboard::create_const(0x0000005088008850), Bitboard::create_const(0x000000a0100010a0), Bitboard::create_const(0x0000004020002040), 
    Bitboard::create_const(0x0000020400040200), Bitboard::create_const(0x0000050800080500), Bitboard::create_const(0x00000a1100110a00), Bitboard::create_const(0x0000142200221400), 
    Bitboard::create_const(0x0000284400442800), Bitboard::create_const(0x0000508800885000), Bitboard::create_const(0x0000a0100010a000), Bitboard::create_const(0x0000402000204000), 
    Bitboard::create_const(0x0002040004020000), Bitboard::create_const(0x0005080008050000), Bitboard::create_const(0x000a1100110a0000), Bitboard::create_const(0x0014220022140000), 
    Bitboard::create_const(0x0028440044280000), Bitboard::create_const(0x0050880088500000), Bitboard::create_const(0x00a0100010a00000), Bitboard::create_const(0x0040200020400000), 
    Bitboard::create_const(0x0204000402000000), Bitboard::create_const(0x0508000805000000), Bitboard::create_const(0x0a1100110a000000), Bitboard::create_const(0x1422002214000000), 
    Bitboard::create_const(0x2844004428000000), Bitboard::create_const(0x5088008850000000), Bitboard::create_const(0xa0100010a0000000), Bitboard::create_const(0x4020002040000000), 
    Bitboard::create_const(0x0400040200000000), Bitboard::create_const(0x0800080500000000), Bitboard::create_const(0x1100110a00000000), Bitboard::create_const(0x2200221400000000), 
    Bitboard::create_const(0x4400442800000000), Bitboard::create_const(0x8800885000000000), Bitboard::create_const(0x100010a000000000), Bitboard::create_const(0x2000204000000000), 
    Bitboard::create_const(0x0004020000000000), Bitboard::create_const(0x0008050000000000), Bitboard::create_const(0x00110a0000000000), Bitboard::create_const(0x0022140000000000), 
    Bitboard::create_const(0x0044280000000000), Bitboard::create_const(0x0088500000000000), Bitboard::create_const(0x0010a00000000000), Bitboard::create_const(0x0020400000000000),
];

const KING: [Bitboard; 64] = [
    Bitboard::create_const(0x0000000000000302), Bitboard::create_const(0x0000000000000705), Bitboard::create_const(0x0000000000000e0a), Bitboard::create_const(0x0000000000001c14), 
    Bitboard::create_const(0x0000000000003828), Bitboard::create_const(0x0000000000007050), Bitboard::create_const(0x000000000000e0a0), Bitboard::create_const(0x000000000000c040), 
    Bitboard::create_const(0x0000000000030203), Bitboard::create_const(0x0000000000070507), Bitboard::create_const(0x00000000000e0a0e), Bitboard::create_const(0x00000000001c141c), 
    Bitboard::create_const(0x0000000000382838), Bitboard::create_const(0x0000000000705070), Bitboard::create_const(0x0000000000e0a0e0), Bitboard::create_const(0x0000000000c040c0), 
    Bitboard::create_const(0x0000000003020300), Bitboard::create_const(0x0000000007050700), Bitboard::create_const(0x000000000e0a0e00), Bitboard::create_const(0x000000001c141c00), 
    Bitboard::create_const(0x0000000038283800), Bitboard::create_const(0x0000000070507000), Bitboard::create_const(0x00000000e0a0e000), Bitboard::create_const(0x00000000c040c000), 
    Bitboard::create_const(0x0000000302030000), Bitboard::create_const(0x0000000705070000), Bitboard::create_const(0x0000000e0a0e0000), Bitboard::create_const(0x0000001c141c0000), 
    Bitboard::create_const(0x0000003828380000), Bitboard::create_const(0x0000007050700000), Bitboard::create_const(0x000000e0a0e00000), Bitboard::create_const(0x000000c040c00000), 
    Bitboard::create_const(0x0000030203000000), Bitboard::create_const(0x0000070507000000), Bitboard::create_const(0x00000e0a0e000000), Bitboard::create_const(0x00001c141c000000), 
    Bitboard::create_const(0x0000382838000000), Bitboard::create_const(0x0000705070000000), Bitboard::create_const(0x0000e0a0e0000000), Bitboard::create_const(0x0000c040c0000000), 
    Bitboard::create_const(0x0003020300000000), Bitboard::create_const(0x0007050700000000), Bitboard::create_const(0x000e0a0e00000000), Bitboard::create_const(0x001c141c00000000), 
    Bitboard::create_const(0x0038283800000000), Bitboard::create_const(0x0070507000000000), Bitboard::create_const(0x00e0a0e000000000), Bitboard::create_const(0x00c040c000000000), 
    Bitboard::create_const(0x0302030000000000), Bitboard::create_const(0x0705070000000000), Bitboard::create_const(0x0e0a0e0000000000), Bitboard::create_const(0x1c141c0000000000), 
    Bitboard::create_const(0x3828380000000000), Bitboard::create_const(0x7050700000000000), Bitboard::create_const(0xe0a0e00000000000), Bitboard::create_const(0xc040c00000000000), 
    Bitboard::create_const(0x0203000000000000), Bitboard::create_const(0x0507000000000000), Bitboard::create_const(0x0a0e000000000000), Bitboard::create_const(0x141c000000000000), 
    Bitboard::create_const(0x2838000000000000), Bitboard::create_const(0x5070000000000000), Bitboard::create_const(0xa0e0000000000000), Bitboard::create_const(0x40c0000000000000),
];


pub const ROOK_MASK: [Bitboard; 64] = [
    Bitboard::create_const(0x000101010101017e), Bitboard::create_const(0x000202020202027c), Bitboard::create_const(0x000404040404047a), Bitboard::create_const(0x0008080808080876), 
    Bitboard::create_const(0x001010101010106e), Bitboard::create_const(0x002020202020205e), Bitboard::create_const(0x004040404040403e), Bitboard::create_const(0x008080808080807e), 
    Bitboard::create_const(0x0001010101017e00), Bitboard::create_const(0x0002020202027c00), Bitboard::create_const(0x0004040404047a00), Bitboard::create_const(0x0008080808087600), 
    Bitboard::create_const(0x0010101010106e00), Bitboard::create_const(0x0020202020205e00), Bitboard::create_const(0x0040404040403e00), Bitboard::create_const(0x0080808080807e00), 
    Bitboard::create_const(0x00010101017e0100), Bitboard::create_const(0x00020202027c0200), Bitboard::create_const(0x00040404047a0400), Bitboard::create_const(0x0008080808760800), 
    Bitboard::create_const(0x00101010106e1000), Bitboard::create_const(0x00202020205e2000), Bitboard::create_const(0x00404040403e4000), Bitboard::create_const(0x00808080807e8000), 
    Bitboard::create_const(0x000101017e010100), Bitboard::create_const(0x000202027c020200), Bitboard::create_const(0x000404047a040400), Bitboard::create_const(0x0008080876080800), 
    Bitboard::create_const(0x001010106e101000), Bitboard::create_const(0x002020205e202000), Bitboard::create_const(0x004040403e404000), Bitboard::create_const(0x008080807e808000), 
    Bitboard::create_const(0x0001017e01010100), Bitboard::create_const(0x0002027c02020200), Bitboard::create_const(0x0004047a04040400), Bitboard::create_const(0x0008087608080800), 
    Bitboard::create_const(0x0010106e10101000), Bitboard::create_const(0x0020205e20202000), Bitboard::create_const(0x0040403e40404000), Bitboard::create_const(0x0080807e80808000), 
    Bitboard::create_const(0x00017e0101010100), Bitboard::create_const(0x00027c0202020200), Bitboard::create_const(0x00047a0404040400), Bitboard::create_const(0x0008760808080800), 
    Bitboard::create_const(0x00106e1010101000), Bitboard::create_const(0x00205e2020202000), Bitboard::create_const(0x00403e4040404000), Bitboard::create_const(0x00807e8080808000), 
    Bitboard::create_const(0x007e010101010100), Bitboard::create_const(0x007c020202020200), Bitboard::create_const(0x007a040404040400), Bitboard::create_const(0x0076080808080800), 
    Bitboard::create_const(0x006e101010101000), Bitboard::create_const(0x005e202020202000), Bitboard::create_const(0x003e404040404000), Bitboard::create_const(0x007e808080808000), 
    Bitboard::create_const(0x7e01010101010100), Bitboard::create_const(0x7c02020202020200), Bitboard::create_const(0x7a04040404040400), Bitboard::create_const(0x7608080808080800), 
    Bitboard::create_const(0x6e10101010101000), Bitboard::create_const(0x5e20202020202000), Bitboard::create_const(0x3e40404040404000), Bitboard::create_const(0x7e80808080808000), 
];


pub const BISHOP_MASK: [Bitboard; 64] = [
    Bitboard::create_const(0x0040201008040200), Bitboard::create_const(0x0000402010080400), Bitboard::create_const(0x0000004020100a00), Bitboard::create_const(0x0000000040221400), 
    Bitboard::create_const(0x0000000002442800), Bitboard::create_const(0x0000000204085000), Bitboard::create_const(0x0000020408102000), Bitboard::create_const(0x0002040810204000), 
    Bitboard::create_const(0x0020100804020000), Bitboard::create_const(0x0040201008040000), Bitboard::create_const(0x00004020100a0000), Bitboard::create_const(0x0000004022140000), 
    Bitboard::create_const(0x0000000244280000), Bitboard::create_const(0x0000020408500000), Bitboard::create_const(0x0002040810200000), Bitboard::create_const(0x0004081020400000), 
    Bitboard::create_const(0x0010080402000200), Bitboard::create_const(0x0020100804000400), Bitboard::create_const(0x004020100a000a00), Bitboard::create_const(0x0000402214001400), 
    Bitboard::create_const(0x0000024428002800), Bitboard::create_const(0x0002040850005000), Bitboard::create_const(0x0004081020002000), Bitboard::create_const(0x0008102040004000), 
    Bitboard::create_const(0x0008040200020400), Bitboard::create_const(0x0010080400040800), Bitboard::create_const(0x0020100a000a1000), Bitboard::create_const(0x0040221400142200), 
    Bitboard::create_const(0x0002442800284400), Bitboard::create_const(0x0004085000500800), Bitboard::create_const(0x0008102000201000), Bitboard::create_const(0x0010204000402000), 
    Bitboard::create_const(0x0004020002040800), Bitboard::create_const(0x0008040004081000), Bitboard::create_const(0x00100a000a102000), Bitboard::create_const(0x0022140014224000), 
    Bitboard::create_const(0x0044280028440200), Bitboard::create_const(0x0008500050080400), Bitboard::create_const(0x0010200020100800), Bitboard::create_const(0x0020400040201000), 
    Bitboard::create_const(0x0002000204081000), Bitboard::create_const(0x0004000408102000), Bitboard::create_const(0x000a000a10204000), Bitboard::create_const(0x0014001422400000), 
    Bitboard::create_const(0x0028002844020000), Bitboard::create_const(0x0050005008040200), Bitboard::create_const(0x0020002010080400), Bitboard::create_const(0x0040004020100800), 
    Bitboard::create_const(0x0000020408102000), Bitboard::create_const(0x0000040810204000), Bitboard::create_const(0x00000a1020400000), Bitboard::create_const(0x0000142240000000), 
    Bitboard::create_const(0x0000284402000000), Bitboard::create_const(0x0000500804020000), Bitboard::create_const(0x0000201008040200), Bitboard::create_const(0x0000402010080400), 
    Bitboard::create_const(0x0002040810204000), Bitboard::create_const(0x0004081020400000), Bitboard::create_const(0x000a102040000000), Bitboard::create_const(0x0014224000000000), 
    Bitboard::create_const(0x0028440200000000), Bitboard::create_const(0x0050080402000000), Bitboard::create_const(0x0020100804020000), Bitboard::create_const(0x0040201008040200), 
];



static mut ROOK: [[Bitboard; 4096]; 64] = [[Bitboard::create_const(0); 4096]; 64];

static mut BISHOP: [[Bitboard; 512]; 64] = [[Bitboard::create_const(0); 512]; 64];



pub fn get_pawn_attack(sq: Square, side: Colour) -> Bitboard {
    PAWN[side.as_index()][sq.as_index()]
}

pub fn get_knight_attack(sq: Square) -> Bitboard {
    KNIGHT[sq.as_index()]
}

pub fn get_bishop_attack(sq: Square, occ: Bitboard) -> Bitboard {
    let bishop_occ = occ & BISHOP_MASK[sq.as_index()];
    let magic_index = bishop_occ.get_raw().wrapping_mul(BISHOP_MAGICS[sq.as_index()]) >> (64 - BISHOP_REL[sq.as_index()]);
    unsafe {BISHOP[sq.as_index()][magic_index as usize]}
}

pub fn get_rook_attack(sq: Square, occ: Bitboard) -> Bitboard {
    let rook_occ = occ & ROOK_MASK[sq.as_index()];
    let magic_index = rook_occ.get_raw().wrapping_mul(ROOK_MAGICS[sq.as_index()]) >> (64 - ROOK_REL[sq.as_index()]);
    unsafe {ROOK[sq.as_index()][magic_index as usize]}
}

pub fn get_queen_attack(sq: Square, occ: Bitboard) -> Bitboard {
    get_rook_attack(sq, occ) | get_bishop_attack(sq, occ)
}

pub fn get_king_attack(sq: Square) -> Bitboard {
    KING[sq.as_index()]
}



pub fn init_slider() {
    for sq in 0..64 {
        for occ_i in 0..4096 {
            let occ = gen_occ(occ_i, ROOK_MASK[sq]);
            let attack = gen_rook_attacks(Square::from_u8(sq as u8), occ);
            let magic_index = occ.get_raw().wrapping_mul(ROOK_MAGICS[sq]) >> (64 - ROOK_REL[sq]);
            unsafe {ROOK[sq][magic_index as usize] = attack;}
        }

        for occ_i in 0..512 {
            let occ = gen_occ(occ_i, BISHOP_MASK[sq]);
            let attack = gen_bishop_attacks(Square::from_u8(sq as u8), occ);
            let magic_index = occ.get_raw().wrapping_mul(BISHOP_MAGICS[sq]) >> (64 - BISHOP_REL[sq]);
            unsafe {BISHOP[sq][magic_index as usize] = attack;}
        }
    }
}



pub fn gen_rook_attacks(source_sq: Square, occ: Bitboard) -> Bitboard {
    let dirs = [Direction::North, Direction::South, Direction::East, Direction::West];

    let mut rook_attack = Bitboard::new(); 

    for dir in dirs {

        if  (source_sq.on_north_edge() && dir == Direction::North) 
            ||  (source_sq.on_south_edge() && dir == Direction::South) 
            ||  (source_sq.on_east_edge() && dir == Direction::East) 
            ||  (source_sq.on_west_edge() && dir == Direction::West) {
                continue;
            }

        let mut target_sq = source_sq;
        for _ in 0..8 {

            target_sq.shift(dir);
            rook_attack.set_bit(target_sq);
            if (occ.get_bit(target_sq)) || 
            (dir == Direction::North && target_sq.on_north_edge()) ||
            (dir == Direction::East && target_sq.on_east_edge()) ||
            (dir == Direction::South && target_sq.on_south_edge()) ||
            (dir == Direction::West && target_sq.on_west_edge()) {
                break;
            }

        }
    }

    rook_attack
}

pub fn gen_bishop_attacks(source_sq: Square, occ: Bitboard) -> Bitboard {
    let dirs = [Direction::NE, Direction::NW, Direction::SE, Direction::SW];

    let mut bishop_attack = Bitboard::new(); 

    for dir in dirs {

        if  (source_sq.on_north_edge() && (dir == Direction::NE || dir == Direction::NW)) 
            ||  (source_sq.on_south_edge() && (dir == Direction::SE || dir == Direction::SW)) 
            ||  (source_sq.on_east_edge() && (dir == Direction::NE || dir == Direction::SE)) 
            ||  (source_sq.on_west_edge() && (dir == Direction::NW || dir == Direction::SW)) {
                continue;
            }

        let mut target_sq = source_sq;
        for _ in 0..8 {
            target_sq.shift(dir);
            bishop_attack.set_bit(target_sq);
            if target_sq.is_edge() || occ.get_bit(target_sq) {
                break;
            }

        }
    }

    bishop_attack
}