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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
