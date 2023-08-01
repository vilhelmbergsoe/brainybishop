use color_eyre::eyre::Result;

use brainybishop::board::{Board, Color, Piece, PieceType, Square};

fn main() -> Result<()> {
    color_eyre::install()?;

    // Default starting position
    let mut _boardstate = Board::default();

    let mut boardstate =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    boardstate
        .board
        .remove_piece(&Square::from_algebraic("d2").unwrap());
    boardstate.board.set_piece(
        Square::from_algebraic("d4").unwrap(),
        Piece(PieceType::Pawn, Color::White),
    );

    print!("{}", boardstate.board);

    Ok(())
}
