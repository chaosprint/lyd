use hound;

use crate::Context;
use std::path::Path;

/// Renders multi-channel audio samples to a WAV file
pub fn render_vec_to_wav(
    samples: Vec<Vec<f32>>,
    sample_rate: usize,
    filename: &str,
) -> Result<(), hound::Error> {
    let channels = samples.len();
    let spec = hound::WavSpec {
        channels: channels as u16,
        sample_rate: sample_rate as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(Path::new(filename), spec)?;

    // Interleave channels during writing
    let frames = samples[0].len();
    for frame in 0..frames {
        for chan in 0..channels {
            writer.write_sample(samples[chan][frame])?;
        }
    }

    writer.finalize()?;
    Ok(())
}

pub fn render_to_wav(
    ctx: &mut Context,
    duration_secs: f32,
    filename: &str,
) -> Result<(), hound::Error> {
    let channels = ctx.channels;
    let sample_rate = ctx.sr;
    let frames = ctx.frames;

    let total_frames = (duration_secs * ctx.sr as f32) as usize;
    let frames_per_block = ctx.frames;
    let num_blocks = total_frames / frames_per_block;

    // Prepare processing parameters
    let gain = 0.7; // -3dB for reasonable headroom
    let mut dc_offset = 0.0f32;
    let smoothing = 0.001; // DC offset smoothing factor

    // Pre-allocate vectors for each channel
    let mut channel_samples: Vec<Vec<f32>> = (0..channels)
        .map(|_| Vec::with_capacity(total_frames))
        .collect();

    // Process audio in blocks
    for _ in 0..num_blocks {
        let block = ctx.next_block();

        // Improved DC offset calculation with smoothing
        let block_dc = block.iter().flat_map(|chan| chan.iter()).sum::<f32>() 
            / (block.len() * frames) as f32;
        dc_offset = dc_offset * (1.0 - smoothing) + block_dc * smoothing;

        // Process samples for each channel
        for chan in 0..channels {
            for frame in 0..frames {
                let mut sample = block[chan][frame] - dc_offset;
                sample *= gain;
                sample = sample.tanh();
                channel_samples[chan].push(sample);
            }
        }
    }

    render_vec_to_wav(channel_samples, sample_rate as usize, filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add, context, sin_osc};

    #[test]
    fn test_wav_render() {
        let mut ctx = context().channels(2).frames(128).sr(44100).build(&[
            ("~mod", &[sin_osc().freq(10.0).amp(300.), add(500.1)]),
            ("out", &[sin_osc().freq("~mod"), add(0.1)]),
        ]);

        let result = render_to_wav(&mut ctx, 1.0, "test.wav");
        assert!(result.is_ok());
    }
}
