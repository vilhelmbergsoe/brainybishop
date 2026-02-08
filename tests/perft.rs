use brainybishop::board::{Board, Color, Piece, PieceType, Square};
use brainybishop::movegen::{
    generate_moves, Move, FLAG_CAPTURE, FLAG_DOUBLE_PUSH, FLAG_EP_CAPTURE, FLAG_KING_CASTLE,
    FLAG_PROMO_B, FLAG_PROMO_CAPTURE_B, FLAG_PROMO_CAPTURE_N, FLAG_PROMO_CAPTURE_Q,
    FLAG_PROMO_CAPTURE_R, FLAG_PROMO_N, FLAG_PROMO_Q, FLAG_PROMO_R, FLAG_QUEEN_CASTLE,
};

fn make_move(board: &Board, mv: Move) -> Board {
    let mut new_board = *board;
    let from = mv.from();
    let to = mv.to();
    let flags = mv.flags();

    let from_sq = Square::from_index(from);
    let to_sq = Square::from_index(to);

    let piece = new_board.get_piece(&from_sq).unwrap();

    // Remove piece from source
    new_board.remove_piece(&from_sq);

    // Handle captures (remove captured piece)
    if flags == FLAG_CAPTURE || flags >= FLAG_PROMO_CAPTURE_N {
        new_board.remove_piece(&to_sq);
    }

    // Handle en passant capture
    if flags == FLAG_EP_CAPTURE {
        let captured_sq = match board.turn {
            Color::White => Square::from_index(to - 8),
            Color::Black => Square::from_index(to + 8),
        };
        new_board.remove_piece(&captured_sq);
    }

    // Place piece at destination (possibly promoted)
    let dest_piece = match flags {
        FLAG_PROMO_N | FLAG_PROMO_CAPTURE_N => Piece(PieceType::Knight, piece.1),
        FLAG_PROMO_B | FLAG_PROMO_CAPTURE_B => Piece(PieceType::Bishop, piece.1),
        FLAG_PROMO_R | FLAG_PROMO_CAPTURE_R => Piece(PieceType::Rook, piece.1),
        FLAG_PROMO_Q | FLAG_PROMO_CAPTURE_Q => Piece(PieceType::Queen, piece.1),
        _ => piece,
    };
    new_board.set_piece(to_sq, dest_piece);

    // Handle castling - move the rook
    if flags == FLAG_KING_CASTLE {
        match board.turn {
            Color::White => {
                new_board.remove_piece(&Square::from_index(7)); // h1
                new_board.set_piece(Square::from_index(5), Piece(PieceType::Rook, Color::White));
            }
            Color::Black => {
                new_board.remove_piece(&Square::from_index(63)); // h8
                new_board.set_piece(Square::from_index(61), Piece(PieceType::Rook, Color::Black));
            }
        }
    }
    if flags == FLAG_QUEEN_CASTLE {
        match board.turn {
            Color::White => {
                new_board.remove_piece(&Square::from_index(0)); // a1
                new_board.set_piece(Square::from_index(3), Piece(PieceType::Rook, Color::White));
            }
            Color::Black => {
                new_board.remove_piece(&Square::from_index(56)); // a8
                new_board.set_piece(Square::from_index(59), Piece(PieceType::Rook, Color::Black));
            }
        }
    }

    // Update castling rights
    // King moves
    if piece.0 == PieceType::King {
        match board.turn {
            Color::White => {
                new_board.castling.0 &= !0b0011;
            }
            Color::Black => {
                new_board.castling.0 &= !0b1100;
            }
        }
    }
    // Rook moves or captures
    if from == 0 || to == 0 {
        new_board.castling.0 &= !0b0010; // White queenside
    }
    if from == 7 || to == 7 {
        new_board.castling.0 &= !0b0001; // White kingside
    }
    if from == 56 || to == 56 {
        new_board.castling.0 &= !0b1000; // Black queenside
    }
    if from == 63 || to == 63 {
        new_board.castling.0 &= !0b0100; // Black kingside
    }

    // Update en passant square
    new_board.en_passant = if flags == FLAG_DOUBLE_PUSH {
        match board.turn {
            Color::White => Some(Square::from_index(from + 8)),
            Color::Black => Some(Square::from_index(from - 8)),
        }
    } else {
        None
    };

    // Switch turn
    new_board.turn = board.turn.opposite();

    // Update move counters
    if board.turn == Color::Black {
        new_board.fullmove += 1;
    }
    if piece.0 == PieceType::Pawn || flags == FLAG_CAPTURE || flags == FLAG_EP_CAPTURE {
        new_board.halfmove = 0;
    } else {
        new_board.halfmove += 1;
    }

    new_board
}

fn perft(board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = generate_moves(board);

    if depth == 1 {
        return moves.len() as u64;
    }

    let mut nodes = 0u64;
    for mv in moves.iter() {
        let new_board = make_move(board, *mv);
        nodes += perft(&new_board, depth - 1);
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perft_starting_position_depth_1() {
        let board = Board::default();
        assert_eq!(perft(&board, 1), 20);
    }

    #[test]
    fn test_perft_starting_position_depth_2() {
        let board = Board::default();
        assert_eq!(perft(&board, 2), 400);
    }

    #[test]
    fn test_perft_starting_position_depth_3() {
        let board = Board::default();
        assert_eq!(perft(&board, 3), 8902);
    }

    #[test]
    fn test_perft_starting_position_depth_4() {
        let board = Board::default();
        assert_eq!(perft(&board, 4), 197281);
    }

    #[test]
    #[ignore] // This test is slow, run with --ignored
    fn test_perft_starting_position_depth_5() {
        let board = Board::default();
        assert_eq!(perft(&board, 5), 4865609);
    }

    // Kiwipete position - good for testing edge cases
    // r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -
    #[test]
    fn test_perft_kiwipete_depth_1() {
        let board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
                .unwrap();
        assert_eq!(perft(&board, 1), 48);
    }

    #[test]
    fn test_perft_kiwipete_depth_2() {
        let board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
                .unwrap();
        assert_eq!(perft(&board, 2), 2039);
    }

    #[test]
    fn test_perft_kiwipete_depth_3() {
        let board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
                .unwrap();
        assert_eq!(perft(&board, 3), 97862);
    }

    // Position 3 from CPW
    // 8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -
    #[test]
    fn test_perft_position3_depth_1() {
        let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&board, 1), 14);
    }

    #[test]
    fn test_perft_position3_depth_2() {
        let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&board, 2), 191);
    }

    #[test]
    fn test_perft_position3_depth_3() {
        let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&board, 3), 2812);
    }

    // Position with promotions
    // n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - -
    #[test]
    fn test_perft_promotions_depth_1() {
        let board = Board::from_fen("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1").unwrap();
        assert_eq!(perft(&board, 1), 24);
    }

    #[test]
    fn test_perft_promotions_depth_2() {
        let board = Board::from_fen("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1").unwrap();
        assert_eq!(perft(&board, 2), 496);
    }
}
