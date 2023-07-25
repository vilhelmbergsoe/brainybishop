use board::Board;
use board::Square;
mod board;

fn main() {
    let mut board = Board::default();


    let board =
        Board::from_fen("rnbqkbnr/p1p1p1p1/8/1p1p1p1p/P1P1P1P1/8/1P1P1P1P/RNBQKBNR w KQkq - 0 1")
            .unwrap();

    let before = std::time::Instant::now();
    for i in 0..1000000000 {
        board.get_piece_at(&Square::from_algebraic("e4").unwrap());
    }
    println!("100 fen parses took: {:?}", before.elapsed());

    print!("{}", board);

    println!(
        "legal moves for e3: {:?}",
        board
            .get_legal_moves(Square::from_algebraic("e4").unwrap())
            .unwrap()
            .iter()
            .map(|x| x.to_algebraic())
            .collect::<Vec<String>>()
    );

    println!(
        "legal moves for a4: {:?}",
        board
            .get_legal_moves(Square::from_algebraic("a4").unwrap())
            .unwrap()
            .iter()
            .map(|x| x.to_algebraic())
            .collect::<Vec<String>>()
    );

    println!(
        "legal moves for b5: {:?}",
        board
            .get_legal_moves(Square::from_algebraic("b5").unwrap())
            .unwrap()
            .iter()
            .map(|x| x.to_algebraic())
            .collect::<Vec<String>>()
    );

    println!(
        "legal moves for h5: {:?}",
        board
            .get_legal_moves(Square::from_algebraic("h5").unwrap())
            .unwrap()
            .iter()
            .map(|x| x.to_algebraic())
            .collect::<Vec<String>>()
    );

    // println!(
    //     "legal moves for e1: {:?}",
    //     board
    //         .get_legal_moves(Square::from_algebraic("e1").unwrap())
    //         .unwrap()
    //         .iter()
    //         .map(|x| x.to_algebraic())
    //         .collect::<Vec<String>>()
    // );
}
