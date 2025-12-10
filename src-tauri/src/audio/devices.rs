use cpal::traits::{DeviceTrait, HostTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_input: bool,
    pub is_default: bool,
}

pub fn list_audio_devices() -> Vec<AudioDevice> {
    let mut devices = Vec::new();
    let host = cpal::default_host();

    // Get default input device name for comparison
    let default_input_name = host.default_input_device().and_then(|d| d.name().ok());

    // List input devices (microphones)
    if let Ok(input_devices) = host.input_devices() {
        for (idx, device) in input_devices.enumerate() {
            if let Ok(name) = device.name() {
                let is_default = default_input_name.as_ref() == Some(&name);
                devices.push(AudioDevice {
                    id: format!("input_{}", idx),
                    name: name.clone(),
                    is_input: true,
                    is_default,
                });
            }
        }
    }

    // Note: System audio capture requires platform-specific handling
    // macOS: ScreenCaptureKit or virtual audio device (BlackHole)
    // Windows: WASAPI loopback
    // Linux: PulseAudio monitor

    devices
}
