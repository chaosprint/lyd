use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lyd::*;

fn fm(c: &mut Criterion) {
    let mut ctx = context().channels(2).frames(128).sr(44100).build(&[
        (
            "~mod",
            &[sin_osc().freq(10.0).amp(black_box(300.)), add(500.1)],
        ),
        ("out", &[sin_osc().freq("~mod"), add(0.1)]),
    ]);

    c.bench_function("fm", |b| {
        b.iter(|| {
            ctx.next_block();
        })
    });
}

criterion_group!(benches, fm);
criterion_main!(benches);
