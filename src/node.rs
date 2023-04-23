use crate::{Buffer, Context};
use smallvec::{smallvec, SmallVec};

pub type RefList = SmallVec<[String; 5]>; // not likely to have more than 5 refs
pub enum ParamResult {
    Float(f32),
    Buffer(Buffer),
}

pub trait Node: Send {
    fn process(&mut self, context: &mut Context, name: &str);
    fn get_ref(&self) -> Option<RefList>;
}

pub trait Signal: Send {
    fn give_buf(&self, context: &Context) -> ParamResult;
    fn get_ref(&self) -> Option<&str>;
}

impl Signal for f32 {
    fn give_buf(&self, _context: &Context) -> ParamResult {
        ParamResult::Float(*self)
    }
    fn get_ref(&self) -> Option<&str> {
        None
    }
}

impl Signal for &str {
    fn give_buf(&self, context: &Context) -> ParamResult {
        let cell = context.buffers.get(*self).unwrap().lock();
        ParamResult::Buffer(cell.clone())
    }
    fn get_ref(&self) -> Option<&str> {
        Some(self)
    }
}

pub struct SinOsc {
    pub freq: Box<dyn Signal>,
    pub phase: f32,
    pub amp: f32,
}

impl SinOsc {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            freq: Box::new(440.),
            phase: 0.,
            amp: 1.,
        })
    }

    pub fn freq(mut self, freq: impl Signal + 'static) -> Box<Self> {
        self.freq = Box::new(freq);
        Box::new(self)
    }

    pub fn phase(mut self, phase: f32) -> Box<Self> {
        self.phase = phase;
        Box::new(self)
    }

    pub fn amp(mut self, amp: f32) -> Box<Self> {
        self.amp = amp;
        Box::new(self)
    }
}

impl Node for SinOsc {
    fn process(&mut self, context: &mut Context, name: &str) {
        // println!("called");
        let ctx = &mut *context as *mut Context;
        let mut lock;
        let buf;
        unsafe {
            lock = (*ctx).buffers.get_mut(name).unwrap().lock();
            buf = &mut *lock;
        }
        let freq = match self.freq.give_buf(context) {
            ParamResult::Float(f) => smallvec![smallvec![f; context.frames]],
            ParamResult::Buffer(b) => b,
        };
        for j in 0..context.frames {
            buf[0][j] = (self.phase * 2.0 * std::f32::consts::PI).sin() * self.amp;
            self.phase += freq[0][j] / context.sr as f32;
        }
        for i in 1..context.channels {
            buf[i] = buf[0].clone();
        }
    }
    fn get_ref(&self) -> Option<RefList> {
        let mut refs = RefList::new();
        if let Some(key) = self.freq.get_ref() {
            refs.push(key.to_string());
        }
        if refs.is_empty() {
            None
        } else {
            Some(refs)
        }
    }
}

pub struct Mul {
    pub val: Box<dyn Signal>,
}

impl Mul {
    pub fn new(val: impl Signal + 'static) -> Box<Self> {
        Box::new(Self { val: Box::new(val) })
    }
}

impl Node for Mul {
    fn process(&mut self, context: &mut Context, name: &str) {
        // println!("called");
        let ctx = &mut *context as *mut Context;
        let mut lock;
        let buf;
        unsafe {
            lock = (*ctx).buffers.get_mut(name).unwrap().lock();
            buf = &mut *lock;
        }
        let val = match self.val.give_buf(context) {
            ParamResult::Float(f) => smallvec![smallvec![f; context.frames]],
            ParamResult::Buffer(b) => b,
        };
        for j in 0..context.frames {
            buf[0][j] *= val[0][j];
        }
        for i in 1..context.channels {
            buf[i] = buf[0].clone();
        }
    }
    fn get_ref(&self) -> Option<RefList> {
        let mut refs = RefList::new();
        if let Some(key) = self.val.get_ref() {
            refs.push(key.to_string());
        }
        if refs.is_empty() {
            None
        } else {
            Some(refs)
        }
    }
}

pub struct Add {
    pub val: Box<dyn Signal>,
}

impl Add {
    pub fn new(val: impl Signal + 'static) -> Box<Self> {
        Box::new(Self { val: Box::new(val) })
    }
}

impl Node for Add {
    fn process(&mut self, context: &mut Context, name: &str) {
        // println!("called");
        let ctx = &mut *context as *mut Context;
        let mut lock;
        let buf;
        unsafe {
            lock = (*ctx).buffers.get_mut(name).unwrap().lock();
            buf = &mut *lock;
        }
        let val = match self.val.give_buf(context) {
            ParamResult::Float(f) => smallvec![smallvec![f; context.frames]],
            ParamResult::Buffer(b) => b,
        };
        for j in 0..context.frames {
            buf[0][j] += val[0][j];
        }
        for i in 1..context.channels {
            buf[i] = buf[0].clone();
        }
    }
    fn get_ref(&self) -> Option<RefList> {
        let mut refs = RefList::new();
        if let Some(key) = self.val.get_ref() {
            refs.push(key.to_string());
        }
        if refs.is_empty() {
            None
        } else {
            Some(refs)
        }
    }
}

pub fn sin_osc() -> Box<SinOsc> {
    SinOsc::new()
}

pub fn mul(val: impl Signal + 'static) -> Box<Mul> {
    Mul::new(val)
}

pub fn add(val: impl Signal + 'static) -> Box<Add> {
    Add::new(val)
}
