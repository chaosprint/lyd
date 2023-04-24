use lyd::*;

fn main() {
    let mut ctx = context().frames(4).channels(2).sr(44100).build(
        &[
            &[sin_osc!(1)],
            &[sin_osc!(200.0, 0.0, 500.0), add!(1000.5)],
        ]
    );
    println!("{:?}", ctx.next_block());
}