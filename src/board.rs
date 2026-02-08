use crate::error::{Error, Result};

use super::bitboard::Bitboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[inline(always)]
    pub fn opposite(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline(always)]
    pub fn index(self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece(pub PieceType, pub Color);

impl Piece {
    pub const fn from_char(c: char) -> Option<Piece> {
        match c {
            'p' => Some(Piece(PieceType::Pawn, Color::Black)),
            'n' => Some(Piece(PieceType::Knight, Color::Black)),
            'b' => Some(Piece(PieceType::Bishop, Color::Black)),
            'r' => Some(Piece(PieceType::Rook, Color::Black)),
            'q' => Some(Piece(PieceType::Queen, Color::Black)),
            'k' => Some(Piece(PieceType::King, Color::Black)),
            'P' => Some(Piece(PieceType::Pawn, Color::White)),
            'N' => Some(Piece(PieceType::Knight, Color::White)),
            'B' => Some(Piece(PieceType::Bishop, Color::White)),
            'R' => Some(Piece(PieceType::Rook, Color::White)),
            'Q' => Some(Piece(PieceType::Queen, Color::White)),
            'K' => Some(Piece(PieceType::King, Color::White)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(pub u64);

impl Square {
    pub fn from(file: u8, rank: u8) -> Result<Self> {
        if file > 7 || rank > 7 {
            return Err(Error::InvalidSquare(file, rank));
        }

        Ok(Square(1 << (file + rank * 8)))
    }

    #[inline(always)]
    pub const fn from_index(sq: usize) -> Self {
        debug_assert!(sq < 64);
        Square(1 << sq)
    }

    pub fn from_algebraic(s: &str) -> Result<Self> {
        let algebraic = s.as_bytes();

        if algebraic.len() != 2 {
            return Err(Error::InvalidAlgebraicNotation);
        }

        let file = algebraic[0];
        let rank = algebraic[1];

        if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
            return Err(Error::InvalidAlgebraicNotation);
        }

        Square::from(file - b'a', rank - b'1')
    }

    pub fn to_algebraic(&self) -> String {
        let file = self.file();
        let rank = self.rank();

        format!("{}{}", (file + b'a') as char, (rank + b'1') as char)
    }

    #[inline(always)]
    pub fn file(&self) -> u8 {
        self.0.trailing_zeros() as u8 % 8
    }

    #[inline(always)]
    pub fn rank(&self) -> u8 {
        self.0.trailing_zeros() as u8 / 8
    }

    #[inline(always)]
    pub fn index(&self) -> usize {
        debug_assert!(self.0.count_ones() == 1);
        self.0.trailing_zeros() as usize
    }
}

// Castling rights bit flags
pub const WK_CASTLE: u8 = 0b0001;
pub const WQ_CASTLE: u8 = 0b0010;
pub const BK_CASTLE: u8 = 0b0100;
pub const BQ_CASTLE: u8 = 0b1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights(pub u8);

impl CastlingRights {
    #[inline(always)]
    pub fn has(self, right: u8) -> bool {
        self.0 & right != 0
    }

    #[inline(always)]
    pub fn remove(&mut self, right: u8) {
        self.0 &= !right;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub bitboard: Bitboard,
    pub turn: Color,
    pub en_passant: Option<Square>,
    pub castling: CastlingRights,
    pub halfmove: u16,
    pub fullmove: u64,
}

impl Board {
    pub fn from_fen(fen: &str) -> Result<Self> {
        let mut parts = fen.split_whitespace();

        let piece_placement = parts.next().ok_or(Error::InvalidFen)?;
        let turn = parts.next().ok_or(Error::InvalidFen)?;
        let castling = parts.next().ok_or(Error::InvalidFen)?;
        let en_passant = parts.next().ok_or(Error::InvalidFen)?;
        let halfmove = parts.next().ok_or(Error::InvalidFen)?;
        let fullmove = parts.next().ok_or(Error::InvalidFen)?;

        let mut boardstate = Board {
            bitboard: Bitboard::new(),
            turn: match turn {
                "w" => Color::White,
                "b" => Color::Black,
                _ => return Err(Error::InvalidTurn),
            },
            en_passant: match en_passant {
                "-" => None,
                _ => Some(Square::from_algebraic(en_passant)?),
            },
            castling: match castling {
                "-" => CastlingRights(0b0000),
                _ => {
                    let mut rights = CastlingRights(0);
                    for c in castling.chars() {
                        if let Some(right) = match c {
                            'K' => Some(WK_CASTLE),
                            'Q' => Some(WQ_CASTLE),
                            'k' => Some(BK_CASTLE),
                            'q' => Some(BQ_CASTLE),
                            _ => None,
                        } {
                            rights.0 |= right;
                        }
                    }
                    rights
                }
            },
            halfmove: halfmove.parse().map_err(Error::ParseError)?,
            fullmove: fullmove.parse().map_err(Error::ParseError)?,
        };

        let mut rank = 7;
        let mut file = 0;
        for c in piece_placement.chars() {
            match c {
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                '1'..='8' => file += c as u8 - b'0',
                _ => {
                    if let Some(piece) = Piece::from_char(c) {
                        let sq = Square::from(file, rank)?;
                        boardstate.set_piece(sq, piece);
                        file += 1;
                    } else {
                        return Err(Error::InvalidFen);
                    }
                }
            }
        }

        Ok(boardstate)
    }

    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        self.bitboard.set_piece(square, piece);
    }

    pub fn get_piece(&self, square: &Square) -> Option<Piece> {
        self.bitboard.get_piece(square)
    }

    pub fn remove_piece(&mut self, square: &Square) {
        self.bitboard.remove_piece(square);
    }

    pub fn is_square_empty(&self, square: &Square) -> bool {
        self.bitboard.is_square_empty(square)
    }

    #[inline(always)]
    pub fn pieces(&self, piece_type: PieceType, color: Color) -> u64 {
        self.bitboard.piece_bb(piece_type, color)
    }

    #[inline(always)]
    pub fn occupancy(&self, color: Color) -> u64 {
        self.bitboard.occupancy(color)
    }

    #[inline(always)]
    pub fn all_occupancy(&self) -> u64 {
        self.bitboard.all
    }

    #[inline(always)]
    pub fn king_square(&self, color: Color) -> usize {
        let king_bb = self.pieces(PieceType::King, color);
        debug_assert!(king_bb != 0);
        debug_assert!(king_bb.count_ones() == 1);
        king_bb.trailing_zeros() as usize
    }
}

impl std::default::Default for Board {
    fn default() -> Self {
        let pieces = [
            0x000000000000FF00, // White pawns
            0x0000000000000042, // White knights
            0x0000000000000024, // White bishops
            0x0000000000000081, // White rooks
            0x0000000000000008, // White queen
            0x0000000000000010, // White king
            0x00FF000000000000, // Black pawns
            0x4200000000000000, // Black knights
            0x2400000000000000, // Black bishops
            0x8100000000000000, // Black rooks
            0x0800000000000000, // Black queen
            0x1000000000000000, // Black king
        ];

        Self {
            bitboard: Bitboard::from_pieces(pieces),
            turn: Color::White,
            castling: CastlingRights(0b1111),
            en_passant: None,
            halfmove: 0,
            fullmove: 1,
        }
    }
}
