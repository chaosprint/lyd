use anyhow::Result;
use lyd::node::*;
use lyd::*;

fn main() -> Result<()> {
    let mut ctx = context().frames(4).channels(2).sr(44100);

    ctx.add_sig("output", vec![sin_osc().freq("~fm"), mul(0.1)]);
    ctx.add_sig("~fm", vec![sin_osc().freq("~mod"), mul(300.), add(600.)]);
    ctx.add_sig("~mod", vec![sin_osc().freq("~a"), mul(300.), add(600.)]);
    ctx.add_sig("~a", vec![sin_osc().freq(440.), mul(0.1)]);

    println!("process_order {:?}", &ctx.process_order);
    for _ in 0..4 {
        ctx.next_block();
        println!("output buffers: {:?}", ctx.buffers.get("output").unwrap());
    }
    Ok(())
}
