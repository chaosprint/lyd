use anyhow::Result;
use lyd::*;

fn main() -> Result<()> {
    let mut ctx = context().frames(4).channels(2).sr(44100);

    let track1 = &[seq("60").speed(2.0), sampler("/bd")];
    let track2 = &[seq("48").speed(8.0), sampler("/bass")];
    let mixer = &[mixer()];
    ctx.connect(track1, mixer, First::Mixer::Inputs);
    
    ctx.set_output(outsig);
    println!("{:?}", ctx.next_buffer());
    Ok(())
}
