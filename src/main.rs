use color_eyre::eyre::Result;

use brainybishop::board::Board;
use brainybishop::movegen::generate_moves;

fn main() -> Result<()> {
    color_eyre::install()?;

    let board = Board::default();
    println!("{}", board.bitboard);

    let moves = generate_moves(&board);
    println!("Legal moves from starting position: {}", moves.len());

    for mv in moves.iter() {
        print!("{} ", mv);
    }
    println!();

    // Test a position with en passant
    let board =
        Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1").unwrap();
    println!("\n{}", board.bitboard);

    let moves = generate_moves(&board);
    println!("Legal moves: {}", moves.len());

    for mv in moves.iter() {
        print!("{} ", mv);
    }
    println!();

    Ok(())
}
