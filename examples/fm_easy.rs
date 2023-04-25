use lyd::*;

fn main() {
    let mut ctx = context().channels(2).frames(1024).sr(48000).build(&[
        &[sin_osc().freq(1), add(0.1)],
        &[sin_osc().freq(10.0).amp(300.), add(500.1)],
    ]);
    println!("{:?}", ctx.next_block());
}
