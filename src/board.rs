use color_eyre::eyre::{eyre, Result};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece(pub PieceType, pub Color);

impl Piece {
    const fn from_char(c: char) -> Option<Piece> {
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
            return Err(eyre!("Invalid square: {}{}", file, rank));
        }

        Ok(Square(1 << (file + rank * 8)))
    }

    pub fn from_algebraic(s: &str) -> Result<Self> {
        let algebraic = s.as_bytes();

        if algebraic.len() != 2 {
            return Err(eyre!("Invalid algebraic notation: '{}'", s));
        }

        let file = algebraic[0];
        let rank = algebraic[1];

        if
        // file < b'a' || file > b'h'
        !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank)
        // rank < b'1' || rank > b'8'
        {
            return Err(eyre!("Invalid algebraic notation: '{}'", s));
        }

        Square::from(file - b'a', rank - b'1')
    }

    pub fn to_algebraic(&self) -> String {
        let file = self.file();
        let rank = self.rank();

        format!("{}{}", (file + b'a') as char, (rank + b'1') as char)
    }

    pub fn file(&self) -> u8 {
        self.0.trailing_zeros() as u8 % 8
    }

    pub fn rank(&self) -> u8 {
        self.0.trailing_zeros() as u8 / 8
    }
}

// pub trait Board {
//     fn get_piece(&self, square: &Square) -> Option<Piece>;
//     fn set_piece(&mut self, square: Square, piece: Piece);
//     fn remove_piece(&mut self, square: &Square);
// }

// 0b0000: none
// 0b0001: white kingside
// 0b0010: white queenside
// 0b0100: black kingside
// 0b1000: black queenside
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights(pub u8);

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

        let piece_placement = parts.next().ok_or_else(|| eyre!("Invalid FEN"))?;
        let turn = parts.next().ok_or_else(|| eyre!("Invalid FEN"))?;
        let castling = parts.next().ok_or_else(|| eyre!("Invalid FEN"))?;
        let en_passant = parts.next().ok_or_else(|| eyre!("Invalid FEN"))?;
        let halfmove = parts.next().ok_or_else(|| eyre!("Invalid FEN"))?;
        let fullmove = parts.next().ok_or_else(|| eyre!("Invalid FEN"))?;

        let mut boardstate = Board {
            bitboard: Bitboard([0; 12]),
            turn: match turn {
                "w" => Color::White,
                "b" => Color::Black,
                _ => return Err(eyre!("Error parsing turn-to-move: '{}'", turn)),
            },
            en_passant: match en_passant {
                "-" => None,
                _ => Some(Square::from_algebraic(en_passant).unwrap()),
            },
            castling: match castling {
                "-" => CastlingRights(0b0000),
                _ => {
                    let mut rights = CastlingRights(0);
                    for c in castling.chars() {
                        if let Some(right) = match c {
                            'K' => Some(0b0001),
                            'Q' => Some(0b0010),
                            'k' => Some(0b0100),
                            'q' => Some(0b1000),
                            _ => None,
                        } {
                            rights.0 |= right;
                        }
                    }
                    rights
                }
            },
            halfmove: halfmove.parse()?,
            fullmove: fullmove.parse()?,
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
                        return Err(eyre!("Invalid FEN"));
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
}

impl std::default::Default for Board {
    fn default() -> Self {
        Self {
            // Default starting position
            bitboard: Bitboard([
                0xFF00,
                0x42,
                0x24,
                0x81,
                0x08,
                0x10,
                0xFF000000000000,
                0x4200000000000000,
                0x2400000000000000,
                0x8100000000000000,
                0x0800000000000000,
                0x1000000000000000,
            ]),

            turn: Color::White,
            castling: CastlingRights(0b1111),
            en_passant: None,
            halfmove: 0,
            fullmove: 1,
        }
    }
}
