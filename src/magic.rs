// Magic bitboard lookup functions for sliding piece attacks

include!(concat!(env!("OUT_DIR"), "/magic_tables.rs"));

#[inline(always)]
pub fn bishop_attacks(sq: usize, occ: u64) -> u64 {
    debug_assert!(sq < 64);

    let mask = BISHOP_MASKS[sq];
    let magic = BISHOP_MAGICS[sq];
    let shift = BISHOP_SHIFTS[sq];
    let offset = BISHOP_OFFSETS[sq];

    let idx = ((occ & mask).wrapping_mul(magic) >> shift) as usize;
    BISHOP_ATTACKS[offset + idx]
}

#[inline(always)]
pub fn rook_attacks(sq: usize, occ: u64) -> u64 {
    debug_assert!(sq < 64);

    let mask = ROOK_MASKS[sq];
    let magic = ROOK_MAGICS[sq];
    let shift = ROOK_SHIFTS[sq];
    let offset = ROOK_OFFSETS[sq];

    let idx = ((occ & mask).wrapping_mul(magic) >> shift) as usize;
    ROOK_ATTACKS[offset + idx]
}

#[inline(always)]
pub fn queen_attacks(sq: usize, occ: u64) -> u64 {
    bishop_attacks(sq, occ) | rook_attacks(sq, occ)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_attacks_empty_board() {
        // Rook on e4 (square 28) on empty board
        let attacks = rook_attacks(28, 0);
        // Should attack entire e-file and 4th rank (minus e4 itself)
        let expected_count = 14; // 7 vertical + 7 horizontal
        assert_eq!(attacks.count_ones(), expected_count);
    }

    #[test]
    fn test_rook_attacks_blocked() {
        // Rook on a1, blocker on a4
        let occ = 1u64 << 24; // a4
        let attacks = rook_attacks(0, occ);
        // Should attack a2, a3, a4 (blocked), b1-h1
        assert!(attacks & (1 << 8) != 0); // a2
        assert!(attacks & (1 << 16) != 0); // a3
        assert!(attacks & (1 << 24) != 0); // a4 (capture)
        assert!(attacks & (1 << 32) == 0); // a5 (blocked)
    }

    #[test]
    fn test_bishop_attacks_empty_board() {
        // Bishop on e4 (square 28) on empty board
        let attacks = bishop_attacks(28, 0);
        // Should attack diagonals
        assert!(attacks.count_ones() == 13);
    }

    #[test]
    fn test_bishop_attacks_blocked() {
        // Bishop on a1, blocker on c3
        let occ = 1u64 << 18; // c3
        let attacks = bishop_attacks(0, occ);
        // Should attack b2, c3 (blocked)
        assert!(attacks & (1 << 9) != 0); // b2
        assert!(attacks & (1 << 18) != 0); // c3 (capture)
        assert!(attacks & (1 << 27) == 0); // d4 (blocked)
    }

    #[test]
    fn test_queen_attacks() {
        // Queen combines rook and bishop
        let occ = 0u64;
        let sq = 28; // e4
        let queen = queen_attacks(sq, occ);
        let rook = rook_attacks(sq, occ);
        let bishop = bishop_attacks(sq, occ);
        assert_eq!(queen, rook | bishop);
    }
}
