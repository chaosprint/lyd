use crate::params::*;
use crate::Buffer;
use smallvec::SmallVec;
use std::f32::consts::PI;

pub enum NodeConfig {
    SinOsc(SinOscConfig),
    Add(AddConfig),
}

pub struct SinOscConfig {
    pub freq: Param,
    pub phase: Param,
    pub amp: Param,
    pub sr: Param,
}

impl Default for SinOscConfig {
    fn default() -> Self {
        Self {
            freq: Param::Float(440.0),
            phase: Param::Float(0.0),
            amp: Param::Float(0.5),
            sr: Param::Int(44100),
        }
    }
}

pub struct AddConfig {
    pub add: Param,
}

pub enum Nodes {
    SinOsc(SinOscStruct),
    Add(AddStruct),
}

pub struct SinOscStruct {
    pub freq: Param,
    pub phase: f32,
    pub amp: f32,
    pub sr: u32,
}

pub struct AddStruct {
    pub add: Param,
}

impl SinOscStruct {
    #[inline]
    pub fn process(
        &mut self,
        buf: &mut Buffer,
        sidechain_buf: Option<*const SmallVec<[Buffer; 64]>>,
    ) {
        let channels = buf.len();
        let frames = buf[0].len();

        for f in 0..frames {
            let val = (2.0 * PI * self.phase).sin() * self.amp;
            for c in 0..channels {
                buf[c][f] = val;
            }
            let freq = match self.freq {
                Param::Float(freq) => freq,
                Param::Int(index) => unsafe { (&*sidechain_buf.unwrap())[index as usize][0][f] },
            };
            self.phase += freq / self.sr as f32;
        }
    }
}

impl AddStruct {
    pub fn process(&mut self, buf: &mut Buffer, sidechain_buf: Option<&SmallVec<[Buffer; 64]>>) {
        let channels = buf.len();
        let frames = buf[0].len();
        for c in 0..channels {
            for f in 0..frames {
                buf[c][f] += match self.add {
                    Param::Float(add) => add,
                    Param::Int(index) => sidechain_buf.unwrap()[index as usize][0][f],
                    _ => 0.0,
                };
            }
        }
    }
}
