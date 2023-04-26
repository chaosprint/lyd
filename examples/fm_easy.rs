use lyd::*;

fn main() {
    let mut ctx = context().channels(2).frames(1024).sr(48000).build(&[
        ("~mod", &[sin_osc().freq(10.0).amp(300.), add(500.1)]),
        ("out", &[sin_osc().freq("~mod"), add(0.1)]),
    ]);
    println!("{:?}", ctx.next_block());
}
