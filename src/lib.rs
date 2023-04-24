pub mod node;
use crate::node::*;
use hashbrown::HashMap;
use smallvec::{smallvec, SmallVec};

pub type Buffers = HashMap<String, Buffer>;
pub type Buffer = SmallVec<[SmallVec<[f32; 1024]>; 2]>;
pub type Sig = Vec<Box<dyn Node + Send>>;
pub type Sigs = HashMap<String, Sig>;

pub fn context() -> Context {
    Context::new()
}

pub struct Context {
    pub sr: usize,
    pub frames: usize,
    pub channels: usize,
    pub sig_chains: Sigs,
    pub buffers: Buffers,
    pub process_order: Vec<String>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            sr: 44100,
            frames: 128,
            channels: 2,
            sig_chains: HashMap::new(),
            buffers: HashMap::new(),
            process_order: Vec::new(),
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

    pub fn add_sig(&mut self, name: &str, sig: Vec<Box<dyn Node + Send>>) {
        for node in sig.iter() {
            if let Some(refs) = node.get_ref() {
                for r in refs {
                    if self.process_order.contains(&r.to_string()) {
                        continue;
                    } else {
                        self.process_order.insert(0, r.to_string());
                    }
                }
            }
        }
        self.sig_chains.insert(name.to_string(), sig);

        self.buffers.insert(
            name.to_string(),
            smallvec![smallvec![0.0_f32; self.frames]; self.channels],
        );
        if !self.process_order.contains(&name.to_string()) {
            self.process_order.push(name.to_string());
        }
    }

    pub fn next_block(&mut self) {
        let ctx = self as *mut Context;
        for name in unsafe { (*ctx).process_order.iter_mut() } {
            let lock = &mut self.sig_chains; //.lock();
            let sig = lock.get_mut(name).unwrap();
            for node in sig {
                node.process(unsafe { &mut *ctx }, &name);
            }
        }
    }
}
