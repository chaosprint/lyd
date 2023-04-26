use lyd::*;

fn main() {
    let mut ctx = context()
        .channels(2)
        .frames(1024)
        .sr(48000)
        .build(&[&[NodeConfig::SinOsc(SinOscConfig {
            freq: Param::Float(999.),
            phase: Param::Float(0.0),
            amp: Param::Float(0.1),
            sr: Param::Int(48000),
        })]]);
    println!("{:?}", ctx.next_block());
}
