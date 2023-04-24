#[allow(unused_imports)]
use crate::enums::*;
use crate::params::*;

#[macro_export]
macro_rules! sin_osc {
    // ( { $($key:ident : $value:expr),* $(,)? } ) => {
    //     {
    //         let mut config = SinOscConfig::default();
    //         $(
    //             match stringify!($key) {
    //                 "freq" => config.freq = $value as f32,
    //                 "phase" => config.phase = $value as f32,
    //                 "amp" => config.amp = $value as f32,
    //                 "sr" => config.sr = $value as u32,
    //                 _ => {}
    //             }
    //         )*
    //         NodeConfig::SinOsc(config)
    //     }
    // };
    ( $($value:expr),* $(,)? ) => {
        {
            let mut config = SinOscConfig::default();
            let mut values = [$($value),*].iter();
            if let Some(&freq) = values.next() {
                config.freq = freq.as_param();
            }
            if let Some(&phase) = values.next() {
                config.phase = phase.as_param();
            }
            if let Some(&amp) = values.next() {
                config.amp = amp.as_param();
            }
            NodeConfig::SinOsc(config)
        }
    };
}

#[macro_export]
macro_rules! add {
    ( $($value:expr),* $(,)? ) => {
        {
            let mut config = AddConfig { add: Param::Float(0.0) };
            let mut values = [$($value),*].iter();
            if let Some(&val) = values.next() {
                config.add = val.as_param();
            }

            NodeConfig::Add(config)
        }
    };
}
