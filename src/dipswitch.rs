use crate::format_calculator;

/// Represents the 8 DIP switches
/// false = LEFT, true = RIGHT
#[derive(Debug, Clone)]
pub struct DipSwitch {
    switches: [bool; 8],
}

impl DipSwitch {
    /// Create a new DipSwitch with all switches in LEFT position (default)
    pub fn new() -> Self {
        Self {
            switches: [false; 8],
        }
    }

    /// Toggle switch at index (0-7, maps to switches 1-8)
    pub fn toggle(&mut self, index: usize) {
        if index < 8 {
            self.switches[index] = !self.switches[index];
        }
    }

    /// Reset all switches to default (LEFT) position
    pub fn reset(&mut self) {
        self.switches = [false; 8];
    }

    /// Get state of switch at index (0-7)
    pub fn get(&self, index: usize) -> bool {
        if index < 8 {
            self.switches[index]
        } else {
            false
        }
    }

    /// Calculate 8-bit bitmask where bit 0 = switch 1, bit 7 = switch 8
    /// Masked with 0b1111_1000 to show only video format related switches (4-8)
    pub fn get_bitmask(&self) -> u8 {
        let mut mask = 0u8;
        for (i, &switch) in self.switches.iter().enumerate() {
            if switch {
                mask |= 1 << i;
            }
        }
        mask & 0b1111_1000
    }

    /// Get bitmask as binary string (e.g., "0b00001101")
    pub fn get_bitmask_binary(&self) -> String {
        format!("0b{:08b}", self.get_bitmask())
    }

    /// Get bitmask as hex string (e.g., "0x0D")
    pub fn get_bitmask_hex(&self) -> String {
        format!("0x{:02X}", self.get_bitmask())
    }

    /// Calculate current output configuration
    pub fn get_config(&self) -> OutputConfig {
        // Switch indices (0-based)
        let sw1_sync_1_4 = self.switches[0];
        let sw2_sync_5_6 = self.switches[1];
        let sw3_sd_audio = self.switches[2];
        let sw4_format = self.switches[3];
        let sw5_hd_fmt = self.switches[4];
        let sw6_s1 = self.switches[5];
        let sw7_s2 = self.switches[6];
        let sw8_s3 = self.switches[7];

        // Calculate Group 1 format (outputs 1-4)
        let group1_format = format_calculator::calculate_group_format(
            sw1_sync_1_4,
            sw4_format,
            sw5_hd_fmt,
            sw6_s1,
            sw7_s2,
            sw8_s3,
        );

        // Calculate Group 2 format (outputs 5-6)
        let group2_format = format_calculator::calculate_group_format(
            sw2_sync_5_6,
            sw4_format,
            sw5_hd_fmt,
            sw6_s1,
            sw7_s2,
            sw8_s3,
        );

        // SD Output and Audio (both controlled by switch 3)
        let (sd_video, audio) = if sw3_sd_audio {
            (SdVideo::ColorBars75, AudioOutput::Tone)
        } else {
            (SdVideo::ColorBlack, AudioOutput::Silent)
        };

        OutputConfig {
            group1_format,
            group2_format,
            sd_video,
            audio,
        }
    }
}

impl Default for DipSwitch {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete output configuration based on DIP switch settings
#[derive(Debug, Clone)]
pub struct OutputConfig {
    pub group1_format: String,
    pub group2_format: String,
    pub sd_video: SdVideo,
    pub audio: AudioOutput,
}

/// SD video output options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdVideo {
    ColorBlack,
    ColorBars75,
}

impl SdVideo {
    pub fn as_str(&self) -> &str {
        match self {
            SdVideo::ColorBlack => "Color Black",
            SdVideo::ColorBars75 => "75% Color Bars",
        }
    }
}

/// Audio output options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioOutput {
    Silent,
    Tone,
}

impl AudioOutput {
    pub fn as_str(&self) -> &str {
        match self {
            AudioOutput::Silent => "Silent",
            AudioOutput::Tone => "Tone",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dipswitch() {
        let ds = DipSwitch::new();
        assert_eq!(ds.get_bitmask(), 0);
    }

    #[test]
    fn test_toggle() {
        let mut ds = DipSwitch::new();
        ds.toggle(0); // Toggle switch 1
        assert_eq!(ds.get(0), true);
        assert_eq!(ds.get_bitmask(), 0b00000000); // Masked out (not part of video format)
        
        ds.toggle(0); // Toggle back
        assert_eq!(ds.get(0), false);
        assert_eq!(ds.get_bitmask(), 0);
    }

    #[test]
    fn test_bitmask() {
        let mut ds = DipSwitch::new();
        ds.toggle(0); // Switch 1 = bit 0 (masked out)
        ds.toggle(2); // Switch 3 = bit 2 (masked out)
        ds.toggle(3); // Switch 4 = bit 3 (included)
        
        assert_eq!(ds.get_bitmask(), 0b00001000); // Only switch 4 counted
        assert_eq!(ds.get_bitmask_binary(), "0b00001000");
        assert_eq!(ds.get_bitmask_hex(), "0x08");
    }

    #[test]
    fn test_bitmask_video_format_only() {
        let mut ds = DipSwitch::new();
        ds.toggle(3); // Switch 4 = bit 3 (FORMAT)
        ds.toggle(4); // Switch 5 = bit 4 (HD FMT)
        ds.toggle(5); // Switch 6 = bit 5 (S1)
        ds.toggle(6); // Switch 7 = bit 6 (S2)
        ds.toggle(7); // Switch 8 = bit 7 (S3)
        
        // All video format switches (4-8) = bits 3-7 = 0b11111000
        assert_eq!(ds.get_bitmask(), 0b11111000);
        assert_eq!(ds.get_bitmask_hex(), "0xF8");
    }

    #[test]
    fn test_reset() {
        let mut ds = DipSwitch::new();
        ds.toggle(0);
        ds.toggle(2);
        ds.toggle(5);
        
        assert_ne!(ds.get_bitmask(), 0);
        
        ds.reset();
        assert_eq!(ds.get_bitmask(), 0);
    }
}
