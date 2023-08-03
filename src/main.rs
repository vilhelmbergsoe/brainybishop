use color_eyre::eyre::Result;

use brainybishop::board::{Board, Color, Piece, PieceType, Square};
use brainybishop::movegen::MoveGen;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Default starting position
    let mut _board = Board::default();

    let mut _board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    // let mut board =
    //     Board::from_fen("rnbqkbnr/p1p1p1p1/k7/1p1p1p1p/P1P1P1P1/8/1P1P1P1P/RNBQKBNR w KQkq - 0 1")
    //         .unwrap();

    let mut board = Board::from_fen("8/8/8/8/8/8/8/8 w KQkq - 0 1").unwrap();

    // board.remove_piece(&Square::from_algebraic("d2").unwrap());
    // board.set_piece(
    //     Square::from_algebraic("d4").unwrap(),
    //     Piece(PieceType::Pawn, Color::White),
    // );

    board.set_piece(
        Square::from_algebraic("a6").unwrap(),
        Piece(PieceType::King, Color::White),
    );
    board.set_piece(
        Square::from_algebraic("a7").unwrap(),
        Piece(PieceType::Pawn, Color::White),
    );
    board.set_piece(
        Square::from_algebraic("d4").unwrap(),
        Piece(PieceType::Knight, Color::White),
    );

    board.set_piece(
        Square::from_algebraic("b3").unwrap(),
        Piece(PieceType::Knight, Color::White),
    );

    print!("{}", board.bitboard);
    println!("{:#064b}", board.bitboard.0[5]);

    // let moves = MoveGen::get_moves(&board, &Square::from_algebraic("c4").unwrap());
    // println!("c4 moves: {:?}", moves.unwrap().to_algebraic());

    // let moves = MoveGen::get_moves(&board, &Square::from_algebraic("b5").unwrap());
    // println!("b5 moves: {:?}", moves.unwrap().to_algebraic());

    let moves = MoveGen::get_moves(&board, &Square::from_algebraic("a6").unwrap());
    println!("a6 moves: {:?}", moves.unwrap().to_algebraic());

    let moves = MoveGen::get_moves(&board, &Square::from_algebraic("d4").unwrap());
    println!("d4 moves: {:?}", moves.unwrap().to_algebraic());

    let moves = MoveGen::get_moves(&board, &Square::from_algebraic("b3").unwrap());
    println!("b3 moves: {:?}", moves.unwrap().to_algebraic());

    Ok(())
}
