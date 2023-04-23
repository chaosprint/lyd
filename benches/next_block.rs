use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lyd::context;

fn next_block(c: &mut Criterion) {
    let mut context = context().frames(128).channels(2).sr(44100);
    context.add_sig("output", vec![lyd::node::SinOsc::new().freq(black_box(440.))]);
    c.bench_function("next_block", |b| b.iter(|| {
        context.next_block();
    }));
}

criterion_group!(benches, next_block);
criterion_main!(benches);