// can be any sfxr sample archetype..corresponding to appropriate aion projectile: particle/energy/laser cannon/technical/matter

use audio_manager::{SoundEffectName, SoundManager};
use sfxr::WaveType;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SoundEffectNames {
    DefaultLaser,
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
            SoundEffectNames::DefaultLaser => 0,
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

// impl SoundEffectName for SoundEffectNames {
//     fn id(&self) -> TypeId {
//         match self {
//             SoundEffectNames::DefaultLaser => TypeId::of::<SoundEffectNames::DefaultLaser>(),
//             SoundEffectNames::TinyShot => TypeId::of::<SoundEffectNames::TinyShot>(),
//             SoundEffectNames::LightShot => TypeId::of::<SoundEffectNames::LightShot>(),
//             SoundEffectNames::MedShot => TypeId::of::<SoundEffectNames::MedShot>(),
//             SoundEffectNames::MamaMiaShot => TypeId::of::<SoundEffectNames::MamaMiaShot>(),
//             SoundEffectNames::Scratch => TypeId::of::<SoundEffectNames::Scratch>(),
//             SoundEffectNames::PhysicalDeath => TypeId::of::<SoundEffectNames::PhysicalDeath>(),
//             SoundEffectNames::PhysicalHarm => TypeId::of::<SoundEffectNames::PhysicalHarm>(),
//             SoundEffectNames::PlayerPhysicalDeath => {
//                 TypeId::of::<SoundEffectNames::PlayerPhysicalDeath>()
//             }
//         }
//     }
// }
// TODO trait
// fn load_;

pub fn load_core_sound_effects(sm: &mut SoundManager) -> Result<(), anyhow::Error> {
    sm.load_source_from_file(
        SoundEffectNames::TinyShot,
        "/home/kenny/projects/aion/aion/assets/tiny_shot.wav",
    )?;
    sm.load_source_from_file(
        SoundEffectNames::Scratch,
        "/home/kenny/projects/aion/aion/assets/scratch.wav",
    )?;
    sm.load_source_from_file(
        SoundEffectNames::PhysicalDeath,
        "/home/kenny/projects/aion/aion/assets/physical_death.wav",
    )?;
    sm.load_source_from_file(
        SoundEffectNames::PhysicalHarm,
        "/home/kenny/projects/aion/aion/assets/physical_harm.wav",
    )?;
    sm.load_source_from_file(
        SoundEffectNames::PlayerPhysicalDeath,
        "/home/kenny/projects/aion/aion/assets/player_physical_death.wav",
    )?;

    // TODO pass in sm, use its load_sfxr_source
    sm.load_source_from_sfxr_sample(SoundEffectNames::DefaultLaser, AionLaser::default());

    Ok(())
}

// ? builder pattern
pub struct AionLaser;
impl AionLaser {
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
        // let mut s = sfxr::Sample::new();

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
        // sfxr::Sample::laser(None)
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

// wave_type: WaveType::Square,
// base_freq: 0.3,
// freq_limit: 0.0,
// freq_ramp: 0.0,
// freq_dramp: 0.0,
// duty: 0.0,
// duty_ramp: 0.0,

// vib_strength: 0.0,
// vib_speed: 0.0,
// vib_delay: 0.0,

// env_attack: 0.4,
// env_sustain: 0.1,
// env_decay: 0.5,
// env_punch: 0.0,

// lpf_resonance: 0.0,
// lpf_freq: 1.0,
// lpf_ramp: 0.0,
// hpf_freq: 0.0,
// hpf_ramp: 0.0,

// pha_offset: 0.0,
// pha_ramp: 0.0,

// repeat_speed: 0.0,

// arp_speed: 0.0,
// arp_mod: 0.0
// }

// let rng = &mut SmallRng::seed_from_u64(seed.unwrap_or(0));
// let mut s = Sample::new();

// let wave_types = {
//     use WaveType::*;
//     [Square, Square, Sine, Sine, Triangle]
// };
// s.wave_type = rand_element(rng, &wave_types);

// if rand_bool(rng, 1, 2) {
//     s.base_freq = rand_f64(rng, 0.3, 0.9);
//     s.freq_limit = rand_f64(rng, 0.0, 0.1);
//     s.freq_ramp = rand_f64(rng, -0.35, -0.65);
// } else {
//     s.base_freq = rand_f64(rng, 0.5, 1.0);
//     s.freq_limit = (s.base_freq - rand_f64(rng, 0.2, 0.8)).max(0.2);
//     s.freq_ramp = rand_f64(rng, -0.15, -0.35);
// }

// if rand_bool(rng, 1, 1) {
//     s.duty = rand_f32(rng, 0.0, 0.5);
//     s.duty_ramp = rand_f32(rng, 0.0, 0.2);
// } else {
//     s.duty = rand_f32(rng, 0.4, 0.9);
//     s.duty_ramp = rand_f32(rng, 0.0, -0.7);
// }

// s.env_attack = 0.0;
// s.env_sustain = rand_f32(rng, 0.1, 0.3);
// s.env_decay = rand_f32(rng, 0.0, 0.4);

// if rand_bool(rng, 1, 1) {
//     s.env_punch = rand_f32(rng, 0.0, 0.3);
// }

// if rand_bool(rng, 1, 2) {
//     s.pha_offset = rand_f32(rng, 0.0, 0.2);
//     s.pha_ramp = -rand_f32(rng, 0.0, 0.2);
// }

// if rand_bool(rng, 1, 1) {
//     s.hpf_freq = rand_f32(rng, 0.0, 0.3);
// }

// s
