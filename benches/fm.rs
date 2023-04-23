use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lyd::{context, node::*};

fn reference(c: &mut Criterion) {
    let mut context = context().frames(128).channels(2).sr(44100);
    context.add_sig("output", vec![sin_osc().freq("~fm"), mul(0.1)]);
    context.add_sig("~fm", vec![sin_osc().freq(black_box(200.)), mul(300.), add(600.)]);
    c.bench_function("next_block", |b| b.iter(|| {
        context.next_block();
    }));
}

criterion_group!(benches, reference);
criterion_main!(benches);