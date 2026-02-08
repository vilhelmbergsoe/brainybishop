// Build script for generating magic bitboard tables
// Generates src/magic_tables.rs with precomputed attack tables

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("magic_tables.rs");
    let mut f = File::create(&dest_path).unwrap();

    generate_magic_tables(&mut f);
}

fn generate_magic_tables(f: &mut File) {
    writeln!(f, "// Auto-generated magic bitboard tables").unwrap();
    writeln!(f, "// Do not edit manually").unwrap();
    writeln!(f).unwrap();

    let rook_masks = generate_rook_masks();
    write_array_u64(f, "ROOK_MASKS", &rook_masks);

    let bishop_masks = generate_bishop_masks();
    write_array_u64(f, "BISHOP_MASKS", &bishop_masks);

    // Magic numbers from CPW
    let rook_magics = get_rook_magics();
    write_array_u64(f, "ROOK_MAGICS", &rook_magics);

    let bishop_magics = get_bishop_magics();
    write_array_u64(f, "BISHOP_MAGICS", &bishop_magics);

    // 64 shifts (popcount of mask)
    let rook_shifts: Vec<u8> = rook_masks.iter().map(|m| 64 - m.count_ones() as u8).collect();
    write_array_u8(f, "ROOK_SHIFTS", &rook_shifts);

    let bishop_shifts: Vec<u8> = bishop_masks.iter().map(|m| 64 - m.count_ones() as u8).collect();
    write_array_u8(f, "BISHOP_SHIFTS", &bishop_shifts);

    let (rook_attacks, rook_offsets) = generate_rook_attack_table(&rook_masks, &rook_magics);
    write_array_usize(f, "ROOK_OFFSETS", &rook_offsets);
    write_array_u64(f, "ROOK_ATTACKS", &rook_attacks);

    let (bishop_attacks, bishop_offsets) =
        generate_bishop_attack_table(&bishop_masks, &bishop_magics);
    write_array_usize(f, "BISHOP_OFFSETS", &bishop_offsets);
    write_array_u64(f, "BISHOP_ATTACKS", &bishop_attacks);
}

fn write_array_u64(f: &mut File, name: &str, arr: &[u64]) {
    writeln!(f, "pub static {}: [u64; {}] = [", name, arr.len()).unwrap();
    for (i, val) in arr.iter().enumerate() {
        if i % 4 == 0 {
            write!(f, "    ").unwrap();
        }
        write!(f, "0x{:016X},", val).unwrap();
        if i % 4 == 3 || i == arr.len() - 1 {
            writeln!(f).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
    writeln!(f).unwrap();
}

fn write_array_u8(f: &mut File, name: &str, arr: &[u8]) {
    writeln!(f, "pub static {}: [u8; {}] = [", name, arr.len()).unwrap();
    for (i, val) in arr.iter().enumerate() {
        if i % 16 == 0 {
            write!(f, "    ").unwrap();
        }
        write!(f, "{:2},", val).unwrap();
        if i % 16 == 15 || i == arr.len() - 1 {
            writeln!(f).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
    writeln!(f).unwrap();
}

fn write_array_usize(f: &mut File, name: &str, arr: &[usize]) {
    writeln!(f, "pub static {}: [usize; {}] = [", name, arr.len()).unwrap();
    for (i, val) in arr.iter().enumerate() {
        if i % 8 == 0 {
            write!(f, "    ").unwrap();
        }
        write!(f, "{},", val).unwrap();
        if i % 8 == 7 || i == arr.len() - 1 {
            writeln!(f).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
    writeln!(f).unwrap();
}

fn generate_rook_masks() -> Vec<u64> {
    (0..64).map(|sq| rook_mask(sq)).collect()
}

fn generate_bishop_masks() -> Vec<u64> {
    (0..64).map(|sq| bishop_mask(sq)).collect()
}

fn rook_mask(sq: usize) -> u64 {
    let file = sq % 8;
    let rank = sq / 8;
    let mut mask = 0u64;

    // North (exclude last rank)
    for r in (rank + 1)..7 {
        mask |= 1 << (file + r * 8);
    }
    // South (exclude first rank)
    for r in 1..rank {
        mask |= 1 << (file + r * 8);
    }
    // East (exclude last file)
    for f in (file + 1)..7 {
        mask |= 1 << (f + rank * 8);
    }
    // West (exclude first file)
    for f in 1..file {
        mask |= 1 << (f + rank * 8);
    }

    mask
}

fn bishop_mask(sq: usize) -> u64 {
    let file = sq % 8;
    let rank = sq / 8;
    let mut mask = 0u64;

    // NE
    let mut f = file + 1;
    let mut r = rank + 1;
    while f < 7 && r < 7 {
        mask |= 1 << (f + r * 8);
        f += 1;
        r += 1;
    }

    // NW
    if file > 0 {
        f = file - 1;
        r = rank + 1;
        while f > 0 && r < 7 {
            mask |= 1 << (f + r * 8);
            f -= 1;
            r += 1;
        }
    }

    // SE
    f = file + 1;
    if rank > 0 {
        r = rank - 1;
        while f < 7 && r > 0 {
            mask |= 1 << (f + r * 8);
            f += 1;
            r -= 1;
        }
    }

    // SW
    if file > 0 && rank > 0 {
        f = file - 1;
        r = rank - 1;
        while f > 0 && r > 0 {
            mask |= 1 << (f + r * 8);
            f -= 1;
            r -= 1;
        }
    }

    mask
}

fn rook_attacks_slow(sq: usize, occ: u64) -> u64 {
    let file = sq % 8;
    let rank = sq / 8;
    let mut attacks = 0u64;

    // North
    for r in (rank + 1)..8 {
        let bit = 1 << (file + r * 8);
        attacks |= bit;
        if occ & bit != 0 {
            break;
        }
    }
    // South
    if rank > 0 {
        for r in (0..rank).rev() {
            let bit = 1 << (file + r * 8);
            attacks |= bit;
            if occ & bit != 0 {
                break;
            }
        }
    }
    // East
    for f in (file + 1)..8 {
        let bit = 1 << (f + rank * 8);
        attacks |= bit;
        if occ & bit != 0 {
            break;
        }
    }
    // West
    if file > 0 {
        for f in (0..file).rev() {
            let bit = 1 << (f + rank * 8);
            attacks |= bit;
            if occ & bit != 0 {
                break;
            }
        }
    }

    attacks
}

fn bishop_attacks_slow(sq: usize, occ: u64) -> u64 {
    let file = sq as i32 % 8;
    let rank = sq as i32 / 8;
    let mut attacks = 0u64;

    // NE
    let mut f = file + 1;
    let mut r = rank + 1;
    while f < 8 && r < 8 {
        let bit = 1 << (f + r * 8);
        attacks |= bit;
        if occ & bit != 0 {
            break;
        }
        f += 1;
        r += 1;
    }

    // NW
    f = file - 1;
    r = rank + 1;
    while f >= 0 && r < 8 {
        let bit = 1 << (f + r * 8);
        attacks |= bit;
        if occ & bit != 0 {
            break;
        }
        f -= 1;
        r += 1;
    }

    // SE
    f = file + 1;
    r = rank - 1;
    while f < 8 && r >= 0 {
        let bit = 1 << (f + r * 8);
        attacks |= bit;
        if occ & bit != 0 {
            break;
        }
        f += 1;
        r -= 1;
    }

    // SW
    f = file - 1;
    r = rank - 1;
    while f >= 0 && r >= 0 {
        let bit = 1 << (f + r * 8);
        attacks |= bit;
        if occ & bit != 0 {
            break;
        }
        f -= 1;
        r -= 1;
    }

    attacks
}

fn index_to_occupancy(index: usize, mask: u64) -> u64 {
    let mut occ = 0u64;
    let mut m = mask;
    let mut i = index;

    while m != 0 {
        let lsb = m.trailing_zeros() as usize;
        if i & 1 != 0 {
            occ |= 1 << lsb;
        }
        m &= m - 1;
        i >>= 1;
    }

    occ
}

fn generate_rook_attack_table(masks: &[u64], magics: &[u64]) -> (Vec<u64>, Vec<usize>) {
    let mut attacks = Vec::new();
    let mut offsets = Vec::with_capacity(64);

    for sq in 0..64 {
        let mask = masks[sq];
        let magic = magics[sq];
        let bits = mask.count_ones();
        let table_size = 1 << bits;
        let offset = attacks.len();
        offsets.push(offset);

        attacks.resize(offset + table_size, 0);

        for idx in 0..table_size {
            let occ = index_to_occupancy(idx, mask);
            let hash = (occ.wrapping_mul(magic)) >> (64 - bits);
            attacks[offset + hash as usize] = rook_attacks_slow(sq, occ);
        }
    }

    (attacks, offsets)
}

fn generate_bishop_attack_table(masks: &[u64], magics: &[u64]) -> (Vec<u64>, Vec<usize>) {
    let mut attacks = Vec::new();
    let mut offsets = Vec::with_capacity(64);

    for sq in 0..64 {
        let mask = masks[sq];
        let magic = magics[sq];
        let bits = mask.count_ones();
        let table_size = 1 << bits;
        let offset = attacks.len();
        offsets.push(offset);

        attacks.resize(offset + table_size, 0);

        for idx in 0..table_size {
            let occ = index_to_occupancy(idx, mask);
            let hash = (occ.wrapping_mul(magic)) >> (64 - bits);
            attacks[offset + hash as usize] = bishop_attacks_slow(sq, occ);
        }
    }

    (attacks, offsets)
}

// Magic numbers from CPW
fn get_rook_magics() -> Vec<u64> {
    vec![
        0x0080001020400080,
        0x0040001000200040,
        0x0080081000200080,
        0x0080040800100080,
        0x0080020400080080,
        0x0080010200040080,
        0x0080008001000200,
        0x0080002040800100,
        0x0000800020400080,
        0x0000400020005000,
        0x0000801000200080,
        0x0000800800100080,
        0x0000800400080080,
        0x0000800200040080,
        0x0000800100020080,
        0x0000800040800100,
        0x0000208000400080,
        0x0000404000201000,
        0x0000808010002000,
        0x0000808008001000,
        0x0000808004000800,
        0x0000808002000400,
        0x0000010100020004,
        0x0000020000408104,
        0x0000208080004000,
        0x0000200040005000,
        0x0000100080200080,
        0x0000080080100080,
        0x0000040080080080,
        0x0000020080040080,
        0x0000010080800200,
        0x0000800080004100,
        0x0000204000800080,
        0x0000200040401000,
        0x0000100080802000,
        0x0000080080801000,
        0x0000040080800800,
        0x0000020080800400,
        0x0000020001010004,
        0x0000800040800100,
        0x0000204000808000,
        0x0000200040008080,
        0x0000100020008080,
        0x0000080010008080,
        0x0000040008008080,
        0x0000020004008080,
        0x0000010002008080,
        0x0000004081020004,
        0x0000204000800080,
        0x0000200040008080,
        0x0000100020008080,
        0x0000080010008080,
        0x0000040008008080,
        0x0000020004008080,
        0x0000800100020080,
        0x0000800041000080,
        0x00FFFCDDFCED714A,
        0x007FFCDDFCED714A,
        0x003FFFCDFFD88096,
        0x0000040810002101,
        0x0001000204080011,
        0x0001000204000801,
        0x0001000082000401,
        0x0001FFFAABFAD1A2,
    ]
}

fn get_bishop_magics() -> Vec<u64> {
    vec![
        0x0002020202020200,
        0x0002020202020000,
        0x0004010202000000,
        0x0004040080000000,
        0x0001104000000000,
        0x0000821040000000,
        0x0000410410400000,
        0x0000104104104000,
        0x0000040404040400,
        0x0000020202020200,
        0x0000040102020000,
        0x0000040400800000,
        0x0000011040000000,
        0x0000008210400000,
        0x0000004104104000,
        0x0000002082082000,
        0x0004000808080800,
        0x0002000404040400,
        0x0001000202020200,
        0x0000800802004000,
        0x0000800400A00000,
        0x0000200100884000,
        0x0000400082082000,
        0x0000200041041000,
        0x0002080010101000,
        0x0001040008080800,
        0x0000208004010400,
        0x0000404004010200,
        0x0000840000802000,
        0x0000404002011000,
        0x0000808001041000,
        0x0000404000820800,
        0x0001041000202000,
        0x0000820800101000,
        0x0000104400080800,
        0x0000020080080080,
        0x0000404040040100,
        0x0000808100020100,
        0x0001010100020800,
        0x0000808080010400,
        0x0000820820004000,
        0x0000410410002000,
        0x0000082088001000,
        0x0000002011000800,
        0x0000080100400400,
        0x0001010101000200,
        0x0002020202000400,
        0x0001010101000200,
        0x0000410410400000,
        0x0000208208200000,
        0x0000002084100000,
        0x0000000020880000,
        0x0000001002020000,
        0x0000040408020000,
        0x0004040404040000,
        0x0002020202020000,
        0x0000104104104000,
        0x0000002082082000,
        0x0000000020841000,
        0x0000000000208800,
        0x0000000010020200,
        0x0000000404080200,
        0x0000040404040400,
        0x0002020202020200,
    ]
}
