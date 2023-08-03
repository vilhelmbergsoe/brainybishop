use criterion::{black_box, criterion_group, criterion_main, Criterion};

use brainybishop::board::Board;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from_fen_default_position", |b| {
        b.iter(|| {
            Board::from_fen(black_box(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ))
        })
    });
    c.bench_function("get_pawn_moves", |b| {
        b.iter(|| {
            let board = Board::default();

            let square = brainybishop::board::Square::from_algebraic("a2").unwrap();

            brainybishop::movegen::MoveGen::get_moves(&board, &square)
        })
    });
    c.bench_function("get_king_moves", |b| {
        b.iter(|| {
            let board = Board::default();

            let square = brainybishop::board::Square::from_algebraic("e1").unwrap();

            brainybishop::movegen::MoveGen::get_moves(&board, &square)
        })
    });
    c.bench_function("get_knight_moves", |b| {
        b.iter(|| {
            let board = Board::default();

            let square = brainybishop::board::Square::from_algebraic("b1").unwrap();

            brainybishop::movegen::MoveGen::get_moves(&board, &square)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
