use cpal::traits::{DeviceTrait, HostTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_input: bool,
    pub is_default: bool,
    pub is_loopback: bool,
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
                    is_loopback: false,
                });
            }
        }
    }

    // List output devices as loopback sources (only on Windows/WASAPI)
    // Note: System audio capture requires platform-specific handling
    // Windows: WASAPI loopback - achieved by using output device as input source
    #[cfg(target_os = "windows")]
    if host.id() == cpal::HostId::Wasapi {
         if let Ok(output_devices) = host.output_devices() {
            for (idx, device) in output_devices.enumerate() {
                if let Ok(name) = device.name() {
                    // We treat loopback devices as inputs so they appear in selection
                    devices.push(AudioDevice {
                        id: format!("loopback_{}", idx),
                        name: format!("{} (Loopback)", name),
                        is_input: true,
                        is_default: false,
                        is_loopback: true,
                    });
                }
            }
        }
    }

    // Linux: PulseAudio monitor sources usually appear in input_devices() automatically.
    // macOS: ScreenCaptureKit or BlackHole. BlackHole appears in input_devices().

    devices
}
