use brainybishop::board::{Board, CastlingRights, Color};

#[cfg(test)]
mod tests {
    use brainybishop::board::{Piece, PieceType, Square};

    use super::*;

    fn default_pieces() -> [u64; 12] {
        [
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
        ]
    }

    #[test]
    fn test_from_fen_default_position() {
        let board =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        assert_eq!(board.bitboard.pieces, default_pieces());
        assert_eq!(board.turn, Color::White);
        assert_eq!(board.castling, CastlingRights(0b1111));
        assert_eq!(board.en_passant, None);
        assert_eq!(board.halfmove, 0);
        assert_eq!(board.fullmove, 1);
    }

    #[test]
    fn test_board_state_default() {
        let board = Board::default();

        assert_eq!(board.bitboard.pieces, default_pieces());
        assert_eq!(board.turn, Color::White);
        assert_eq!(board.castling, CastlingRights(0b1111));
        assert_eq!(board.en_passant, None);
        assert_eq!(board.halfmove, 0);
        assert_eq!(board.fullmove, 1);
    }

    #[test]
    fn test_occupancy_tracking() {
        let board = Board::default();

        let white_expected = 0x000000000000FFFFu64;
        let black_expected = 0xFFFF000000000000u64;

        assert_eq!(board.bitboard.white, white_expected);
        assert_eq!(board.bitboard.black, black_expected);
        assert_eq!(board.bitboard.all, white_expected | black_expected);
    }

    #[test]
    fn test_get_piece_default_position() {
        let board = Board::default().bitboard;

        assert_eq!(
            board.get_piece(&Square::from(0, 0).unwrap()),
            Some(Piece(PieceType::Rook, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(1, 0).unwrap()),
            Some(Piece(PieceType::Knight, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(2, 0).unwrap()),
            Some(Piece(PieceType::Bishop, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(3, 0).unwrap()),
            Some(Piece(PieceType::Queen, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(4, 0).unwrap()),
            Some(Piece(PieceType::King, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(5, 0).unwrap()),
            Some(Piece(PieceType::Bishop, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(6, 0).unwrap()),
            Some(Piece(PieceType::Knight, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(7, 0).unwrap()),
            Some(Piece(PieceType::Rook, Color::White))
        );

        assert_eq!(
            board.get_piece(&Square::from(0, 1).unwrap()),
            Some(Piece(PieceType::Pawn, Color::White))
        );

        assert_eq!(board.get_piece(&Square::from(0, 2).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(4, 4).unwrap()), None);

        assert_eq!(
            board.get_piece(&Square::from(0, 6).unwrap()),
            Some(Piece(PieceType::Pawn, Color::Black))
        );

        assert_eq!(
            board.get_piece(&Square::from(4, 7).unwrap()),
            Some(Piece(PieceType::King, Color::Black))
        );
    }

    #[test]
    fn test_board_helper_methods() {
        let board = Board::default();

        // Test king_square
        assert_eq!(board.king_square(Color::White), 4); // e1
        assert_eq!(board.king_square(Color::Black), 60); // e8

        // Test pieces
        assert_eq!(board.pieces(PieceType::Pawn, Color::White), 0xFF00);
        assert_eq!(board.pieces(PieceType::Pawn, Color::Black), 0x00FF000000000000);

        // Test occupancy
        assert_eq!(board.occupancy(Color::White), 0xFFFF);
        assert_eq!(board.occupancy(Color::Black), 0xFFFF000000000000);
    }
}
