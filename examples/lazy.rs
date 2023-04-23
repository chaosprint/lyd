use anyhow::Result;
use lyd::node::*;
use lyd::*;

fn main() -> Result<()> {
    let mut ctx = context().frames(4).channels(2).sr(44100);
    ctx.add_sig("output", vec![sin_osc().freq("~fm"), mul(0.1)]);
    ctx.add_sig("~fm", vec![sin_osc().freq(200.), mul(300.), add(600.)]);

    for _ in 0..8 {
        ctx.next_block();
        println!(
            "output buffer {:?}",
            ctx.buffers.get("output").unwrap() //.lock()
        );
    }
    Ok(())
}
