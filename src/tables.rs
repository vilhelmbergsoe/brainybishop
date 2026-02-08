// File masks
pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = 0x0202020202020202;
pub const FILE_C: u64 = 0x0404040404040404;
pub const FILE_D: u64 = 0x0808080808080808;
pub const FILE_E: u64 = 0x1010101010101010;
pub const FILE_F: u64 = 0x2020202020202020;
pub const FILE_G: u64 = 0x4040404040404040;
pub const FILE_H: u64 = 0x8080808080808080;

pub const NOT_FILE_A: u64 = !FILE_A;
pub const NOT_FILE_H: u64 = !FILE_H;
pub const NOT_FILE_AB: u64 = !(FILE_A | FILE_B);
pub const NOT_FILE_GH: u64 = !(FILE_G | FILE_H);

// Rank masks
pub const RANK_1: u64 = 0x00000000000000FF;
pub const RANK_2: u64 = 0x000000000000FF00;
pub const RANK_3: u64 = 0x0000000000FF0000;
pub const RANK_4: u64 = 0x00000000FF000000;
pub const RANK_5: u64 = 0x000000FF00000000;
pub const RANK_6: u64 = 0x0000FF0000000000;
pub const RANK_7: u64 = 0x00FF000000000000;
pub const RANK_8: u64 = 0xFF00000000000000;

// Direction indices for RAYS array
pub const DIR_N: usize = 0;
pub const DIR_NE: usize = 1;
pub const DIR_E: usize = 2;
pub const DIR_SE: usize = 3;
pub const DIR_S: usize = 4;
pub const DIR_SW: usize = 5;
pub const DIR_W: usize = 6;
pub const DIR_NW: usize = 7;

pub const KNIGHT_ATTACKS: [u64; 64] = generate_knight_attacks();

const fn generate_knight_attacks() -> [u64; 64] {
    let mut attacks = [0u64; 64];
    let mut sq = 0usize;

    while sq < 64 {
        let bb = 1u64 << sq;
        let mut atk = 0u64;

        // NNE: +17 (up 2, right 1)
        if bb & NOT_FILE_H != 0 {
            atk |= bb << 17;
        }
        // NEE: +10 (up 1, right 2)
        if bb & NOT_FILE_GH != 0 {
            atk |= bb << 10;
        }
        // SEE: -6 (down 1, right 2)
        if bb & NOT_FILE_GH != 0 {
            atk |= bb >> 6;
        }
        // SSE: -15 (down 2, right 1)
        if bb & NOT_FILE_H != 0 {
            atk |= bb >> 15;
        }
        // SSW: -17 (down 2, left 1)
        if bb & NOT_FILE_A != 0 {
            atk |= bb >> 17;
        }
        // SWW: -10 (down 1, left 2)
        if bb & NOT_FILE_AB != 0 {
            atk |= bb >> 10;
        }
        // NWW: +6 (up 1, left 2)
        if bb & NOT_FILE_AB != 0 {
            atk |= bb << 6;
        }
        // NNW: +15 (up 2, left 1)
        if bb & NOT_FILE_A != 0 {
            atk |= bb << 15;
        }

        attacks[sq] = atk;
        sq += 1;
    }

    attacks
}

pub const KING_ATTACKS: [u64; 64] = generate_king_attacks();

const fn generate_king_attacks() -> [u64; 64] {
    let mut attacks = [0u64; 64];
    let mut sq = 0usize;

    while sq < 64 {
        let bb = 1u64 << sq;
        let mut atk = 0u64;

        // North
        atk |= bb << 8;
        // South
        atk |= bb >> 8;
        // East (not on H file)
        if bb & NOT_FILE_H != 0 {
            atk |= bb << 1;
            atk |= bb << 9; // NE
            atk |= bb >> 7; // SE
        }
        // West (not on A file)
        if bb & NOT_FILE_A != 0 {
            atk |= bb >> 1;
            atk |= bb << 7; // NW
            atk |= bb >> 9; // SW
        }

        attacks[sq] = atk;
        sq += 1;
    }

    attacks
}

// Pawn attack table - [color][square]
// color: 0 = white, 1 = black
pub const PAWN_ATTACKS: [[u64; 64]; 2] = generate_pawn_attacks();

const fn generate_pawn_attacks() -> [[u64; 64]; 2] {
    let mut attacks = [[0u64; 64]; 2];
    let mut sq = 0usize;

    while sq < 64 {
        let bb = 1u64 << sq;

        // White pawn attacks (moving up the board)
        let mut white_atk = 0u64;
        if bb & NOT_FILE_A != 0 {
            white_atk |= bb << 7; // NW capture
        }
        if bb & NOT_FILE_H != 0 {
            white_atk |= bb << 9; // NE capture
        }
        attacks[0][sq] = white_atk;

        // Black pawn attacks (moving down the board)
        let mut black_atk = 0u64;
        if bb & NOT_FILE_A != 0 {
            black_atk |= bb >> 9; // SW capture
        }
        if bb & NOT_FILE_H != 0 {
            black_atk |= bb >> 7; // SE capture
        }
        attacks[1][sq] = black_atk;

        sq += 1;
    }

    attacks
}

// Ray tables - [direction][square]
// Each ray extends from the square in one direction until it hits the edge
pub const RAYS: [[u64; 64]; 8] = generate_rays();

const fn generate_rays() -> [[u64; 64]; 8] {
    let mut rays = [[0u64; 64]; 8];
    let mut sq = 0usize;

    while sq < 64 {
        let file = sq % 8;
        let rank = sq / 8;

        // North ray
        let mut ray = 0u64;
        let mut r = rank + 1;
        while r < 8 {
            ray |= 1u64 << (file + r * 8);
            r += 1;
        }
        rays[DIR_N][sq] = ray;

        // South ray
        ray = 0;
        if rank > 0 {
            let mut r = rank - 1;
            loop {
                ray |= 1u64 << (file + r * 8);
                if r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        rays[DIR_S][sq] = ray;

        // East ray
        ray = 0;
        let mut f = file + 1;
        while f < 8 {
            ray |= 1u64 << (f + rank * 8);
            f += 1;
        }
        rays[DIR_E][sq] = ray;

        // West ray
        ray = 0;
        if file > 0 {
            let mut f = file - 1;
            loop {
                ray |= 1u64 << (f + rank * 8);
                if f == 0 {
                    break;
                }
                f -= 1;
            }
        }
        rays[DIR_W][sq] = ray;

        // NE ray
        ray = 0;
        let mut f = file + 1;
        let mut r = rank + 1;
        while f < 8 && r < 8 {
            ray |= 1u64 << (f + r * 8);
            f += 1;
            r += 1;
        }
        rays[DIR_NE][sq] = ray;

        // NW ray
        ray = 0;
        r = rank + 1;
        if file > 0 {
            let mut f = file - 1;
            while r < 8 {
                ray |= 1u64 << (f + r * 8);
                if f == 0 {
                    break;
                }
                f -= 1;
                r += 1;
            }
        }
        rays[DIR_NW][sq] = ray;

        // SE ray
        ray = 0;
        f = file + 1;
        if rank > 0 {
            let mut r = rank - 1;
            while f < 8 {
                ray |= 1u64 << (f + r * 8);
                if r == 0 {
                    break;
                }
                f += 1;
                r -= 1;
            }
        }
        rays[DIR_SE][sq] = ray;

        // SW ray
        ray = 0;
        if file > 0 && rank > 0 {
            let mut f = file - 1;
            let mut r = rank - 1;
            loop {
                ray |= 1u64 << (f + r * 8);
                if f == 0 || r == 0 {
                    break;
                }
                f -= 1;
                r -= 1;
            }
        }
        rays[DIR_SW][sq] = ray;

        sq += 1;
    }

    rays
}

// Between table - squares between sq1 and sq2 (exclusive)
// Only filled for squares on the same rank, file, or diagonal
pub const BETWEEN: [[u64; 64]; 64] = generate_between();

const fn generate_between() -> [[u64; 64]; 64] {
    let mut between = [[0u64; 64]; 64];
    let mut sq1 = 0usize;

    while sq1 < 64 {
        let mut sq2 = 0usize;
        while sq2 < 64 {
            if sq1 != sq2 {
                let f1 = sq1 % 8;
                let r1 = sq1 / 8;
                let f2 = sq2 % 8;
                let r2 = sq2 / 8;

                let df = (f2 as i32) - (f1 as i32);
                let dr = (r2 as i32) - (r1 as i32);

                // Check if squares are aligned
                let aligned = f1 == f2 || r1 == r2 || abs(df) == abs(dr);

                if aligned {
                    let step_f = sign(df);
                    let step_r = sign(dr);

                    let mut f = (f1 as i32) + step_f;
                    let mut r = (r1 as i32) + step_r;
                    let mut result = 0u64;

                    while f != (f2 as i32) || r != (r2 as i32) {
                        if f < 0 || f > 7 || r < 0 || r > 7 {
                            break;
                        }
                        result |= 1u64 << ((f as usize) + (r as usize) * 8);
                        f += step_f;
                        r += step_r;
                    }

                    between[sq1][sq2] = result;
                }
            }
            sq2 += 1;
        }
        sq1 += 1;
    }

    between
}

// Line table - full line through sq1 and sq2
// Only filled for squares on the same rank, file, or diagonal
pub const LINE: [[u64; 64]; 64] = generate_line();

const fn generate_line() -> [[u64; 64]; 64] {
    let mut line = [[0u64; 64]; 64];
    let mut sq1 = 0usize;

    while sq1 < 64 {
        let mut sq2 = 0usize;
        while sq2 < 64 {
            if sq1 != sq2 {
                let f1 = sq1 % 8;
                let r1 = sq1 / 8;
                let f2 = sq2 % 8;
                let r2 = sq2 / 8;

                let df = (f2 as i32) - (f1 as i32);
                let dr = (r2 as i32) - (r1 as i32);

                // Check if squares are on same file
                if f1 == f2 {
                    line[sq1][sq2] = RAYS[DIR_N][sq1] | RAYS[DIR_S][sq1] | (1u64 << sq1);
                }
                // Check if squares are on same rank
                else if r1 == r2 {
                    line[sq1][sq2] = RAYS[DIR_E][sq1] | RAYS[DIR_W][sq1] | (1u64 << sq1);
                }
                // Check if squares are on same diagonal
                else if abs(df) == abs(dr) {
                    if (df > 0) == (dr > 0) {
                        // NE-SW diagonal
                        line[sq1][sq2] = RAYS[DIR_NE][sq1] | RAYS[DIR_SW][sq1] | (1u64 << sq1);
                    } else {
                        // NW-SE diagonal
                        line[sq1][sq2] = RAYS[DIR_NW][sq1] | RAYS[DIR_SE][sq1] | (1u64 << sq1);
                    }
                }
            }
            sq2 += 1;
        }
        sq1 += 1;
    }

    line
}

const fn abs(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

const fn sign(x: i32) -> i32 {
    if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_attacks_corner() {
        // a1 knight can attack b3, c2
        assert_eq!(KNIGHT_ATTACKS[0], (1 << 17) | (1 << 10));
        // h8 knight can attack g6, f7
        assert_eq!(KNIGHT_ATTACKS[63], (1 << 46) | (1 << 53));
    }

    #[test]
    fn test_knight_attacks_center() {
        // e4 (square 28) should have 8 attacks
        let attacks = KNIGHT_ATTACKS[28];
        assert_eq!(attacks.count_ones(), 8);
    }

    #[test]
    fn test_king_attacks_corner() {
        // a1 king can attack a2, b1, b2
        let expected = (1 << 1) | (1 << 8) | (1 << 9);
        assert_eq!(KING_ATTACKS[0], expected);
    }

    #[test]
    fn test_king_attacks_center() {
        // e4 (square 28) should have 8 attacks
        let attacks = KING_ATTACKS[28];
        assert_eq!(attacks.count_ones(), 8);
    }

    #[test]
    fn test_pawn_attacks() {
        // White pawn on e4 attacks d5, f5
        assert_eq!(PAWN_ATTACKS[0][28], (1 << 35) | (1 << 37));
        // Black pawn on e5 attacks d4, f4
        assert_eq!(PAWN_ATTACKS[1][36], (1 << 27) | (1 << 29));
    }

    #[test]
    fn test_between_horizontal() {
        // Between a1 (0) and d1 (3) should be b1, c1
        assert_eq!(BETWEEN[0][3], (1 << 1) | (1 << 2));
    }

    #[test]
    fn test_between_vertical() {
        // Between a1 (0) and a4 (24) should be a2, a3
        assert_eq!(BETWEEN[0][24], (1 << 8) | (1 << 16));
    }

    #[test]
    fn test_between_diagonal() {
        // Between a1 (0) and d4 (27) should be b2, c3
        assert_eq!(BETWEEN[0][27], (1 << 9) | (1 << 18));
    }

    #[test]
    fn test_line_horizontal() {
        // Line through a1 and d1 should be the entire first rank
        let expected = RANK_1;
        assert_eq!(LINE[0][3], expected);
    }

    #[test]
    fn test_line_vertical() {
        // Line through a1 and a4 should be the entire A file
        assert_eq!(LINE[0][24], FILE_A);
    }
}
