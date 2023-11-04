use std::{collections::HashMap, fs::File, io::BufReader};

use anyhow::Context;
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
        self.id() == other.id()
    }
}

impl Eq for dyn SoundEffectName {}

use std::hash::{Hash, Hasher};
impl Hash for dyn SoundEffectName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

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

        let device = match host
            .output_devices()
            .expect("Should retrieve audio output device")
            .find(|x| x.name().map(|y| y == device_name).unwrap_or(false))
            .with_context(|| format!("Failed to find audio output device: {}", device_name))
        {
            Ok(device) => device,
            Err(e) => return Err(e),
        };

        let (_stream, stream_handle) = match OutputStream::try_from_device(&device)
            .with_context(|| "Failed to create outputstream")
        {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

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

        match self.sources.insert(name.id(), source) {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("No entry for id: {}", name.id())),
        }
    }

    pub fn load_source_from_assets(
        &mut self,
        name: impl SoundEffectName,
        file_name: &str,
    ) -> Result<(), anyhow::Error> {
        let home_key = if cfg!(target_os = "windows") {
            "USERPROFILE"
        } else {
            "HOME"
        };
        let home_dir = match std::env::var(home_key) {
            Ok(dir) => dir,
            Err(e) => return Err(anyhow::anyhow!("Couldnt determine home directory. {}", e)),
        };

        let project_root = std::path::PathBuf::from(home_dir).join("projects/aion/aion/assets/");

        let file_path = project_root.join(file_name);
        let file = File::open(file_path.clone())
            .with_context(|| format!("Missing file: {:?}", file_path.to_str().unwrap_or("")))?;

        let decoder = Decoder::new(BufReader::new(file))
            .with_context(|| "Failed to decode file bufreader")?
            .buffered();

        let source = SoundSource::BufferedFile(decoder);

        self.sources.insert(name.id(), source);

        Ok(())
    }

    pub fn play(&self, name: impl SoundEffectName) {
        if let Some(sound_source) = self.sources.get(&name.id()) {
            let result = match sound_source {
                SoundSource::BufferedFile(source) => self
                    .stream_handle
                    .play_raw(source.clone().convert_samples()),
                SoundSource::SfxrBuffer(source) => self
                    .stream_handle
                    .play_raw(source.clone().convert_samples()),
            };

            if let Err(e) = result {
                eprintln!("Error playing sound: {e}");
            }
        } else {
            eprintln!("No source loaded for given sound effect name.");
        }
    }
}

pub fn scan_usable_devices_by_sound() {
    const SLEEP_DURATION_SECS: u64 = 1;

    let host = rodio::cpal::default_host();
    let output_devices = host.output_devices().unwrap();

    println!("\n\nScanning for usable devices, listen for sound. Watch for errors.",);
    println!("NB: These are *ALSA* device names as provided by `cpal`.\n",);
    for device in output_devices {
        // todo use hold in memory instead (faster than file i/o) ? what if file mutates?
        let file =
            BufReader::new(File::open("/home/kenny/projects/aion/aion/assets/jump.wav").unwrap());
        let my_source = Decoder::new(file).unwrap();

        // let name = device.name().unwrap();

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
