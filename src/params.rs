use crate::Context;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy)]
pub enum Param {
    Float(f32),
    Int(u32),
}

pub trait AsParam {
    fn as_param(&self) -> Param;
}

impl AsParam for f32 {
    fn as_param(&self) -> Param {
        Param::Float(*self)
    }
}

impl AsParam for u32 {
    fn as_param(&self) -> Param {
        Param::Int(*self)
    }
}

impl Param {
    pub fn as_float(&self) -> f32 {
        match self {
            Param::Float(f) => *f,
            Param::Int(i) => *i as f32,
        }
    }

    pub fn as_int(&self) -> u32 {
        match self {
            Param::Float(f) => *f as u32,
            Param::Int(i) => *i,
        }
    }
}
