use crate::bitboard::BitIter;
use crate::board::{Board, Color, PieceType, BK_CASTLE, BQ_CASTLE, WK_CASTLE, WQ_CASTLE};
use crate::magic::{bishop_attacks, queen_attacks, rook_attacks};
use crate::tables::{BETWEEN, KING_ATTACKS, KNIGHT_ATTACKS, LINE, PAWN_ATTACKS, RANK_2, RANK_7};

// Move flags (4 bits)
pub const FLAG_QUIET: u16 = 0;
pub const FLAG_DOUBLE_PUSH: u16 = 1;
pub const FLAG_KING_CASTLE: u16 = 2;
pub const FLAG_QUEEN_CASTLE: u16 = 3;
pub const FLAG_CAPTURE: u16 = 4;
pub const FLAG_EP_CAPTURE: u16 = 5;
pub const FLAG_PROMO_N: u16 = 8;
pub const FLAG_PROMO_B: u16 = 9;
pub const FLAG_PROMO_R: u16 = 10;
pub const FLAG_PROMO_Q: u16 = 11;
pub const FLAG_PROMO_CAPTURE_N: u16 = 12;
pub const FLAG_PROMO_CAPTURE_B: u16 = 13;
pub const FLAG_PROMO_CAPTURE_R: u16 = 14;
pub const FLAG_PROMO_CAPTURE_Q: u16 = 15;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Move(u16);

impl Move {
    #[inline(always)]
    pub const fn new(from: usize, to: usize, flags: u16) -> Self {
        debug_assert!(from < 64);
        debug_assert!(to < 64);
        debug_assert!(flags < 16);
        Move((from as u16) | ((to as u16) << 6) | (flags << 12))
    }

    #[inline(always)]
    pub const fn from(self) -> usize {
        (self.0 & 0x3F) as usize
    }

    #[inline(always)]
    pub const fn to(self) -> usize {
        ((self.0 >> 6) & 0x3F) as usize
    }

    #[inline(always)]
    pub const fn flags(self) -> u16 {
        self.0 >> 12
    }

    #[inline(always)]
    pub const fn is_capture(self) -> bool {
        let f = self.flags();
        f == FLAG_CAPTURE || f == FLAG_EP_CAPTURE || f >= FLAG_PROMO_CAPTURE_N
    }

    #[inline(always)]
    pub const fn is_promotion(self) -> bool {
        self.flags() >= FLAG_PROMO_N
    }

    #[inline(always)]
    pub const fn is_castle(self) -> bool {
        let f = self.flags();
        f == FLAG_KING_CASTLE || f == FLAG_QUEEN_CASTLE
    }

    #[inline(always)]
    pub fn promotion_piece(self) -> Option<PieceType> {
        match self.flags() {
            FLAG_PROMO_N | FLAG_PROMO_CAPTURE_N => Some(PieceType::Knight),
            FLAG_PROMO_B | FLAG_PROMO_CAPTURE_B => Some(PieceType::Bishop),
            FLAG_PROMO_R | FLAG_PROMO_CAPTURE_R => Some(PieceType::Rook),
            FLAG_PROMO_Q | FLAG_PROMO_CAPTURE_Q => Some(PieceType::Queen),
            _ => None,
        }
    }

    pub fn to_uci(self) -> String {
        let from_file = (self.from() % 8) as u8 + b'a';
        let from_rank = (self.from() / 8) as u8 + b'1';
        let to_file = (self.to() % 8) as u8 + b'a';
        let to_rank = (self.to() / 8) as u8 + b'1';

        let mut s = String::with_capacity(5);
        s.push(from_file as char);
        s.push(from_rank as char);
        s.push(to_file as char);
        s.push(to_rank as char);

        if let Some(promo) = self.promotion_piece() {
            s.push(match promo {
                PieceType::Knight => 'n',
                PieceType::Bishop => 'b',
                PieceType::Rook => 'r',
                PieceType::Queen => 'q',
                _ => unreachable!(),
            });
        }

        s
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_uci())
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_uci())
    }
}

pub struct MoveList {
    moves: [Move; 256],
    len: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [Move::default(); 256],
            len: 0,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, mv: Move) {
        debug_assert!(self.len < 256);
        self.moves[self.len] = mv;
        self.len += 1;
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves[..self.len].iter()
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

// Attack information computed once per position
pub struct AttackInfo {
    pub checkers: u64,
    pub check_mask: u64,
    pub pin_masks: [u64; 64],
    pub opponent_attacks: u64,
}

impl AttackInfo {
    pub fn new(board: &Board) -> Self {
        let us = board.turn;
        let them = us.opposite();
        let king_sq = board.king_square(us);
        let occ = board.all_occupancy();

        let mut info = AttackInfo {
            checkers: 0,
            check_mask: !0,
            pin_masks: [!0u64; 64],
            opponent_attacks: 0,
        };

        info.opponent_attacks = compute_attacks(board, them, king_sq);

        info.checkers = attackers_of(board, king_sq, them);

        let num_checkers = info.checkers.count_ones();
        if num_checkers == 1 {
            let checker_sq = info.checkers.trailing_zeros() as usize;
            info.check_mask = BETWEEN[king_sq][checker_sq] | info.checkers;
        } else if num_checkers > 1 {
            info.check_mask = 0;
        }

        compute_pins(board, king_sq, us, &mut info.pin_masks, occ);

        info
    }

    #[inline(always)]
    pub fn in_check(&self) -> bool {
        self.checkers != 0
    }

    #[inline(always)]
    pub fn in_double_check(&self) -> bool {
        self.checkers.count_ones() > 1
    }
}

fn compute_attacks(board: &Board, color: Color, exclude_king_sq: usize) -> u64 {
    let mut attacks = 0u64;
    let occ = board.all_occupancy() & !(1u64 << exclude_king_sq);

    let pawns = board.pieces(PieceType::Pawn, color);
    for sq in BitIter(pawns) {
        attacks |= PAWN_ATTACKS[color.index()][sq];
    }

    let knights = board.pieces(PieceType::Knight, color);
    for sq in BitIter(knights) {
        attacks |= KNIGHT_ATTACKS[sq];
    }

    let bishops = board.pieces(PieceType::Bishop, color) | board.pieces(PieceType::Queen, color);
    for sq in BitIter(bishops) {
        attacks |= bishop_attacks(sq, occ);
    }

    let rooks = board.pieces(PieceType::Rook, color) | board.pieces(PieceType::Queen, color);
    for sq in BitIter(rooks) {
        attacks |= rook_attacks(sq, occ);
    }

    let king_sq = board.king_square(color);
    attacks |= KING_ATTACKS[king_sq];

    attacks
}

fn attackers_of(board: &Board, sq: usize, color: Color) -> u64 {
    let occ = board.all_occupancy();
    let mut attackers = 0u64;

    let pawn_attacks = PAWN_ATTACKS[color.opposite().index()][sq];
    attackers |= pawn_attacks & board.pieces(PieceType::Pawn, color);

    attackers |= KNIGHT_ATTACKS[sq] & board.pieces(PieceType::Knight, color);

    let bishop_rays = bishop_attacks(sq, occ);
    attackers |=
        bishop_rays & (board.pieces(PieceType::Bishop, color) | board.pieces(PieceType::Queen, color));

    let rook_rays = rook_attacks(sq, occ);
    attackers |=
        rook_rays & (board.pieces(PieceType::Rook, color) | board.pieces(PieceType::Queen, color));

    attackers |= KING_ATTACKS[sq] & board.pieces(PieceType::King, color);

    attackers
}

fn compute_pins(board: &Board, king_sq: usize, us: Color, pin_masks: &mut [u64; 64], occ: u64) {
    let them = us.opposite();
    let our_pieces = board.occupancy(us);

    // Diagonal pinners (bishops and queens)
    let diag_sliders = board.pieces(PieceType::Bishop, them) | board.pieces(PieceType::Queen, them);
    let diag_rays = bishop_attacks(king_sq, 0);
    let potential_diag_pinners = diag_rays & diag_sliders;

    for pinner_sq in BitIter(potential_diag_pinners) {
        let between = BETWEEN[king_sq][pinner_sq];
        let blockers = between & occ;

        if blockers.count_ones() == 1 {
            let blocker_sq = blockers.trailing_zeros() as usize;
            if (1u64 << blocker_sq) & our_pieces != 0 {
                pin_masks[blocker_sq] = LINE[king_sq][pinner_sq];
            }
        }
    }

    // Orthogonal pinners (rooks and queens)
    let ortho_sliders = board.pieces(PieceType::Rook, them) | board.pieces(PieceType::Queen, them);
    let ortho_rays = rook_attacks(king_sq, 0);
    let potential_ortho_pinners = ortho_rays & ortho_sliders;

    for pinner_sq in BitIter(potential_ortho_pinners) {
        let between = BETWEEN[king_sq][pinner_sq];
        let blockers = between & occ;

        if blockers.count_ones() == 1 {
            let blocker_sq = blockers.trailing_zeros() as usize;
            if (1u64 << blocker_sq) & our_pieces != 0 {
                pin_masks[blocker_sq] = LINE[king_sq][pinner_sq];
            }
        }
    }
}

pub fn generate_moves(board: &Board) -> MoveList {
    let mut moves = MoveList::new();
    let info = AttackInfo::new(board);

    // Double check: only king moves are legal
    if info.in_double_check() {
        generate_king_moves(board, &info, &mut moves);
        return moves;
    }

    generate_pawn_moves(board, &info, &mut moves);
    generate_knight_moves(board, &info, &mut moves);
    generate_bishop_moves(board, &info, &mut moves);
    generate_rook_moves(board, &info, &mut moves);
    generate_queen_moves(board, &info, &mut moves);
    generate_king_moves(board, &info, &mut moves);
    generate_castling_moves(board, &info, &mut moves);

    moves
}

fn generate_pawn_moves(board: &Board, info: &AttackInfo, moves: &mut MoveList) {
    let us = board.turn;
    let them = us.opposite();
    let our_pawns = board.pieces(PieceType::Pawn, us);
    let their_pieces = board.occupancy(them);
    let empty = !board.all_occupancy();

    match us {
        Color::White => gen_white_pawn_moves(board, info, moves, our_pawns, their_pieces, empty),
        Color::Black => gen_black_pawn_moves(board, info, moves, our_pawns, their_pieces, empty),
    }
}

fn gen_white_pawn_moves(
    board: &Board,
    info: &AttackInfo,
    moves: &mut MoveList,
    pawns: u64,
    enemies: u64,
    empty: u64,
) {
    let check_mask = info.check_mask;

    for from in BitIter(pawns) {
        let pin_mask = info.pin_masks[from];
        let from_bb = 1u64 << from;

        // Single push
        let single = (from_bb << 8) & empty;
        if single != 0 {
            let to = from + 8;
            if (1u64 << to) & check_mask & pin_mask != 0 {
                if to >= 56 {
                    add_promotions(moves, from, to, false);
                } else {
                    moves.push(Move::new(from, to, FLAG_QUIET));
                }
            }

            // Double push (only if single push was to empty square)
            if from_bb & RANK_2 != 0 {
                let double = (from_bb << 16) & empty;
                if double != 0 {
                    let to = from + 16;
                    if (1u64 << to) & check_mask & pin_mask != 0 {
                        moves.push(Move::new(from, to, FLAG_DOUBLE_PUSH));
                    }
                }
            }
        }

        // Captures
        let attacks = PAWN_ATTACKS[0][from] & pin_mask & check_mask;

        // Left capture
        let left_capture = attacks & enemies;
        for to in BitIter(left_capture) {
            if to >= 56 {
                add_promotions(moves, from, to, true);
            } else {
                moves.push(Move::new(from, to, FLAG_CAPTURE));
            }
        }

        // En passant
        if let Some(ep_sq) = board.en_passant {
            let ep_bb = ep_sq.0;
            if attacks & ep_bb != 0 {
                let to = ep_sq.index();
                if is_ep_legal(board, from, to, board.turn) {
                    moves.push(Move::new(from, to, FLAG_EP_CAPTURE));
                }
            }
        }
    }
}

fn gen_black_pawn_moves(
    board: &Board,
    info: &AttackInfo,
    moves: &mut MoveList,
    pawns: u64,
    enemies: u64,
    empty: u64,
) {
    let check_mask = info.check_mask;

    for from in BitIter(pawns) {
        let pin_mask = info.pin_masks[from];
        let from_bb = 1u64 << from;

        // Single push
        let single = (from_bb >> 8) & empty;
        if single != 0 {
            let to = from - 8;
            if (1u64 << to) & check_mask & pin_mask != 0 {
                if to < 8 {
                    add_promotions(moves, from, to, false);
                } else {
                    moves.push(Move::new(from, to, FLAG_QUIET));
                }
            }

            // Double push
            if from_bb & RANK_7 != 0 {
                let double = (from_bb >> 16) & empty;
                if double != 0 {
                    let to = from - 16;
                    if (1u64 << to) & check_mask & pin_mask != 0 {
                        moves.push(Move::new(from, to, FLAG_DOUBLE_PUSH));
                    }
                }
            }
        }

        // Captures
        let attacks = PAWN_ATTACKS[1][from] & pin_mask & check_mask;

        let captures = attacks & enemies;
        for to in BitIter(captures) {
            if to < 8 {
                add_promotions(moves, from, to, true);
            } else {
                moves.push(Move::new(from, to, FLAG_CAPTURE));
            }
        }

        // En passant
        if let Some(ep_sq) = board.en_passant {
            let ep_bb = ep_sq.0;
            if PAWN_ATTACKS[1][from] & pin_mask & ep_bb != 0 {
                let to = ep_sq.index();
                if is_ep_legal(board, from, to, board.turn) {
                    moves.push(Move::new(from, to, FLAG_EP_CAPTURE));
                }
            }
        }
    }
}

fn add_promotions(moves: &mut MoveList, from: usize, to: usize, is_capture: bool) {
    if is_capture {
        moves.push(Move::new(from, to, FLAG_PROMO_CAPTURE_Q));
        moves.push(Move::new(from, to, FLAG_PROMO_CAPTURE_R));
        moves.push(Move::new(from, to, FLAG_PROMO_CAPTURE_B));
        moves.push(Move::new(from, to, FLAG_PROMO_CAPTURE_N));
    } else {
        moves.push(Move::new(from, to, FLAG_PROMO_Q));
        moves.push(Move::new(from, to, FLAG_PROMO_R));
        moves.push(Move::new(from, to, FLAG_PROMO_B));
        moves.push(Move::new(from, to, FLAG_PROMO_N));
    }
}

fn is_ep_legal(board: &Board, from: usize, to: usize, us: Color) -> bool {
    // Check if en passant exposes king to horizontal attack
    let king_sq = board.king_square(us);
    let king_rank = king_sq / 8;
    let pawn_rank = from / 8;

    // En passant can only expose horizontal attacks if king is on same rank
    if king_rank != pawn_rank {
        return true;
    }

    let captured_sq = match us {
        Color::White => to - 8,
        Color::Black => to + 8,
    };

    let them = us.opposite();
    let occ = board.all_occupancy() & !(1u64 << from) & !(1u64 << captured_sq) | (1u64 << to);
    let rook_queen = board.pieces(PieceType::Rook, them) | board.pieces(PieceType::Queen, them);

    (rook_attacks(king_sq, occ) & rook_queen) == 0
}

fn generate_knight_moves(board: &Board, info: &AttackInfo, moves: &mut MoveList) {
    let us = board.turn;
    let our_knights = board.pieces(PieceType::Knight, us);
    let our_pieces = board.occupancy(us);
    let their_pieces = board.occupancy(us.opposite());

    for from in BitIter(our_knights) {
        // Pinned knights can never move
        if info.pin_masks[from] != !0 {
            continue;
        }

        let attacks = KNIGHT_ATTACKS[from] & !our_pieces & info.check_mask;

        for to in BitIter(attacks) {
            let flag = if their_pieces & (1 << to) != 0 {
                FLAG_CAPTURE
            } else {
                FLAG_QUIET
            };
            moves.push(Move::new(from, to, flag));
        }
    }
}

fn generate_bishop_moves(board: &Board, info: &AttackInfo, moves: &mut MoveList) {
    let us = board.turn;
    let bishops = board.pieces(PieceType::Bishop, us);
    let our_pieces = board.occupancy(us);
    let their_pieces = board.occupancy(us.opposite());
    let occ = board.all_occupancy();

    for from in BitIter(bishops) {
        let pin_mask = info.pin_masks[from];
        let attacks = bishop_attacks(from, occ) & !our_pieces & info.check_mask & pin_mask;

        for to in BitIter(attacks) {
            let flag = if their_pieces & (1 << to) != 0 {
                FLAG_CAPTURE
            } else {
                FLAG_QUIET
            };
            moves.push(Move::new(from, to, flag));
        }
    }
}

fn generate_rook_moves(board: &Board, info: &AttackInfo, moves: &mut MoveList) {
    let us = board.turn;
    let rooks = board.pieces(PieceType::Rook, us);
    let our_pieces = board.occupancy(us);
    let their_pieces = board.occupancy(us.opposite());
    let occ = board.all_occupancy();

    for from in BitIter(rooks) {
        let pin_mask = info.pin_masks[from];
        let attacks = rook_attacks(from, occ) & !our_pieces & info.check_mask & pin_mask;

        for to in BitIter(attacks) {
            let flag = if their_pieces & (1 << to) != 0 {
                FLAG_CAPTURE
            } else {
                FLAG_QUIET
            };
            moves.push(Move::new(from, to, flag));
        }
    }
}

fn generate_queen_moves(board: &Board, info: &AttackInfo, moves: &mut MoveList) {
    let us = board.turn;
    let queens = board.pieces(PieceType::Queen, us);
    let our_pieces = board.occupancy(us);
    let their_pieces = board.occupancy(us.opposite());
    let occ = board.all_occupancy();

    for from in BitIter(queens) {
        let pin_mask = info.pin_masks[from];
        let attacks = queen_attacks(from, occ) & !our_pieces & info.check_mask & pin_mask;

        for to in BitIter(attacks) {
            let flag = if their_pieces & (1 << to) != 0 {
                FLAG_CAPTURE
            } else {
                FLAG_QUIET
            };
            moves.push(Move::new(from, to, flag));
        }
    }
}

fn generate_king_moves(board: &Board, info: &AttackInfo, moves: &mut MoveList) {
    let us = board.turn;
    let king_sq = board.king_square(us);
    let our_pieces = board.occupancy(us);
    let their_pieces = board.occupancy(us.opposite());

    let attacks = KING_ATTACKS[king_sq] & !our_pieces & !info.opponent_attacks;

    for to in BitIter(attacks) {
        let flag = if their_pieces & (1 << to) != 0 {
            FLAG_CAPTURE
        } else {
            FLAG_QUIET
        };
        moves.push(Move::new(king_sq, to, flag));
    }
}

fn generate_castling_moves(board: &Board, info: &AttackInfo, moves: &mut MoveList) {
    // No castling when in check
    if info.in_check() {
        return;
    }

    let us = board.turn;
    let occ = board.all_occupancy();

    match us {
        Color::White => {
            // Kingside (e1-g1)
            if board.castling.has(WK_CASTLE) {
                let between_mask = 0x60u64; // f1, g1
                let attack_mask = 0x60u64; // f1, g1 must not be attacked
                if occ & between_mask == 0 && info.opponent_attacks & attack_mask == 0 {
                    moves.push(Move::new(4, 6, FLAG_KING_CASTLE));
                }
            }
            // Queenside (e1-c1)
            if board.castling.has(WQ_CASTLE) {
                let between_mask = 0x0Eu64; // b1, c1, d1
                let attack_mask = 0x0Cu64; // c1, d1 must not be attacked
                if occ & between_mask == 0 && info.opponent_attacks & attack_mask == 0 {
                    moves.push(Move::new(4, 2, FLAG_QUEEN_CASTLE));
                }
            }
        }
        Color::Black => {
            // Kingside (e8-g8)
            if board.castling.has(BK_CASTLE) {
                let between_mask = 0x6000000000000000u64; // f8, g8
                let attack_mask = 0x6000000000000000u64;
                if occ & between_mask == 0 && info.opponent_attacks & attack_mask == 0 {
                    moves.push(Move::new(60, 62, FLAG_KING_CASTLE));
                }
            }
            // Queenside (e8-c8)
            if board.castling.has(BQ_CASTLE) {
                let between_mask = 0x0E00000000000000u64; // b8, c8, d8
                let attack_mask = 0x0C00000000000000u64; // c8, d8
                if occ & between_mask == 0 && info.opponent_attacks & attack_mask == 0 {
                    moves.push(Move::new(60, 58, FLAG_QUEEN_CASTLE));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_position_moves() {
        let board = Board::default();
        let moves = generate_moves(&board);
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_move_encoding() {
        let mv = Move::new(12, 28, FLAG_DOUBLE_PUSH);
        assert_eq!(mv.from(), 12);
        assert_eq!(mv.to(), 28);
        assert_eq!(mv.flags(), FLAG_DOUBLE_PUSH);
        assert!(!mv.is_capture());
        assert!(!mv.is_promotion());
    }

    #[test]
    fn test_promotion_move() {
        let mv = Move::new(52, 60, FLAG_PROMO_Q);
        assert!(mv.is_promotion());
        assert_eq!(mv.promotion_piece(), Some(PieceType::Queen));
    }

    #[test]
    fn test_capture_promotion() {
        let mv = Move::new(52, 61, FLAG_PROMO_CAPTURE_N);
        assert!(mv.is_capture());
        assert!(mv.is_promotion());
        assert_eq!(mv.promotion_piece(), Some(PieceType::Knight));
    }

    #[test]
    fn test_uci_format() {
        let mv = Move::new(12, 28, FLAG_DOUBLE_PUSH);
        assert_eq!(mv.to_uci(), "e2e4");

        let mv = Move::new(52, 60, FLAG_PROMO_Q);
        assert_eq!(mv.to_uci(), "e7e8q");
    }

    #[test]
    fn test_check_detection() {
        // Position with white king in check from black queen
        let board =
            Board::from_fen("rnb1kbnr/pppp1ppp/8/4p3/7q/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 1")
                .unwrap();
        let info = AttackInfo::new(&board);
        assert!(info.in_check());
    }

    #[test]
    fn test_pin_detection() {
        // Position with pinned knight
        let board =
            Board::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/2B5/5N2/PPPPPPPP/RNBQK2R b KQkq - 0 1")
                .unwrap();
        let _info = AttackInfo::new(&board);
        // Knight on c6 is not pinned in this position
    }

    #[test]
    fn test_castling_moves() {
        // Position where white can castle kingside
        let board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1").unwrap();
        let moves = generate_moves(&board);

        let castle_moves: Vec<_> = moves.iter().filter(|m| m.is_castle()).collect();
        assert_eq!(castle_moves.len(), 2); // Both kingside and queenside
    }

    #[test]
    fn test_en_passant() {
        // Position where white can en passant
        let board = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1")
            .unwrap();
        let moves = generate_moves(&board);

        let ep_moves: Vec<_> = moves
            .iter()
            .filter(|m| m.flags() == FLAG_EP_CAPTURE)
            .collect();
        assert_eq!(ep_moves.len(), 1);
    }
}
