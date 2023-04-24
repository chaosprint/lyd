use lyd::*;

fn main() {
    let mut ctx = context().frames(4).channels(2).sr(44100).build(
        &[
            &[sin_osc!(440.0)],
        ]
    );
    println!("{:?}", ctx.next_block());
}