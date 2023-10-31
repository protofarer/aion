use std::{collections::HashMap, fs::File, io::BufReader};

use rodio::{
    cpal::traits::HostTrait, source::Buffered, Decoder, DeviceTrait, OutputStream,
    OutputStreamHandle, Source,
};

type BufferedFile = Buffered<Decoder<BufReader<File>>>;
pub struct SoundManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sources: HashMap<String, BufferedFile>,
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

    pub fn load_source(&mut self, name: String, file_path: &str) -> Result<(), anyhow::Error> {
        self.sources.insert(
            name,
            Decoder::new(BufReader::new(
                File::open(file_path).expect("File not found."),
            ))
            .expect("Couldn't load source.")
            .buffered(),
        );
        Ok(())
    }

    pub fn play(&self, name: &str) {
        self.sources.get(name).map_or_else(
            || Err(eprintln!("Audio source not found for name: {}", name)),
            |src| {
                let src = src.clone();
                self.stream_handle.play_raw(src.convert_samples());
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
