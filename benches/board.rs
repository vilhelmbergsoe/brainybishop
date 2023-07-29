use criterion::{black_box, criterion_group, criterion_main, Criterion};

use brainybishop::board::BoardState;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from_fen", |b| {
        b.iter(|| {
            BoardState::from_fen(black_box(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
