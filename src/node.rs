use crate::Context;
use smallvec::SmallVec;

pub type RefList = SmallVec<[String; 5]>; // not likely to have more than 5 refs
pub enum ParamResult {
    Float(f32),
    Buffer(*const SmallVec<[f32; 1024]>),
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
        let buf: &SmallVec<[SmallVec<[f32; 1024]>; 2]> = context.buffers.get(*self).unwrap();
        let buf_ptr: *const SmallVec<[f32; 1024]> = buf.as_ptr();
        ParamResult::Buffer(buf_ptr)
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
        let ctx = &mut *context as *mut Context;
        let lock;
        let buf;
        unsafe {
            lock = (*ctx).buffers.get_mut(name).unwrap(); //.lock();
            buf = &mut *lock;
        }
        let frames = context.frames;
        let two_pi = 2.0 * std::f32::consts::PI;
        let inv_sr = 1.0 / context.sr as f32;

        let result = self.freq.give_buf(context);

        for j in 0..frames {
            buf[0][j] = (self.phase * two_pi).sin() * self.amp;
            // let freq = unsafe { &*freq_ptr.add(j) };
            self.phase += match result {
                ParamResult::Float(f) => f * inv_sr,
                ParamResult::Buffer(b) => unsafe { (*b)[j] * inv_sr },
            };
        }

        let buf0_ptr: *const f32 = buf[0].as_ptr();
        for channel in buf.iter_mut().skip(1) {
            unsafe {
                std::ptr::copy_nonoverlapping(buf0_ptr, channel.as_mut_ptr(), frames);
            }
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
        let ctx = &mut *context as *mut Context;
        let lock: &mut SmallVec<[SmallVec<[f32; 1024]>; 2]>;
        let buf;
        unsafe {
            lock = (*ctx).buffers.get_mut(name).unwrap(); //.lock();
            buf = &mut *lock;
        }
        let frames = context.frames;

        // let val_ptr = match self.val.give_buf(context) {
        //     ParamResult::Float(f) => {
        //         let v = Box::new([f; 1024]); // only one chan
        //         v.as_ptr()
        //     }
        //     ParamResult::Buffer(b) => unsafe { (*b).as_ptr() },
        // };

        let result = self.val.give_buf(context);

        for j in 0..frames {
            // let val = unsafe { &*val_ptr.add(j) };
            buf[0][j] *= match result {
                ParamResult::Float(f) => f,
                ParamResult::Buffer(b) => unsafe { (*b)[j] },
            };
        }

        let buf0_ptr: *const f32 = buf[0].as_ptr();
        for channel in buf.iter_mut().skip(1) {
            unsafe {
                std::ptr::copy_nonoverlapping(buf0_ptr, channel.as_mut_ptr(), frames);
            }
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
        let lock;
        let buf;
        unsafe {
            lock = (*ctx).buffers.get_mut(name).unwrap();
            buf = &mut *lock;
        }
        let frames = context.frames;

        let result = self.val.give_buf(context);

        for j in 0..frames {
            // let val = unsafe { &*val_ptr.add(j) };
            buf[0][j] += match result {
                ParamResult::Float(f) => f,
                ParamResult::Buffer(b) => unsafe { (*b)[j] },
            };
        }

        // let val_ptr = match self.val.give_buf(context) {
        //     ParamResult::Float(f) => {
        //         let v = Box::new([f; 1024]); // only one chan
        //         v.as_ptr()
        //     }
        //     ParamResult::Buffer(b) => unsafe { (*b).as_ptr() },
        // };

        // for j in 0..frames {
        //     let val = unsafe { &*val_ptr.add(j) };
        //     buf[0][j] *= val;
        // }

        let buf0_ptr: *const f32 = buf[0].as_ptr();
        for channel in buf.iter_mut().skip(1) {
            unsafe {
                std::ptr::copy_nonoverlapping(buf0_ptr, channel.as_mut_ptr(), frames);
            }
        }
        // let val = match self.val.give_buf(context) {
        //     ParamResult::Float(f) => smallvec![smallvec![f; context.frames]],
        //     ParamResult::Buffer(b) => b,
        // };
        // for j in 0..context.frames {
        //     buf[0][j] += val[0][j];
        // }
        // for i in 1..context.channels {
        //     buf[i] = buf[0].clone();
        // }
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
