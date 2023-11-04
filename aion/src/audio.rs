use std::path::PathBuf;

use audio_manager::{SoundEffectName, SoundManager};
use sfxr::WaveType;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SoundEffectNames {
    Laser,
    TinyShot,
    LightShot,
    MedShot,
    MamaMiaShot,
    Scratch,
    PhysicalDeath,
    PhysicalHarm,
    PlayerPhysicalDeath,
}

impl SoundEffectName for SoundEffectNames {
    fn id(&self) -> u32 {
        match *self {
            SoundEffectNames::Laser => 0,
            SoundEffectNames::TinyShot => 1,
            SoundEffectNames::LightShot => 2,
            SoundEffectNames::MedShot => 3,
            SoundEffectNames::MamaMiaShot => 4,
            SoundEffectNames::Scratch => 5,
            SoundEffectNames::PhysicalDeath => 6,
            SoundEffectNames::PhysicalHarm => 7,
            SoundEffectNames::PlayerPhysicalDeath => 8,
        }
    }
}

pub fn load_essential_sound_effects(sm: &mut SoundManager) -> Result<(), anyhow::Error> {
    sm.load_source_from_assets(SoundEffectNames::TinyShot, "XXXXXXtiny_shot.wav")?;
    sm.load_source_from_assets(SoundEffectNames::Scratch, "scratch.wav")?;
    sm.load_source_from_assets(SoundEffectNames::PhysicalDeath, "physical_death.wav")?;
    sm.load_source_from_assets(SoundEffectNames::PhysicalHarm, "physical_harm.wav")?;
    sm.load_source_from_assets(
        SoundEffectNames::PlayerPhysicalDeath,
        "player_physical_death.wav",
    )?;

    sm.load_source_from_sfxr_sample(SoundEffectNames::Laser, Laser::default());
    Ok(())
}

pub struct Laser;
impl Laser {
    pub fn new(
        wave_type: WaveType,
        base_freq: f64,
        freq_limit: f64,
        freq_ramp: f64,
    ) -> Result<sfxr::Sample, &'static str> {
        if !base_freq.validate(0.1, 1.) {
            return Err("base_freq must be between 0.1 and 1.0");
        }
        if !freq_limit.validate(0., 1.) {
            return Err("freq_limit must be between 0.0 and 1.0");
        }
        if !freq_ramp.validate(-1., -0.01) {
            return Err("freq_ramp must be between -1.0 and -0.01");
        }

        let mut s = Self::default();

        // set specifiables
        s.wave_type = wave_type;
        s.base_freq = base_freq;
        s.freq_limit = freq_limit;
        s.freq_ramp = freq_ramp;

        Ok(s)
    }
    pub fn default() -> sfxr::Sample {
        let mut s = sfxr::Sample::new();
        s.wave_type = sfxr::WaveType::Square;
        s.base_freq = 0.8;
        s.freq_limit = 0.5;
        s.freq_ramp = -0.3;

        // set sample defaults
        // mid means middle value wrt sfxr example rng ranges
        s.env_attack = 0.;
        s.env_sustain = 0.2; // mid
        s.env_decay = 0.1; // mid

        // s.duty = 0.;
        // s.duty_ramp = 0.;

        s
    }
}

pub trait ValidRange {
    fn validate(&self, min: Self, max: Self) -> bool;
}

impl ValidRange for f64 {
    fn validate(&self, min: Self, max: Self) -> bool {
        *self >= min && *self <= max
    }
}
// placeholder for photon/particle shot
// pub struct Laser;
// impl Laser {
//     pub fn new(
//         wave_type: WaveType,
//         base_freq: f64,
//         freq_limit: f64,
//         freq_ramp: f64,
//     ) -> Result<sfxr::Sample, &'static str> {
//         if !base_freq.validate(0.1, 1.) {
//             return Err("base_freq must be between 0.1 and 1.0");
//         }
//         if !freq_limit.validate(0., 1.) {
//             return Err("freq_limit must be between 0.0 and 1.0");
//         }
//         if !freq_ramp.validate(-1., -0.01) {
//             return Err("freq_ramp must be between -1.0 and -0.01");
//         }

//         let mut s = Self::default();

//         // set specifiables
//         s.wave_type = wave_type;
//         s.base_freq = base_freq;
//         s.freq_limit = freq_limit;
//         s.freq_ramp = freq_ramp;

//         Ok(s)
//     }
//     pub fn default() -> sfxr::Sample {
//         let mut s = sfxr::Sample::new();
//         s.wave_type = sfxr::WaveType::Square;
//         s.base_freq = 0.8;
//         s.freq_limit = 0.5;
//         s.freq_ramp = -0.3;

//         // set sample defaults
//         // mid means middle value wrt sfxr example rng ranges
//         s.env_attack = 0.;
//         s.env_sustain = 0.2; // mid
//         s.env_decay = 0.1; // mid

//         // s.duty = 0.;
//         // s.duty_ramp = 0.;

//         s
//     }
// }
