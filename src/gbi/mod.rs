mod envelope_generator;
mod square_wave;
mod types;
mod utils;

use std::convert::TryFrom;

use envelope_generator::{EnvelopeGenerator, EnvelopeState};
use square_wave::SquareWaveOscillator;
pub use types::{AudioProcessor, Oscillator};

pub type Signal = (f64, f64);

pub struct GameBoyInstrument {
    square_osc: SquareWaveOscillator,
    envelope_gen: EnvelopeGenerator,
    master_volume: f64,
}

#[derive(Copy, Clone)]
pub enum Parameter {
    MasterVolume = 0,
    AttackTime,
    DecayTime,
    Sustain,
    ReleaseTime,
}

impl TryFrom<u32> for Parameter {
    type Error = ();

    fn try_from(id: u32) -> Result<Self, Self::Error> {
        if id == Parameter::MasterVolume as u32 {
            Ok(Parameter::MasterVolume)
        } else if id == Parameter::AttackTime as u32 {
            Ok(Parameter::AttackTime)
        } else if id == Parameter::DecayTime as u32 {
            Ok(Parameter::DecayTime)
        } else if id == Parameter::Sustain as u32 {
            Ok(Parameter::Sustain)
        } else if id == Parameter::ReleaseTime as u32 {
            Ok(Parameter::ReleaseTime)
        } else {
            Err(())
        }
    }
}

pub trait Parametric<Parameter> {
    fn set_param(&mut self, param: &Parameter, value: f64);
    fn get_param(&self, param: &Parameter) -> f64;
}

impl AudioProcessor<Signal> for GameBoyInstrument {
    fn process(&mut self, sample_rate: f64) -> Signal {
        let osc = self.square_osc.process(sample_rate).to_f64();
        let env = self.envelope_gen.process(sample_rate);

        let signal = osc * env * self.master_volume;
        (signal, signal)
    }
}

impl GameBoyInstrument {
    pub fn new() -> GameBoyInstrument {
        GameBoyInstrument {
            square_osc: SquareWaveOscillator::new(),
            envelope_gen: EnvelopeGenerator::new(),
            master_volume: 1.0,
        }
    }

    pub fn note_on(&mut self, pitch: i16) {
        self.square_osc.set_pitch(pitch);
        self.envelope_gen.set_state(EnvelopeState::Attack);
    }

    pub fn note_off(&mut self) {
        self.envelope_gen.set_state(EnvelopeState::Release);
    }
}

impl Parametric<Parameter> for GameBoyInstrument {
    fn set_param(&mut self, param: &Parameter, value: f64) {
        match param {
            Parameter::MasterVolume => self.master_volume = value,
            Parameter::AttackTime => self.envelope_gen.attack_time = value,
            Parameter::DecayTime => self.envelope_gen.decay_time = value,
            Parameter::Sustain => self.envelope_gen.sustain_val = value,
            Parameter::ReleaseTime => self.envelope_gen.release_time = value,
        }
    }

    fn get_param(&self, param: &Parameter) -> f64 {
        match param {
            Parameter::MasterVolume => self.master_volume,
            Parameter::AttackTime => self.envelope_gen.attack_time,
            Parameter::DecayTime => self.envelope_gen.decay_time,
            Parameter::Sustain => self.envelope_gen.sustain_val,
            Parameter::ReleaseTime => self.envelope_gen.release_time,
        }
    }
}
