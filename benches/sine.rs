use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lyd::*;

fn sine(c: &mut Criterion) {
    let mut ctx = context()
        .frames(128)
        .channels(2)
        .sr(44100)
        .build(&[("out", &[sin_osc().freq(black_box(440.))])]);
    c.bench_function("sine", |b| {
        b.iter(|| {
            ctx.next_block();
        })
    });
}

criterion_group!(benches, sine);
criterion_main!(benches);
