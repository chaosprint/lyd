use crate::Buffer;
use smallvec::SmallVec;
use std::f32::consts::PI;

pub enum NodeConfig {
    SinOsc(SinOscConfig),
    Add(AddConfig),
}

pub struct SinOscConfig {
    pub freq: f32,
    pub phase: f32,
    pub amp: f32,
    pub sr: u32,
}

pub struct AddConfig {
    pub add: f32,
}

pub enum Nodes {
    SinOsc(SinOscStruct),
    Add(AddStruct),
}

pub struct SinOscStruct {
    pub freq: f32,
    pub phase: f32,
    pub amp: f32,
    pub sr: u32,
}

pub struct AddStruct {
    pub add: f32,
}

impl Default for SinOscConfig {
    fn default() -> Self {
        Self {
            freq: 440.0,
            phase: 0.0,
            amp: 1.0,
            sr: 44100,
        }
    }
}

impl SinOscStruct {
    pub fn process(&mut self, buf: &mut Buffer, _sidechain_buf: Option<SmallVec<[Buffer; 64]>>) {
        let channels = buf.len();
        let frames = buf[0].len();
        // let buf_ptr = buf.as_mut_ptr();
        for f in 0..frames {
            for c in 0..channels {
                buf[c][f] = (2.0 * PI * self.phase).sin() * self.amp;
            }
            self.phase += self.freq / self.sr as f32;
        }
        // for c in 1..channels {
        //     buf[c].copy_from_slice(unsafe {&*buf_ptr});
        // }
    }
}

impl AddStruct {
    pub fn process(&mut self, buf: &mut Buffer, _sidechain_buf: Option<SmallVec<[Buffer; 64]>>) {
        let channels = buf.len();
        let frames = buf[0].len();
        for c in 0..channels {
            for f in 0..frames {
                buf[c][f] += self.add;
            }
        }
    }
}