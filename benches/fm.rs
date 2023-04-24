use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lyd::*;

fn fm(c: &mut Criterion) {

    let mut ctx = context().frames(128).channels(2).sr(44100).build(
        &[
            &[sin_osc!(1)],
            &[sin_osc!(10.0, 0.0, 200.0), add!(400.5)],
        ]
    );

    c.bench_function("fm", |b| b.iter(|| {
        ctx.next_block();
    }));
}

criterion_group!(benches, fm);
criterion_main!(benches);