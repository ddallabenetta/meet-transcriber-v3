use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};
use hound::{WavSpec, WavWriter};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Nessun dispositivo di input trovato")]
    NoInputDevice,
    #[error("Errore dispositivo: {0}")]
    DeviceError(String),
    #[error("Errore stream: {0}")]
    StreamError(String),
    #[error("Errore file: {0}")]
    FileError(String),
}

pub struct AudioRecorder {
    pub is_recording: Arc<AtomicBool>,
    pub output_path: Option<PathBuf>,
    stop_signal: Option<Arc<AtomicBool>>,
}

impl AudioRecorder {
    pub fn new() -> Self {
        Self {
            is_recording: Arc::new(AtomicBool::new(false)),
            output_path: None,
            stop_signal: None,
        }
    }

    pub fn start_recording(
        &mut self,
        output_path: PathBuf,
        device_id: Option<String>,
    ) -> Result<(), AudioError> {
        if self.is_recording.load(Ordering::SeqCst) {
            return Err(AudioError::StreamError(
                "Registrazione già in corso".to_string(),
            ));
        }

        let host = cpal::default_host();

        let (device, is_loopback) = if let Some(id) = device_id {
            if id.starts_with("loopback_") {
                let idx: usize = id.replace("loopback_", "").parse().unwrap_or(0);
                let device = host.output_devices()
                    .map_err(|e| AudioError::DeviceError(e.to_string()))?
                    .nth(idx)
                    .ok_or(AudioError::NoInputDevice)?;
                (device, true)
            } else {
                let idx: usize = id.replace("input_", "").parse().unwrap_or(0);
                let device = host.input_devices()
                    .map_err(|e| AudioError::DeviceError(e.to_string()))?
                    .nth(idx)
                    .ok_or(AudioError::NoInputDevice)?;
                (device, false)
            }
        } else {
            (host.default_input_device()
                .ok_or(AudioError::NoInputDevice)?, false)
        };

        let config = if is_loopback {
             device
                .default_output_config()
                .map_err(|e| AudioError::DeviceError(e.to_string()))?
        } else {
            device
                .default_input_config()
                .map_err(|e| AudioError::DeviceError(e.to_string()))?
        };

        let sample_format = config.sample_format();
        let config: cpal::StreamConfig = config.into();

        let spec = WavSpec {
            channels: config.channels,
            sample_rate: config.sample_rate.0,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let writer = WavWriter::create(&output_path, spec)
            .map_err(|e| AudioError::FileError(e.to_string()))?;
        let writer = Arc::new(Mutex::new(Some(writer)));

        let is_recording = Arc::new(AtomicBool::new(true));
        let stop_signal = is_recording.clone();

        // Clone per ciascuna closure
        let is_recording_i16 = is_recording.clone();
        let writer_i16 = writer.clone();
        let is_recording_f32 = is_recording.clone();
        let writer_f32 = writer.clone();
        let is_recording_loop = is_recording.clone();
        let writer_finalize = writer.clone();

        thread::spawn(move || {
            let err_fn = |err| eprintln!("Errore stream audio: {}", err);

            // Note: For WASAPI loopback, building an input stream on an output device works.
            let stream = match sample_format {
                SampleFormat::I16 => device.build_input_stream(
                    &config,
                    move |data: &[i16], _: &_| {
                        if is_recording_i16.load(Ordering::SeqCst) {
                            if let Ok(mut guard) = writer_i16.lock() {
                                if let Some(ref mut writer) = *guard {
                                    for &sample in data {
                                        let _ = writer.write_sample(sample);
                                    }
                                }
                            }
                        }
                    },
                    err_fn,
                    None,
                ),
                SampleFormat::F32 => device.build_input_stream(
                    &config,
                    move |data: &[f32], _: &_| {
                        if is_recording_f32.load(Ordering::SeqCst) {
                            if let Ok(mut guard) = writer_f32.lock() {
                                if let Some(ref mut writer) = *guard {
                                    for &sample in data {
                                        let sample_i16: i16 = Sample::from_sample(sample);
                                        let _ = writer.write_sample(sample_i16);
                                    }
                                }
                            }
                        }
                    },
                    err_fn,
                    None,
                ),
                _ => {
                    eprintln!("Formato sample non supportato: {:?}", sample_format);
                    return;
                }
            };

            match stream {
                Ok(stream) => {
                    if let Err(e) = stream.play() {
                        eprintln!("Errore avvio stream: {}", e);
                        return;
                    }

                    while is_recording_loop.load(Ordering::SeqCst) {
                        thread::sleep(std::time::Duration::from_millis(100));
                    }

                    if let Ok(mut guard) = writer_finalize.lock() {
                        if let Some(writer) = guard.take() {
                            let _ = writer.finalize();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Errore creazione stream: {}", e);
                }
            }
        });

        self.is_recording.store(true, Ordering::SeqCst);
        self.output_path = Some(output_path);
        self.stop_signal = Some(stop_signal);

        Ok(())
    }

    pub fn stop_recording(&mut self) -> Result<PathBuf, AudioError> {
        if let Some(stop_signal) = &self.stop_signal {
            stop_signal.store(false, Ordering::SeqCst);
        }

        self.is_recording.store(false, Ordering::SeqCst);

        thread::sleep(std::time::Duration::from_millis(200));

        self.stop_signal = None;

        self.output_path.take().ok_or(AudioError::FileError(
            "Nessuna registrazione attiva".to_string(),
        ))
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }
}

impl Default for AudioRecorder {
    fn default() -> Self {
        Self::new()
    }
}
