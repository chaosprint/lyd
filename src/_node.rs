use crate::{Buffer, Context};
use parking_lot::Mutex;
use smallvec::{smallvec, SmallVec};
use std::sync::Arc;

pub type Ref = SmallVec<[String; 5]>;

pub fn sin_osc() -> Box<SinOsc> {
    SinOsc::new()
}

pub trait Node: Send {
    fn process(&mut self, context: &mut Context, name: &str);
    fn get_ref(&self) -> Option<Ref>;
}

pub trait Signal: Send {
    fn give_buf(&self, context: &mut Context) -> &Arc<Mutex<Buffer>>;
    fn get_ref(&self) -> Option<&str>;
}

impl Signal for f32 {
    fn give_buf(&self, context: &mut Context) -> &Arc<Mutex<Buffer>> {
        context.temp_buffer = Arc::new(Mutex::new(smallvec![smallvec![*self; context.frames]]));
        &context.temp_buffer
    }
    fn get_ref(&self) -> Option<&str> {
        None
    }
}

impl Signal for &str {
    fn give_buf(&self, context: &mut Context) -> &Arc<Mutex<Buffer>> {
        let cell = context.buffers.get(*self).unwrap();
        cell
        // unsafe { &*cell.get() } // safe as we just need read it
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
        let mut cell = context.buffers.get_mut(name).unwrap().lock();
        // let buf = unsafe { &mut *cell.get() };
        let buf = &mut *cell;
        let freq = self.freq.give_buf(context).lock();
        for j in 0..context.frames {
            buf[0][j] = (self.phase * 2.0 * std::f32::consts::PI).sin() * self.amp;
            self.phase += freq[0][j] / context.sr as f32;
        }
        for i in 1..context.channels {
            buf[i] = buf[0].clone();
        }
    }
    fn get_ref(&self) -> Option<Ref> {
        let mut refs = Ref::new();
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

impl Into<Box<dyn Node + 'static>> for SinOsc {
    fn into(self) -> Box<dyn Node + 'static> {
        Box::new(self) as Box<dyn Node + 'static>
    }
}

// impl Into<Box<dyn Node + 'static>> for Con {
//     fn into(self) -> Box<dyn Node + 'static> {
//         Box::new(self) as Box<dyn Node + 'static>
//     }
// }

// pub fn con(val: impl Signal + 'static) -> Box<Con> {
//     Con::new(val)
// }

// pub struct Con {
//     pub val: Box<dyn Signal>,
// }

// impl Con {
//     pub fn new(val: impl Signal + 'static) -> Box<Self> {
//         Box::new(Self { val: Box::new(val) })
//     }
// }

// impl Node for Con {
//     fn process(&mut self, context: &mut Context, name: &str) {
//         let cell = context.buffers.get_mut(name).unwrap();
//         // let buf = unsafe { &mut *cell.get() };
//         let buf = &mut *cell.lock();
//         let val_buf = self.val.give_buf(context).lock();
//         for j in 0..context.frames {
//             buf[0][j] = val_buf[0][j];
//         }
//         for i in 1..context.channels {
//             buf[i] = buf[0].clone();
//         }
//     }
//     fn get_ref(&self) -> Option<Ref> {
//         let mut refs = Ref::new();
//         if let Some(key) = self.val.get_ref() {
//             refs.push(key.to_string());
//         }
//         if refs.is_empty() {
//             None
//         } else {
//             Some(refs)
//         }
//     }
// }

impl Into<Box<dyn Node + 'static>> for Mul {
    fn into(self) -> Box<dyn Node + 'static> {
        Box::new(self) as Box<dyn Node + 'static>
    }
}

pub fn mul(val: impl Signal + 'static) -> Box<Mul> {
    Mul::new(val)
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
        let cell = context.buffers.get_mut(name).unwrap();
        // let buf = unsafe { &mut *cell.get() };
        let buf = &mut *cell.lock();
        let val_buf = self.val.give_buf(context).lock();
        for j in 0..context.frames {
            buf[0][j] *= val_buf[0][j];
        }
        for i in 1..context.channels {
            buf[i] = buf[0].clone();
        }
    }
    fn get_ref(&self) -> Option<Ref> {
        let mut refs = Ref::new();
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
