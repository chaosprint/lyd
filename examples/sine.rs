use lyd::*;

fn main() {
    let mut ctx = context().frames(4).channels(2).sr(44100).build(
        &[
            &[NodeConfig::SinOsc(
                SinOscConfig {
                    freq: 440.0,
                    phase: 0.0,
                    amp: 0.5,
                    sr: 44100,
                }
            )],
        ]
    );
    println!("{:?}", ctx.next_buffer());
}