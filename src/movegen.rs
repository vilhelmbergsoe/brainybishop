#![allow(dead_code)]

use crate::board::{Board, Color, PieceType, Square};

const A_FILE: u64 = 0x0101010101010101;
const B_FILE: u64 = 0x0202020202020202;
const C_FILE: u64 = 0x0404040404040404;
const D_FILE: u64 = 0x0808080808080808;
const E_FILE: u64 = 0x1010101010101010;
const F_FILE: u64 = 0x2020202020202020;
const G_FILE: u64 = 0x4040404040404040;
const H_FILE: u64 = 0x8080808080808080;

const RANK_8: u64 = 0xFF00000000000000;
const RANK_7: u64 = 0x00FF000000000000;
const RANK_6: u64 = 0x0000FF0000000000;
const RANK_5: u64 = 0x000000FF00000000;
const RANK_4: u64 = 0x00000000FF000000;
const RANK_3: u64 = 0x0000000000FF0000;
const RANK_2: u64 = 0x000000000000FF00;
const RANK_1: u64 = 0x00000000000000FF;


// from should only represent a single square
// to is a bitboard of all possible moves / squares to move to
#[derive(Debug)]
pub struct Moves {
    pub from: Square,
    pub to: Square,
}

impl Moves {
    pub fn to_algebraic(&self) -> String {
        const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        const RANKS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

        let mut algebraic_squares = String::new();
        let mut bitboard = self.to.0;

        while bitboard != 0 {
            let square_idx = bitboard.trailing_zeros();
            let file = square_idx as usize % 8;
            let rank = square_idx as usize / 8;

            algebraic_squares.push(FILES[file]);
            algebraic_squares.push(RANKS[rank]);

            bitboard &= bitboard - 1; // Clear the least significant bit.
            if bitboard != 0 {
                algebraic_squares.push(',');
            }
        }

        algebraic_squares
    }
}

pub struct MoveGen;

impl MoveGen {
    // Gets pseudo-legal moves for a given piece
    pub fn get_moves(board: &Board, square: &Square) -> Option<Moves> {
        let piece = board.bitboard.get_piece(square).unwrap();

        // MAYBE MAKE THIS JUST RETURN THEIR MOVE TABLE AND THEN CHECK IF IT'S LEGAL AFTER IN ONE FUNCTION

        // MAYBE PRECALCULATE THE STATIC MOVES AND THEN DO BOUNDSCHECK AND LEGALITY CHECK AFTER

        match piece.0 {
            PieceType::Pawn => MoveGen::get_pawn_moves(board, square, piece.1),
            PieceType::Knight => MoveGen::get_knight_moves(board, square),
            // PieceType::Bishop => MoveGen::get_bishop_moves(board, square),
            // PieceType::Rook => MoveGen::get_rook_moves(board, square),
            // PieceType::Queen => MoveGen::get_queen_moves(board, square),
            PieceType::King => MoveGen::get_king_moves(board, square),
            _ => unimplemented!(),
        }
    }
    pub fn get_pawn_moves(board: &Board, square: &Square, color: Color) -> Option<Moves> {
        let mut moves: u64 = 0;

        match color {
            Color::White => {
                let move_up = square.0 << 8;
                let move_up_2 = square.0 << 16;
                let capture_right = square.0 << 7;
                let capture_left = square.0 << 9;

                if board.is_square_empty(&Square(move_up)) && (square.0 & RANK_8 == 0) {
                    moves |= move_up;
                }
                if board.is_square_empty(&Square(move_up))
                    && board.is_square_empty(&Square(move_up_2))
                    && (square.0 & RANK_2 != 0)
                {
                    moves |= move_up_2;
                }
                if !board.is_square_empty(&Square(capture_right)) && (square.0 & H_FILE == 0) {
                    moves |= capture_right;
                }
                if !board.is_square_empty(&Square(capture_left)) && (square.0 & A_FILE == 0) {
                    moves |= capture_left;
                }
            }
            Color::Black => {
                let move_up = square.0 >> 8;
                let move_up_2 = square.0 >> 16;
                let capture_right = square.0 >> 7;
                let capture_left = square.0 >> 9;

                if board.is_square_empty(&Square(move_up)) && (square.0 & RANK_1 == 0) {
                    moves |= move_up;
                }
                if board.is_square_empty(&Square(move_up_2)) && (square.0 & RANK_7 != 0) {
                    moves |= move_up_2;
                }
                if !board.is_square_empty(&Square(capture_right)) && (square.0 & A_FILE == 0) {
                    moves |= capture_right;
                }
                if !board.is_square_empty(&Square(capture_left)) && (square.0 & H_FILE == 0) {
                    moves |= capture_left;
                }
            }
        }

        if moves == 0 {
            return None;
        }

        Some(Moves {
            from: *square,
            to: Square(moves),
        })
    }
    pub fn get_king_moves(board: &Board, square: &Square) -> Option<Moves> {
        let piece = match board.get_piece(&square) {
            Some(piece) => piece,
            None => return None,
        };

        let mut moves: u64 = 0;

        if A_FILE & square.0 == 0 {
            // left
            moves |= square.0 >> 1;

            if RANK_8 & square.0 == 0 {
                // down-left
                moves |= square.0 >> 9;
            }

            if RANK_1 & square.0 == 0 {
                // up-left
                moves |= square.0 << 7;
            }
        }

        if H_FILE & square.0 == 0 {
            // right
            moves |= square.0 << 1;

            if RANK_8 & square.0 == 0 {
                // up-right
                moves |= square.0 << 9;
            }

            if RANK_1 & square.0 == 0 {
                // down-right
                moves |= square.0 >> 7;
            }
        }

        if RANK_8 & square.0 == 0 {
            // up
            moves |= square.0 << 8;
        }

        if RANK_1 & square.0 == 0 {
            // down
            moves |= square.0 >> 8;
        }

        moves = MoveGen::filter_same_color_moves(board, moves, piece.1);

        if moves == 0 {
            return None;
        }

        Some(Moves {
            from: *square,
            to: Square(moves),
        })
    }

    fn get_knight_moves(board: &Board, square: &Square) -> Option<Moves> {
        let piece = match board.get_piece(square) {
            Some(piece) => piece,
            None => return None,
        };

        let mut moves: u64 = 0;

        // not 8th rank and not a-bfile
        //
        // FIND A WAY TO DO BOUNDS CHECKS HERE AND THEN DO COLOR CHECKS AFTER

        if (A_FILE | B_FILE) & square.0 == 0 {
            if RANK_8 & square.0 == 0 {
                // left-up
                moves |= square.0 << 6;
            }
            if RANK_1 & square.0 == 0 {
                // left-down
                moves |= square.0 >> 10;
            }
        }

        if (G_FILE | H_FILE) & square.0 == 0 {
            if RANK_8 & square.0 == 0 {
                // right-up
                moves |= square.0 << 10;
            }
            if RANK_1 & square.0 == 0 {
                // right-down
                moves |= square.0 >> 6;
            }
        }

        if (RANK_8 | RANK_7) & square.0 == 0 {
            if A_FILE & square.0 == 0 {
                // up-left
                moves |= square.0 << 15;
            }
            if G_FILE & square.0 == 0 {
                // up-right
                moves |= square.0 << 17;
            }
        }


        if (RANK_1 | RANK_2) & square.0 == 0 {
            if A_FILE & square.0 == 0 {
                // down-left
                moves |= square.0 >> 17;
            }
            if G_FILE & square.0 == 0 {
                // down-right
                moves |= square.0 >> 15;
            }
        }

        moves = MoveGen::filter_same_color_moves(board, moves, piece.1);

        Some(Moves {
            from: *square,
            to: Square(moves),
        })
    }

    pub fn filter_same_color_moves(board: &Board, moves: u64, color: Color) -> u64 {
        let mut filtered_moves: u64 = 0;
        let mut bitboard = moves;

        while bitboard != 0 {
            let square_idx = bitboard.trailing_zeros();
            let square = Square(1 << square_idx);

            if let Some(piece) = board.get_piece(&square) {
                if piece.1 != color {
                    filtered_moves |= square.0;
                }
            } else {
                filtered_moves |= square.0;
            }

            bitboard &= bitboard - 1; // Clear the least significant bit.
        }

        filtered_moves
    }
}
