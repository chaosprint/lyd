use anyhow;
use lyd::{add, context, render_to_wav, sin_osc};

fn main() -> anyhow::Result<()> {
    let mut ctx = context().channels(2).frames(128).sr(44100).build(&[
        ("~mod", &[sin_osc().freq(10.0).amp(20.0), add(500.1)]),
        ("out", &[sin_osc().freq("~mod")]),
    ]);

    render_to_wav(&mut ctx, 3.0, "fm_demo.wav")?;
    println!("Successfully rendered FM synthesis to fm_demo.wav");
    Ok(())
}
