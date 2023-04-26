#![cfg_attr(feature = "no_std", no_std)]

pub mod enums;
pub use crate::enums::*;

// pub mod macros;
// pub use crate::macros::*;

pub mod params;
pub use crate::params::*;

use hashbrown::HashMap;
use smallvec::{smallvec, SmallVec};

pub type Buffer = SmallVec<[SmallVec<[f32; 1024]>; 2]>;
pub type Signal = SmallVec<[Nodes; 8]>; // Nodes is a enum

// #[derive(Debug)]
// pub struct ProcessOrder {
//     pub row: usize,
//     pub column: usize,
//     pub sidechain_buf: SmallVec<[usize; 64]>,
// }

pub fn context() -> Context {
    Context::new()
}

pub struct Context {
    pub sr: u32,
    pub frames: usize,
    pub channels: usize,
    pub signals: HashMap<&'static str, Signal>,
    pub buffers: HashMap<&'static str, Buffer>,
    // pub signals: SmallVec<[Signal; 4]>,
    // pub buffers: SmallVec<[Buffer; 4]>,
    // pub process_order: SmallVec<[ProcessOrder; 1024]>, // 64*16
}

impl Context {
    pub fn new() -> Self {
        Self {
            sr: 44100,
            frames: 128,
            channels: 2,
            signals: HashMap::new(),
            buffers: HashMap::new(),
            // signals: smallvec![],
            // buffers: smallvec![],
            // process_order: smallvec![],
        }
    }

    pub fn frames(mut self, frames: usize) -> Self {
        self.frames = frames;
        self
    }

    pub fn channels(mut self, channels: usize) -> Self {
        self.channels = channels;
        self
    }

    pub fn sr(mut self, sr: u32) -> Self {
        self.sr = sr;
        self
    }

    pub fn build(mut self, signals: &[(&'static str, &[NodeConfig])]) -> Self {
        for (_row, chain) in signals.iter().enumerate() {
            let (refname, signal) = chain;
            // self.signals.push(smallvec![]);
            let mut sig = smallvec![];
            for (_, node) in signal.iter().enumerate() {
                match node {
                    NodeConfig::SinOsc(config) => {
                        sig.push(Nodes::SinOsc(SinOscStruct {
                            freq: config.freq,
                            phase: config.phase.as_float(),
                            amp: config.amp.as_float(),
                            sr: self.sr,
                        }));
                    }
                    NodeConfig::Add(config) => {
                        sig.push(Nodes::Add(AddStruct { add: config.add }));
                    }
                }
            }
            self.signals.insert(refname, sig);
            self.buffers.insert(
                refname,
                smallvec![smallvec![0.0; self.frames]; self.channels],
            );
        }
        self
    }

    pub fn next_block(&mut self) -> &mut Buffer {
        //-> &mut Buffer
        // println!("self.process_order {:?}", &self.process_order);
        let ctx = self as *const Self;
        for (refname, signal) in self.signals.iter_mut() {
            // let signal = &mut self.signal.get_mut(key).unwrap();
            let buf = &mut self.buffers.get_mut(refname).unwrap();

            for node in signal.iter_mut() {
                match node {
                    Nodes::SinOsc(node) => node.process(
                        buf,
                        Some(unsafe { &(&*ctx).buffers } as *const HashMap<&'static str, Buffer>),
                    ),
                    Nodes::Add(node) => node.process(buf, None),
                }
            }
        }
        // for order in &self.process_order {
        //     let buf = &mut self.buffers[order.row];
        //     let signal = &mut self.signals[order.row];
        //     for node in signal {
        //         match node {
        //             Nodes::SinOsc(node) => node.process(
        //                 buf,
        //                 Some(unsafe { &(&*ctx).buffers } as *const SmallVec<[Buffer; 4]>),
        //             ),
        //             Nodes::Add(node) => node.process(buf, None),
        //         }
        //     }
        // }
        self.buffers.get_mut("out").unwrap()
    }
}
