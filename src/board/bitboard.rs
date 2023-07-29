use super::{Board, Color, Piece, PieceType, Square};

use std::fmt;

const WHITE_PAWN: usize = 0;
const WHITE_KNIGHT: usize = 1;
const WHITE_BISHOP: usize = 2;
const WHITE_ROOK: usize = 3;
const WHITE_QUEEN: usize = 4;
const WHITE_KING: usize = 5;
const BLACK_PAWN: usize = 6;
const BLACK_KNIGHT: usize = 7;
const BLACK_BISHOP: usize = 8;
const BLACK_ROOK: usize = 9;
const BLACK_QUEEN: usize = 10;
const BLACK_KING: usize = 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub [u64; 12]);

impl Board for Bitboard {
    fn get_piece(&self, square: &Square) -> Option<Piece> {
        let square = square.0;

        for (index, &bitboard) in self.0.iter().enumerate() {
            if bitboard & square != 0 {
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

    fn set_piece(&mut self, square: Square, piece: Piece) {
        self.remove_piece(&square);

        let idx = self.get_piece_idx(piece);

        self.0[idx] |= square.0;
    }

    fn remove_piece(&mut self, square: &Square) {
        let prev_piece = match self.get_piece(square) {
            Some(piece) => piece,
            None => return,
        };

        let idx = self.get_piece_idx(prev_piece);

        self.0[idx] &= !square.0;
    }
}

impl Bitboard {
    fn get_piece_idx(&self, piece: Piece) -> usize {
        match piece {
            Piece(PieceType::Pawn, Color::White) => WHITE_PAWN,
            Piece(PieceType::Knight, Color::White) => WHITE_KNIGHT,
            Piece(PieceType::Bishop, Color::White) => WHITE_BISHOP,
            Piece(PieceType::Rook, Color::White) => WHITE_ROOK,
            Piece(PieceType::Queen, Color::White) => WHITE_QUEEN,
            Piece(PieceType::King, Color::White) => WHITE_KING,
            Piece(PieceType::Pawn, Color::Black) => BLACK_PAWN,
            Piece(PieceType::Knight, Color::Black) => BLACK_KNIGHT,
            Piece(PieceType::Bishop, Color::Black) => BLACK_BISHOP,
            Piece(PieceType::Rook, Color::Black) => BLACK_ROOK,
            Piece(PieceType::Queen, Color::Black) => BLACK_QUEEN,
            Piece(PieceType::King, Color::Black) => BLACK_KING,
        }
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chessboard = [
            '♟', '♞', '♝', '♜', '♛', '♚', // White pieces
            '♙', '♘', '♗', '♖', '♕', '♔', // Black pieces
        ];

        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;

            for file in 0..8 {
                let square = match Square::from(file, rank) {
                    Ok(square) => square,
                    Err(_) => return Err(std::fmt::Error),
                };
                let mut piece_found = false;

                for (index, &bitboard) in self.0.iter().enumerate() {
                    if bitboard & square.0 != 0 {
                        write!(f, "{} ", chessboard[index])?;
                        piece_found = true;
                        break;
                    }
                }

                if !piece_found {
                    write!(f, "· ")?;
                }
            }

            writeln!(f)?;
        }

        writeln!(f, "  a b c d e f g h")?;

        Ok(())
    }
}
