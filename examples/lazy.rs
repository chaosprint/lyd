use anyhow::Result;
use lyd::node::*;
use lyd::*;

fn main() -> Result<()> {
    let mut ctx = context().frames(4).channels(2).sr(44100);
    ctx.add_sig("output", vec![SinOsc::new().freq(440.)]);
    ctx.next_block();
    println!(
        "output buffer {:?}",
        ctx.buffers.get("output").unwrap() //.lock()
    );
    Ok(())
}
