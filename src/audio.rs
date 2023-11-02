use std::{collections::HashMap, fs::File, io::BufReader};

use rodio::{
    cpal::traits::HostTrait, source::Buffered, Decoder, DeviceTrait, OutputStream,
    OutputStreamHandle, Source,
};

#[derive(Clone, Copy)]
pub struct SfxrBuffer {
    buffer: [f32; 44_100],
    index: usize,
}

impl Iterator for SfxrBuffer {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.index >= self.buffer.len() {
            None
        } else {
            let sample = self.buffer[self.index];
            self.index += 1;
            Some(sample)
        }
    }
}

impl Source for SfxrBuffer {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.buffer.len() - self.index)
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        44_100
    }
    fn total_duration(&self) -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs_f32(
            self.buffer.len() as f32 / 44_100.,
        ))
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum SoundEffectName {
    TinyShot,
    LightShot,
    MedShot,
    MamaMiaShot,
    Scratch,
    PhysicalDeath,
    PhysicalHarm,
    PlayerPhysicalDeath,
}

type BufferedFile = Buffered<Decoder<BufReader<File>>>;

pub enum SoundSource {
    BufferedFile(BufferedFile),
    SfxrBuffer(SfxrBuffer),
}
pub struct SoundManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sources: HashMap<SoundEffectName, SoundSource>,
}

impl SoundManager {
    pub fn new() -> Result<Self, anyhow::Error> {
        let device_name = "hdmi:CARD=HDMI,DEV=2";

        let host = rodio::cpal::default_host();
        let device = host
            .output_devices()
            .unwrap()
            .find(|x| x.name().map(|y| y == device_name).unwrap_or(false))
            .expect("Failed to find audio output device");

        let (_stream, stream_handle) = OutputStream::try_from_device(&device).unwrap();

        Ok(Self {
            _stream,
            stream_handle,
            sources: HashMap::new(),
        })
    }

    pub fn load_source(
        &mut self,
        name: SoundEffectName,
        file_path: &str,
    ) -> Result<(), anyhow::Error> {
        let source = SoundSource::BufferedFile(
            Decoder::new(BufReader::new(
                File::open(file_path).expect("Audio file not found."),
            ))
            .expect("Couldn't load source.")
            .buffered(),
        );
        self.sources.insert(name, source);
        Ok(())
    }

    pub fn dev_gen_source(
        &mut self,
        name: SoundEffectName,
        sample: sfxr::Sample,
    ) -> Result<(), anyhow::Error> {
        let mut generator = sfxr::Generator::new(sample);

        // I need to feed params into a `sfxr::Sample::... to get... a buffer?`
        let mut source = SoundSource::SfxrBuffer(SfxrBuffer {
            buffer: [0.; 44_100],
            index: 0,
        });

        if let SoundSource::SfxrBuffer(sfxr_buffer) = &mut source {
            generator.generate(&mut sfxr_buffer.buffer);
        }
        self.sources.insert(name, source);
        Ok(())
    }

    // pub fn gen_source(&mut self, params: SourceParams) {
    //     // TODO read params to gen buffer of samples using sfxr
    //     // TODO clamp/bound params or resulting sample to ensure "reasonableness": volume range, distortion, length, etc...
    // }

    pub fn play(&self, name: &SoundEffectName) {
        self.sources.get(name).map_or_else(
            || Err(eprintln!("Audio source not found for name.")),
            |sound_source| {
                match sound_source {
                    SoundSource::BufferedFile(source) => {
                        self.stream_handle
                            .play_raw(source.clone().convert_samples());
                    }
                    SoundSource::SfxrBuffer(source) => {
                        self.stream_handle
                            .play_raw(source.clone().convert_samples());
                    }
                }
                Ok(())
            },
        );
    }

    pub fn scan_usable_devices_by_sound() {
        const SLEEP_DURATION_SECS: u64 = 1;

        let host = rodio::cpal::default_host();
        let output_devices = host.output_devices().unwrap();

        println!("\n\nScanning for usable devices, listen for sound. Watch for errors.",);
        println!("NB: These are *ALSA* device names as provided by `cpal`.\n",);
        for device in output_devices {
            // todo use hold in memory instead (faster than file i/o)
            let file = BufReader::new(File::open("jump.wav").unwrap());
            let my_source = Decoder::new(file).unwrap();

            let name = device.name().unwrap();
            println!("{}", name);

            match OutputStream::try_from_device(&device) {
                Ok((_stream, stream_handle)) => {
                    match stream_handle.play_raw(my_source.convert_samples()) {
                        Ok(_) => {
                            println!("\tPlaying on device...");
                            std::thread::sleep(std::time::Duration::from_secs(SLEEP_DURATION_SECS));
                            println!("\tDone.");
                        }
                        Err(e) => {
                            println!("\tFailed to play on device: {:?}", e)
                        }
                    }
                }
                Err(e) => {
                    eprintln!("\tFailed to create output stream: {:?}", e);
                }
            };
            println!("---------------------------------------");
        }
    }
}

// type AionProjectileEmissionSample = sfxr::Sample;
// can be any sfxr sample archetype..corresponding to appropriate aion projectile: particle/energy/laser cannon/technical/matter

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
