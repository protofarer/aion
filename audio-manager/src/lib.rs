use std::{collections::HashMap, fs::File, io::BufReader};

// use egui_wgpu::wgpu::Device;
use rodio::{
    cpal::traits::HostTrait, source::Buffered, Decoder, DeviceTrait, OutputStream,
    OutputStreamHandle, Source,
};
use sfxr::Sample;

pub trait SoundEffectName: 'static {
    fn id(&self) -> u32;
}

impl PartialEq for dyn SoundEffectName {
    fn eq(&self, other: &Self) -> bool {
        // Compare the underlying SoundEffectName values
        self.id() == other.id()
    }
}

impl Eq for dyn SoundEffectName {}

use std::hash::{Hash, Hasher};
impl Hash for dyn SoundEffectName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the underlying SoundEffectName value
        self.id().hash(state);
    }
}
// pub struct SoundEffectNameWrapper(Box<dyn SoundEffectName>);

// impl PartialEq for SoundEffectNameWrapper {
//     fn eq(&self, other: &Self) -> bool {
//         // Compare the underlying SoundEffectName values
//         self.0.id() == other.0.id()
//     }
// }

// impl Eq for SoundEffectNameWrapper {}

// use std::hash::{Hash, Hasher};
// impl Hash for SoundEffectNameWrapper {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         // Hash the underlying SoundEffectName value
//         self.0.id().hash(state);
//     }
// }

#[derive(Clone, Copy)]
pub struct SfxrBuffer {
    // buffer: [f32; 44_100],
    buffer: [f32; 22050], // ? set buffer size precisely using binary search after buffer is mutated by sfxr generator
    index: usize,
}

impl SfxrBuffer {
    pub fn new() -> Self {
        Self {
            // buffer: [0.; 44_100],
            buffer: [0.; 22050],
            index: 0,
        }
    }
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

type BufferedFile = Buffered<Decoder<BufReader<File>>>;

pub enum SoundSource {
    BufferedFile(BufferedFile),
    SfxrBuffer(SfxrBuffer),
}

type HashMapSoundEffects = HashMap<u32, SoundSource>;
pub struct SoundManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sources: HashMapSoundEffects,
}

impl SoundManager {
    pub fn new() -> Result<Self, anyhow::Error> {
        // let device_name = "hdmi:CARD=HDMI,DEV=2";
        // let device_name = "front:CARD=Generic,DEV=0";
        let device_name = "sysdefault:CARD=Generic";

        let host = rodio::cpal::default_host();
        let device = host
            .output_devices()
            .unwrap()
            .find(|x| x.name().map(|y| y == device_name).unwrap_or(false))
            .expect("Failed to find audio output device");

        let (_stream, stream_handle) = OutputStream::try_from_device(&device).unwrap();
        // let sources = match SoundManager::load_core_sound_effects() {
        // Ok(sources) => sources,
        //     Err(e) => return Err(anyhow::anyhow!("Failed to load core sound effects")),
        // };

        Ok(Self {
            _stream,
            stream_handle,
            sources: HashMap::new(),
        })
    }
    pub fn load_source_from_sfxr_sample(
        &mut self,
        name: impl SoundEffectName,
        sample: Sample,
    ) -> Result<(), anyhow::Error> {
        let mut generator = sfxr::Generator::new(sample);
        let mut source = SoundSource::SfxrBuffer(SfxrBuffer::new());

        if let SoundSource::SfxrBuffer(sfxr_buffer) = &mut source {
            generator.generate(&mut sfxr_buffer.buffer);
        }

        // println!("in load core sfx",);
        // if let SoundSource::SfxrBuffer(source) = &mut source {
        //     if !source.buffer.iter().all(|&sample| sample != 0.0) {
        //         println!("buffer not filled completely",);
        //     }
        // }

        self.sources.insert(name.id(), source);
        Ok(())
    }

    pub fn load_source_from_file(
        &mut self,
        name: impl SoundEffectName,
        file_path: &str,
    ) -> Result<(), anyhow::Error> {
        let source = SoundSource::BufferedFile(
            Decoder::new(BufReader::new(
                File::open(file_path).expect("Audio file not found."),
            ))
            .expect("Couldn't load source.")
            .buffered(),
        );
        self.sources.insert(name.id(), source);
        Ok(())
    }

    pub fn dev_gen_source(
        &mut self,
        name: impl SoundEffectName,
        sample: sfxr::Sample,
    ) -> Result<(), anyhow::Error> {
        let mut generator = sfxr::Generator::new(sample);

        // I need to feed params into a `sfxr::Sample::... to get... a buffer?`
        let mut source = SoundSource::SfxrBuffer(SfxrBuffer::new());

        if let SoundSource::SfxrBuffer(sfxr_buffer) = &mut source {
            generator.generate(&mut sfxr_buffer.buffer);
        }
        self.sources.insert(name.id(), source);
        Ok(())
    }

    // pub fn gen_source(&mut self, params: SourceParams) {
    //     // TODO read params to gen buffer of samples using sfxr
    //     // TODO clamp/bound params or resulting sample to ensure "reasonableness": volume range, distortion, length, etc...
    // }

    pub fn play(&self, name: impl SoundEffectName) {
        self.sources.get(&name.id()).map_or_else(
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
