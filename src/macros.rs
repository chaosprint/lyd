use crate::enums::*;

#[macro_export]
macro_rules! sin_osc {
    () => {
        sin_osc!({})
    };
    ( {} ) => {
        sin_osc!({freq: 440.0, phase: 0.0, amp: 1.0, sr: 44100})
    };
    ( { $($key:ident : $value:expr),* $(,)? } ) => {
        {
            let mut osc = SinOscConfig::default();
            $(
                match stringify!($key) {
                    "freq" => osc.freq = $value,
                    "phase" => osc.phase = $value,
                    "amp" => osc.amp = $value,
                    "sr" => osc.sr = $value,
                    _ => {}
                }
            )*
            NodeConfig::SinOsc(osc)
        }
    };
    ( $($value:expr),* $(,)? ) => {
        {
            let mut osc = SinOscConfig::default();
            let mut values = [$($value),*].iter();
            if let Some(&freq) = values.next() {
                osc.freq = freq;
            }
            if let Some(&phase) = values.next() {
                osc.phase = phase;
            }
            if let Some(&amp) = values.next() {
                osc.amp = amp;
            }
            if let Some(&sr) = values.next() {
                osc.sr = sr as u32;
            }
            NodeConfig::SinOsc(osc)
        }
    };
}
