use super::board::{Color, Piece, PieceType, Square};

pub const WHITE_PAWN: usize = 0;
pub const WHITE_KNIGHT: usize = 1;
pub const WHITE_BISHOP: usize = 2;
pub const WHITE_ROOK: usize = 3;
pub const WHITE_QUEEN: usize = 4;
pub const WHITE_KING: usize = 5;
pub const BLACK_PAWN: usize = 6;
pub const BLACK_KNIGHT: usize = 7;
pub const BLACK_BISHOP: usize = 8;
pub const BLACK_ROOK: usize = 9;
pub const BLACK_QUEEN: usize = 10;
pub const BLACK_KING: usize = 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard {
    pub pieces: [u64; 12],
    pub white: u64,
    pub black: u64,
    pub all: u64,
}

impl Bitboard {
    pub fn new() -> Self {
        Self {
            pieces: [0; 12],
            white: 0,
            black: 0,
            all: 0,
        }
    }

    pub fn from_pieces(pieces: [u64; 12]) -> Self {
        let white = pieces[0] | pieces[1] | pieces[2] | pieces[3] | pieces[4] | pieces[5];
        let black = pieces[6] | pieces[7] | pieces[8] | pieces[9] | pieces[10] | pieces[11];
        Self {
            pieces,
            white,
            black,
            all: white | black,
        }
    }

    fn update_occupancy(&mut self) {
        self.white = self.pieces[0]
            | self.pieces[1]
            | self.pieces[2]
            | self.pieces[3]
            | self.pieces[4]
            | self.pieces[5];
        self.black = self.pieces[6]
            | self.pieces[7]
            | self.pieces[8]
            | self.pieces[9]
            | self.pieces[10]
            | self.pieces[11];
        self.all = self.white | self.black;
    }

    pub fn get_piece(&self, square: &Square) -> Option<Piece> {
        debug_assert!(square.0.count_ones() == 1);

        let sq = square.0;

        for (index, &bitboard) in self.pieces.iter().enumerate() {
            if bitboard & sq != 0 {
                return match index {
                    WHITE_PAWN => Some(Piece(PieceType::Pawn, Color::White)),
                    WHITE_KNIGHT => Some(Piece(PieceType::Knight, Color::White)),
                    WHITE_BISHOP => Some(Piece(PieceType::Bishop, Color::White)),
                    WHITE_ROOK => Some(Piece(PieceType::Rook, Color::White)),
                    WHITE_QUEEN => Some(Piece(PieceType::Queen, Color::White)),
                    WHITE_KING => Some(Piece(PieceType::King, Color::White)),
                    BLACK_PAWN => Some(Piece(PieceType::Pawn, Color::Black)),
                    BLACK_KNIGHT => Some(Piece(PieceType::Knight, Color::Black)),
                    BLACK_BISHOP => Some(Piece(PieceType::Bishop, Color::Black)),
                    BLACK_ROOK => Some(Piece(PieceType::Rook, Color::Black)),
                    BLACK_QUEEN => Some(Piece(PieceType::Queen, Color::Black)),
                    BLACK_KING => Some(Piece(PieceType::King, Color::Black)),
                    _ => unreachable!(),
                };
            }
        }

        None
    }

    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        debug_assert!(square.0.count_ones() == 1);

        self.remove_piece(&square);

        let idx = piece_to_index(piece);
        self.pieces[idx] |= square.0;
        self.update_occupancy();
    }

    pub fn remove_piece(&mut self, square: &Square) {
        debug_assert!(square.0.count_ones() == 1);

        let prev_piece = match self.get_piece(square) {
            Some(piece) => piece,
            None => return,
        };

        let idx = piece_to_index(prev_piece);
        self.pieces[idx] &= !square.0;
        self.update_occupancy();
    }

    pub fn is_square_empty(&self, square: &Square) -> bool {
        debug_assert!(square.0.count_ones() == 1);
        self.all & square.0 == 0
    }

    #[inline(always)]
    pub fn piece_bb(&self, piece_type: PieceType, color: Color) -> u64 {
        let idx = piece_type_color_to_index(piece_type, color);
        self.pieces[idx]
    }

    #[inline(always)]
    pub fn occupancy(&self, color: Color) -> u64 {
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }
}

#[inline(always)]
pub fn piece_to_index(piece: Piece) -> usize {
    piece_type_color_to_index(piece.0, piece.1)
}

#[inline(always)]
pub fn piece_type_color_to_index(piece_type: PieceType, color: Color) -> usize {
    let base = match piece_type {
        PieceType::Pawn => 0,
        PieceType::Knight => 1,
        PieceType::Bishop => 2,
        PieceType::Rook => 3,
        PieceType::Queen => 4,
        PieceType::King => 5,
    };
    match color {
        Color::White => base,
        Color::Black => base + 6,
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chessboard = [
            '♟', '♞', '♝', '♜', '♛', '♚', // White pieces (indices 0-5)
            '♙', '♘', '♗', '♖', '♕', '♔', // Black pieces (indices 6-11)
        ];

        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;

            for file in 0..8 {
                let sq_bit = 1u64 << (file + rank * 8);
                let mut piece_found = false;

                for (index, &bitboard) in self.pieces.iter().enumerate() {
                    if bitboard & sq_bit != 0 {
                        write!(f, "{} ", chessboard[index])?;
                        piece_found = true;
                        break;
                    }
                }

                if !piece_found {
                    write!(f, ". ")?;
                }
            }

            writeln!(f)?;
        }

        writeln!(f, "  a b c d e f g h")?;

        Ok(())
    }
}

// Iterator over set bits in a bitboard
pub struct BitIter(pub u64);

impl Iterator for BitIter {
    type Item = usize;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let sq = self.0.trailing_zeros() as usize;
            self.0 &= self.0 - 1;
            Some(sq)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_occupancy_tracking() {
        let mut bb = Bitboard::new();
        assert_eq!(bb.white, 0);
        assert_eq!(bb.black, 0);
        assert_eq!(bb.all, 0);

        let e2 = Square::from(4, 1).unwrap();
        bb.set_piece(e2, Piece(PieceType::Pawn, Color::White));

        assert_eq!(bb.white, e2.0);
        assert_eq!(bb.black, 0);
        assert_eq!(bb.all, e2.0);

        let e7 = Square::from(4, 6).unwrap();
        bb.set_piece(e7, Piece(PieceType::Pawn, Color::Black));

        assert_eq!(bb.white, e2.0);
        assert_eq!(bb.black, e7.0);
        assert_eq!(bb.all, e2.0 | e7.0);
    }

    #[test]
    fn test_bit_iter() {
        let bb = 0b1010_0101u64;
        let squares: Vec<usize> = BitIter(bb).collect();
        assert_eq!(squares, vec![0, 2, 5, 7]);
    }
}
