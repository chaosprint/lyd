use lyd::*;

fn main() {
    let mut ctx = context().frames(4).channels(2).sr(44100).build(
        &[
            &[sin_osc().amp(0.1)],
            &[sin_osc(1).amp(300.), add(600.)],
        ]
    )
    println!("{:?}", ctx.next_buffer());
}