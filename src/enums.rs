use crate::params::*;
use crate::Buffer;
use smallvec::SmallVec;
use hashbrown::HashMap;

#[cfg(feature = "no_std")]
use core::f32::consts::PI;

#[cfg(feature = "no_std")]
extern crate libm;

#[cfg(not(feature = "no_std"))]
use std::f32::consts::PI;

pub fn sin_osc() -> NodeConfig {
    NodeConfig::SinOsc(SinOscConfig::default())
}

impl NodeConfig {
    pub fn freq(mut self, freq: impl AsParam) -> Self {
        match &mut self {
            NodeConfig::SinOsc(config) => config.freq = freq.as_param(),
            _ => unreachable!("freq() is only available for SinOsc"),
        }
        self
    }
    pub fn phase(mut self, phase: f32) -> Self {
        match &mut self {
            NodeConfig::SinOsc(config) => config.phase = Param::Float(phase),
            _ => unreachable!("phase() is only available for SinOsc"),
        }
        self
    }
    pub fn amp(mut self, amp: f32) -> Self {
        match &mut self {
            NodeConfig::SinOsc(config) => config.amp = Param::Float(amp),
            _ => unreachable!("amp() is only available for SinOsc"),
        }
        self
    }
    pub fn sr(mut self, sr: u32) -> Self {
        match &mut self {
            NodeConfig::SinOsc(config) => config.sr = Param::Int(sr),
            _ => unreachable!("sr() is only available for SinOsc"),
        }
        self
    }
}

pub fn add(val: f32) -> NodeConfig {
    NodeConfig::Add(AddConfig {
        add: Param::Float(val),
    })
}

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
        sidechain_buf: Option<*const HashMap<&'static str, Buffer>>,
        // sidechain_buf: Option<*const HashMap<[Buffer; 4]>>,
    ) {
        let channels = buf.len();
        let frames = buf[0].len();

        for f in 0..frames {
            let val = {
                #[cfg(feature = "no_std")]
                {
                    libm::sinf(2.0 * PI * self.phase) * self.amp
                }
                #[cfg(not(feature = "no_std"))]
                {
                    (2.0 * PI * self.phase as f32).sin() * self.amp
                }
            };
            for c in 0..channels {
                buf[c][f] = val;
            }
            let freq = match self.freq {
                Param::Float(freq) => freq,
                // Param::Int(index) => unsafe { (&*sidechain_buf.unwrap())[index as usize][0][f] },
                Param::Str(s) => unsafe { (&*sidechain_buf.unwrap()).get(s).unwrap()[0][f] },
                _ => 0.0,
            };
            self.phase += freq / self.sr as f32;
        }
    }
}

impl AddStruct {
    pub fn process(&mut self, buf: &mut Buffer, sidechain_buf: Option<&SmallVec<[Buffer; 4]>>) {
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
