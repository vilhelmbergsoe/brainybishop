use criterion::{black_box, criterion_group, criterion_main, Criterion};

use brainybishop::board::Board;
use brainybishop::movegen::generate_moves;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from_fen_default_position", |b| {
        b.iter(|| {
            Board::from_fen(black_box(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ))
        })
    });

    c.bench_function("generate_moves_starting", |b| {
        let board = Board::default();
        b.iter(|| generate_moves(black_box(&board)))
    });

    c.bench_function("generate_moves_kiwipete", |b| {
        let board = Board::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();
        b.iter(|| generate_moves(black_box(&board)))
    });

    c.bench_function("generate_moves_midgame", |b| {
        let board =
            Board::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1")
                .unwrap();
        b.iter(|| generate_moves(black_box(&board)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
