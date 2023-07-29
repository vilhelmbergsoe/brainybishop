use color_eyre::eyre::Result;

use brainybishop::board::{Board, BoardState, Color, Piece, PieceType, Square};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut _boardstate = BoardState::default();

    let mut boardstate =
    BoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    boardstate.board.remove_piece(&Square::from_algebraic("d2").unwrap());
    boardstate.board.set_piece(Square::from_algebraic("d4").unwrap(), Piece(PieceType::Pawn, Color::White));

    print!("{}", boardstate.board);

    Ok(())
}
