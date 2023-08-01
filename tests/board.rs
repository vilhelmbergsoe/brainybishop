use brainybishop::bitboard::Bitboard;
use brainybishop::board::{Board, CastlingRights, Color};

#[cfg(test)]
mod tests {
    use brainybishop::board::{Piece, PieceType, Square};

    use super::*;

    #[test]
    fn test_from_fen_default_position() {
        let board =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap();

        assert_eq!(
            board,
            Board {
                board: Bitboard([
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
        );
    }

    #[test]
    fn test_board_state_default() {
        let board = Board::default();

        assert_eq!(
            board,
            Board {
                // Default starting position
                board: Bitboard([
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
        );
    }

    #[test]
    fn test_get_piece_default_position() {
        let board = Board::default().board;

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
        assert_eq!(
            board.get_piece(&Square::from(1, 1).unwrap()),
            Some(Piece(PieceType::Pawn, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(2, 1).unwrap()),
            Some(Piece(PieceType::Pawn, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(3, 1).unwrap()),
            Some(Piece(PieceType::Pawn, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(4, 1).unwrap()),
            Some(Piece(PieceType::Pawn, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(5, 1).unwrap()),
            Some(Piece(PieceType::Pawn, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(6, 1).unwrap()),
            Some(Piece(PieceType::Pawn, Color::White))
        );
        assert_eq!(
            board.get_piece(&Square::from(7, 1).unwrap()),
            Some(Piece(PieceType::Pawn, Color::White))
        );

        assert_eq!(board.get_piece(&Square::from(0, 2).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(1, 2).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(2, 2).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(3, 2).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(4, 2).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(5, 2).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(6, 2).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(7, 2).unwrap()), None);

        assert_eq!(board.get_piece(&Square::from(0, 3).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(1, 3).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(2, 3).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(3, 3).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(4, 3).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(5, 3).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(6, 3).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(7, 3).unwrap()), None);

        assert_eq!(board.get_piece(&Square::from(0, 4).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(1, 4).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(2, 4).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(3, 4).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(4, 4).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(5, 4).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(6, 4).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(7, 4).unwrap()), None);

        assert_eq!(board.get_piece(&Square::from(0, 5).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(1, 5).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(2, 5).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(3, 5).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(4, 5).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(5, 5).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(6, 5).unwrap()), None);
        assert_eq!(board.get_piece(&Square::from(7, 5).unwrap()), None);

        assert_eq!(
            board.get_piece(&Square::from(0, 6).unwrap()),
            Some(Piece(PieceType::Pawn, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(1, 6).unwrap()),
            Some(Piece(PieceType::Pawn, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(2, 6).unwrap()),
            Some(Piece(PieceType::Pawn, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(3, 6).unwrap()),
            Some(Piece(PieceType::Pawn, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(4, 6).unwrap()),
            Some(Piece(PieceType::Pawn, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(5, 6).unwrap()),
            Some(Piece(PieceType::Pawn, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(6, 6).unwrap()),
            Some(Piece(PieceType::Pawn, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(7, 6).unwrap()),
            Some(Piece(PieceType::Pawn, Color::Black))
        );

        assert_eq!(
            board.get_piece(&Square::from(0, 7).unwrap()),
            Some(Piece(PieceType::Rook, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(1, 7).unwrap()),
            Some(Piece(PieceType::Knight, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(2, 7).unwrap()),
            Some(Piece(PieceType::Bishop, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(3, 7).unwrap()),
            Some(Piece(PieceType::Queen, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(4, 7).unwrap()),
            Some(Piece(PieceType::King, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(5, 7).unwrap()),
            Some(Piece(PieceType::Bishop, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(6, 7).unwrap()),
            Some(Piece(PieceType::Knight, Color::Black))
        );
        assert_eq!(
            board.get_piece(&Square::from(7, 7).unwrap()),
            Some(Piece(PieceType::Rook, Color::Black))
        );
    }
}
