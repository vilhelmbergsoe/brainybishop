use crate::board::{Board, Square};

// Acts as a wrapper for a Square / multiple squares
// e.g. Square1 | Square2 means to possible moves
type Move = Square;

struct MoveGen;

impl MoveGen {
    // Gets pseudo-legal moves for a given piece
    fn get_moves(board: &Board, square: &Square) -> Move {
        let piece = board.get_piece(square).unwrap();

        match piece.0 {
            PieceType::Pawn => MoveGen::get_pawn_moves(board, square),
            PieceType::Knight => MoveGen::get_knight_moves(board, square),
            PieceType::Bishop => MoveGen::get_bishop_moves(board, square),
            PieceType::Rook => MoveGen::get_rook_moves(board, square),
            PieceType::Queen => MoveGen::get_queen_moves(board, square),
            PieceType::King => MoveGen::get_king_moves(board, square),
        }
    }
    fn get_pawn_moves(board: &Board, square: &Square) -> Move {
        let piece = board.get_piece(square).unwrap();

        let moves: u64 = 0;

        let mut move_up = square << 8;
        let mut move_up_2 = square << 16;
        let mut capture_left = square << 7;
        let mut capture_right = square << 9;

        if piece.color == Color::White {
            move_up = square >> 8;
            move_up_2 = square >> 16;
            capture_left = square >> 7;
            capture_right = square >> 9;
        }

        if board.get_piece(&move_up).is_none() {
            moves |= move_up;
        }

        if board.get_piece(&move_up_2).is_none() {
            moves |= move_up_2;
        }

        if board.get_piece(&capture_left).is_some() {
            moves |= capture_left;
        }

        if board.get_piece(&capture_right).is_some() && square.file() != 7 {
            moves |= capture_right;
        }

        // something like this^
    }
}
