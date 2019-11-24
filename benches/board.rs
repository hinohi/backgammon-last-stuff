use criterion::{criterion_group, criterion_main, Criterion};

use backgammon_last_stuff::Board;

fn list_moves_1(c: &mut Criterion) {
    let board = Board::new([2, 2, 2, 2, 2, 2]);
    c.bench_function("list_moves_1", |b| {
        b.iter(|| board.list_moves(&[3, 4]));
    });
}

fn list_moves_2(c: &mut Criterion) {
    let board = Board::new([1, 2, 0, 0, 0, 0]);
    c.bench_function("list_moves_2", |b| {
        b.iter(|| board.list_moves(&[5, 5, 5, 5]));
    });
}

criterion_group!(benches, list_moves_1, list_moves_2);
criterion_main!(benches);
