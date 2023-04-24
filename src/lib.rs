pub mod enums;
pub use crate::enums::*;

use smallvec::{smallvec, SmallVec};

pub type Buffer = SmallVec<[SmallVec<[f32; 128]>; 2]>;
pub type Signal = SmallVec<[Nodes; 16]>; // Nodes is a enum

#[derive(Debug)]
pub struct ProcessOrder {
    pub row: usize,
    pub column: usize,
    pub sidechain_buf: SmallVec<[usize; 64]>, 
}

pub fn context() -> Context {
    Context::new()
}

pub struct Context {
    pub sr: usize,
    pub frames: usize,
    pub channels: usize,
    pub signals: SmallVec<[Signal; 64]>,
    pub buffers: SmallVec<[Buffer; 64]>,
    pub process_order: SmallVec<[ProcessOrder; 1024]>, // 64*16
}

impl Context {
    pub fn new() -> Self {
        Self {
            sr: 44100,
            frames: 128,
            channels: 2,
            signals: smallvec![],
            buffers: smallvec![],
            process_order: smallvec![],
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

    pub fn sr(mut self, sr: usize) -> Self {
        self.sr = sr;
        self
    }

    pub fn build(mut self, signals: &[&[NodeConfig]]) -> Self {
        for (row, signal) in signals.iter().enumerate() {
            self.signals.push(smallvec![]);
            for (column, node) in signal.iter().enumerate() {
                match node {
                    NodeConfig::SinOsc(config) => {
                        self.signals[row].push(Nodes::SinOsc(SinOscStruct {
                            freq: config.freq,
                            phase: config.phase,
                            amp: config.amp,
                            sr: config.sr,
                        }));
                    }
                    NodeConfig::Add(config) => {
                        self.signals[row].push(Nodes::Add(AddStruct {
                            add: config.add,
                        }));
                    }
                }
                self.process_order.push(ProcessOrder {
                    row,
                    column,
                    sidechain_buf: smallvec![],
                });
            }
            self.buffers.push(smallvec![smallvec![0.0; self.frames]; self.channels]);
        }
        self
    }

    pub fn next_block(&mut self) -> &Buffer {
        // println!("self.process_order {:?}", &self.process_order);
        for order in &self.process_order {
            let buf = &mut self.buffers[order.row];
            let signal = &mut self.signals[order.row];
            for node in signal {
                match node {
                    Nodes::SinOsc(node) => {
                        node.process(buf, None)
                    }
                    Nodes::Add(node) => {
                        node.process(buf, None)
                    },
                }
            }
        }
        &self.buffers[0]
    }
}
