use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lyd::*;

fn sine(c: &mut Criterion) {
    let mut ctx = context().frames(4).channels(2).sr(44100).build(
        &[
            &[NodeConfig::SinOsc(
                SinOscConfig {
                    freq: black_box(440.0),
                    phase: 0.0,
                    amp: 0.5,
                    sr: 44100,
                }
            )],
        ]
    );
    c.bench_function("next_block", |b| b.iter(|| {
        ctx.next_block();
    }));
}

criterion_group!(benches, sine);
criterion_main!(benches);