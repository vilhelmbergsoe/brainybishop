use color_eyre::{eyre::eyre, Result};

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

#[derive(Debug, Eq, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn from_char(c: char) -> Option<(Piece, Color)> {
        match c {
            'p' => Some((Piece::Pawn, Color::Black)),
            'n' => Some((Piece::Knight, Color::Black)),
            'b' => Some((Piece::Bishop, Color::Black)),
            'r' => Some((Piece::Rook, Color::Black)),
            'q' => Some((Piece::Queen, Color::Black)),
            'k' => Some((Piece::King, Color::Black)),
            'P' => Some((Piece::Pawn, Color::White)),
            'N' => Some((Piece::Knight, Color::White)),
            'B' => Some((Piece::Bishop, Color::White)),
            'R' => Some((Piece::Rook, Color::White)),
            'Q' => Some((Piece::Queen, Color::White)),
            'K' => Some((Piece::King, Color::White)),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug)]
pub struct Square(u8);

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}

impl Square {
    pub fn from(file: u8, rank: u8) -> Square {
        Square(rank * 8 + file)
    }

    pub fn from_algebraic(algebraic: &str) -> Option<Square> {
        let algebraic = algebraic.as_bytes();

        if algebraic.len() != 2 {
            return None;
        }

        let file = algebraic[0];
        let rank = algebraic[1];

        if file < b'a' || file > b'h' || rank < b'1' || rank > b'8' {
            return None;
        }

        Some(Square::from(file - b'a', rank - b'1'))
    }

    pub fn to_algebraic(&self) -> String {
        let file = self.0 % 8;
        let rank = self.0 / 8;

        let mut algebraic = String::new();

        algebraic.push((file + b'a') as char);
        algebraic.push((rank + b'1') as char);

        algebraic
    }
}

pub struct Board {
    // pub castling_ability: SOMETHING?
    // 0-5: white, 6-11: black
    // 0: pawn, 1: knight, 2: bishop, 3: rook, 4: queen, 5: king
    // 6: pawn, 7: knight, 8: bishop, 9: rook, 10: queen, 11: king
    pub bitboard: [u64; 12],
    pub side_to_move: Color,
    pub en_passant_square: Option<Square>,
    pub halfmove_clock: u64,
    pub fullmove_count: u64,
}

impl std::default::Default for Board {
    fn default() -> Self {
        Board {
            // Default starting position
            bitboard: [
                0xFF00,
                0x42,
                0x24,
                0x81,
                0x08,
                0x16,
                0xFF000000000000,
                0x4200000000000000,
                0x2400000000000000,
                0x8100000000000000,
                0x0800000000000000,
                0x1600000000000000,
            ],

            side_to_move: Color::White,
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_count: 0,
        }
    }
}

impl Board {
    pub fn from_fen(fen: &str) -> Result<Board> {
        let fen: Vec<_> = fen.split_whitespace().collect();

        // TODO castling rights
        let (
            piece_placement,
            active_color,
            castling_rights,
            en_passant_square,
            halfmove_clock,
            fullmove_number,
        ) = match fen.as_slice() {
            [piece_placement, active_color, castling_rights, en_passant_square, halfmove_clock, fullmove_number] => {
                (
                    piece_placement,
                    active_color,
                    castling_rights,
                    en_passant_square,
                    halfmove_clock,
                    fullmove_number,
                )
            }
            _ => return Err(eyre!("Invalid FEN")),
        };

        let mut board = Board {
            bitboard: [0; 12],
            side_to_move: match *active_color {
                "w" => Color::White,
                "b" => Color::Black,
                _ => return Err(eyre!("Invalid FEN")),
            },
            en_passant_square: match *en_passant_square {
                "-" => None,
                _ => Some(Square::from_algebraic(en_passant_square).unwrap()),
            },
            halfmove_clock: halfmove_clock.parse()?,
            fullmove_count: fullmove_number.parse()?,
        };

        // Start at the top left corner (a8)
        let mut rank = 7;
        let mut file = 0;

        let ranks = piece_placement.split("/");

        for v in ranks {
            for c in v.chars() {
                if let Some(piece) = Piece::from_char(c) {
                    board.set_piece_at(Square::from(file, rank), piece);
                    file += 1;
                } else if let Some(digit) = c.to_digit(10) {
                    file += digit as u8;
                } else {
                    return Err(eyre!("Invalid FEN"));
                }
            }

            if rank != 0 {
                rank -= 1;
                file = 0;
            }
        }

        Ok(board)
    }

    pub fn get_piece_at(&self, square: &Square) -> Option<(Piece, Color)> {
        let square = square.0;

        if square > 63 {
            return None;
        }

        for (index, &bitboard) in self.bitboard.iter().enumerate() {
            if bitboard & (1u64 << square) != 0 {
                match index {
                    WHITE_PAWN => return Some((Piece::Pawn, Color::White)),
                    BLACK_PAWN => return Some((Piece::Pawn, Color::Black)),
                    WHITE_KNIGHT => return Some((Piece::Knight, Color::White)),
                    BLACK_KNIGHT => return Some((Piece::Knight, Color::Black)),
                    WHITE_BISHOP => return Some((Piece::Bishop, Color::White)),
                    BLACK_BISHOP => return Some((Piece::Bishop, Color::Black)),
                    WHITE_ROOK => return Some((Piece::Rook, Color::White)),
                    BLACK_ROOK => return Some((Piece::Rook, Color::Black)),
                    WHITE_QUEEN => return Some((Piece::Queen, Color::White)),
                    BLACK_QUEEN => return Some((Piece::Queen, Color::Black)),
                    WHITE_KING => return Some((Piece::King, Color::White)),
                    BLACK_KING => return Some((Piece::King, Color::Black)),
                    _ => unreachable!(),
                };
            }
        }

        None
    }
    pub fn set_piece_at(&mut self, square: Square, piece: (Piece, Color)) {
        if square.0 > 63 {
            return;
        }

        self.remove_piece_at(&square);

        let idx = self.get_piece_idx(piece);

        self.bitboard[idx] |= 1u64 << square.0;
    }
    pub fn remove_piece_at(&mut self, square: &Square) {
        if square.0 > 63 {
            return;
        }

        let prev_piece = match self.get_piece_at(square) {
            Some(v) => v,
            None => return,
        };

        let idx = self.get_piece_idx(prev_piece);

        self.bitboard[idx] &= !(1u64 << square.0);
    }
    pub fn move_piece(&mut self, from: Square, to: Square) {
        if from.0 > 63 || to.0 > 63 {
            return;
        }

        let piece = self.get_piece_at(&from).unwrap();

        self.remove_piece_at(&from);
        self.remove_piece_at(&to);

        self.set_piece_at(to, piece);
    }
    pub fn get_piece_idx(&self, piece: (Piece, Color)) -> usize {
        match piece {
            (Piece::Pawn, Color::White) => WHITE_PAWN,
            (Piece::Knight, Color::White) => WHITE_KNIGHT,
            (Piece::Bishop, Color::White) => WHITE_BISHOP,
            (Piece::Rook, Color::White) => WHITE_ROOK,
            (Piece::Queen, Color::White) => WHITE_QUEEN,
            (Piece::King, Color::White) => WHITE_KING,
            (Piece::Pawn, Color::Black) => BLACK_PAWN,
            (Piece::Knight, Color::Black) => BLACK_KNIGHT,
            (Piece::Bishop, Color::Black) => BLACK_BISHOP,
            (Piece::Rook, Color::Black) => BLACK_ROOK,
            (Piece::Queen, Color::Black) => BLACK_QUEEN,
            (Piece::King, Color::Black) => BLACK_KING,
        }
    }
    pub fn get_legal_moves(&self, square: Square) -> Option<Vec<Square>> {
        let (piece, color) = match self.get_piece_at(&square) {
            Some((piece, color)) => (piece, color),
            None => return None, // No piece at the given square
        };

        match piece {
            Piece::Pawn => self.get_pawn_moves(square, color),
            // Piece::King => self.get_king_moves(square, color),
            _ => todo!(),
        }
    }

    // pub fn get_king_moves(&self, square: Square, color: Color) -> Option<Vec<Square>> {
    //     let mut moves: Vec<Square> = Vec::new();

    //     let directions = [
    //         (1, 1),
    //         (1, 0),
    //         (1, -1),
    //         (0, 1),
    //         (0, -1),
    //         (-1, 1),
    //         (-1, 0),
    //         (-1, -1),
    //     ];

    //     for (x, y) in directions.iter() {
    //         if square.0 + x < 0 || target_square.0 > 63 {
    //             continue;
    //         }

    //         let target_square = Square::from(
    //             (square.0 as i8 + x) as u8,
    //             (square.0 as i8 + y) as u8,
    //         );

    //         let target_piece = self.get_piece_at(&target_square);
    //         if target_piece.is_none() || target_piece.unwrap().1 != color {
    //             moves.push(target_square);
    //         }
    //     }

    //     Some(moves)
    // }

    pub fn get_pawn_moves(&self, square: Square, color: Color) -> Option<Vec<Square>> {
        let direction = match color {
            Color::White => 1,
            Color::Black => -1,
        };

        let mut moves: Vec<Square> = Vec::new();

        // Normal pawn move (one square forward)
        let forward_one = square.0 as i8 + direction * 8;
        if forward_one >= 0 && forward_one < 64 {
            let target_square = Square(forward_one as u8);
            if self.get_piece_at(&target_square).is_none() {
                moves.push(target_square);
            }
        }

        // Double pawn move (two squares forward from starting position)
        // TODO Set a passant square in game state for one move if double move is played
        let starting_rank = match color {
            Color::White => 0xFF00,
            Color::Black => 0xFF000000000000,
        };
        if (2_u64.pow(square.0.into())) & starting_rank != 0 {
            let forward_two = square.0 as i8 + (2 * direction * 8);
            if forward_two >= 0 && forward_two < 64 {
                let target_square = Square(forward_two as u8);
                if self.get_piece_at(&target_square).is_none() {
                    moves.push(target_square);
                }
            }
        }

        // Capture moves
        let left_capture = square.0 as i8 + direction * 8 - 1;
        let right_capture = square.0 as i8 + direction * 8 + 1;
        let rank = square.0 % 8;

        if left_capture >= 0 && left_capture < 64 && ((left_capture % 8) < rank as i8) {
            let target_square = Square(left_capture as u8);
            if let Some((captured_piece, captured_color)) = self.get_piece_at(&target_square) {
                if color != captured_color {
                    moves.push(target_square);
                }
            }
        }
        if right_capture >= 0 && right_capture < 64 && ((right_capture % 8) > rank as i8) {
            let target_square = Square(right_capture as u8);
            if let Some((captured_piece, captured_color)) = self.get_piece_at(&target_square) {
                if color != captured_color {
                    moves.push(target_square);
                }
            }
        }

        // En passant (to be implemented)

        // Promotion (to be implemented)

        if moves.is_empty() {
            None
        } else {
            Some(moves)
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chessboard = [
            '♟', '♞', '♝', '♜', '♛', '♚', // White pieces
            '♙', '♘', '♗', '♖', '♕', '♔', // Black pieces
        ];

        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;

            for file in 0..8 {
                let square_index = rank * 8 + file;
                let mut piece_found = false;

                for (index, &bitboard) in self.bitboard.iter().enumerate() {
                    if bitboard & (1u64 << square_index) != 0 {
                        write!(f, "{} ", chessboard[index])?;
                        piece_found = true;
                        break;
                    }
                }

                if !piece_found {
                    write!(f, "  ")?;
                }
            }

            writeln!(f)?;
        }

        writeln!(f, "  a b c d e f g h")?;

        Ok(())
    }
}
