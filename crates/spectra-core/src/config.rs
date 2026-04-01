//! Display configuration for the SPECTRA rendering pipeline.

use serde::{Deserialize, Serialize};

/// Display resolution presets for Sylvain venue tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Resolution {
    /// 8K × 4K — Sylvain Étoilée tier (mass premium)
    UHD_8K,
    /// 16K × 8K — Sylvain Visionnaire tier (flagship luxury)
    UHD_16K,
    /// Custom resolution for SANCTUM private installations
    Custom { width: u32, height: u32 },
}

impl Resolution {
    /// Returns the pixel dimensions (width, height).
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Resolution::UHD_8K => (7680, 4320),
            Resolution::UHD_16K => (15360, 8640),
            Resolution::Custom { width, height } => (*width, *height),
        }
    }

    /// Total pixel count.
    pub fn pixel_count(&self) -> u64 {
        let (w, h) = self.dimensions();
        w as u64 * h as u64
    }
}

/// HDR transfer function mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HdrMode {
    /// Perceptual Quantizer (SMPTE ST 2084) — cinema standard
    PQ,
    /// Hybrid Log-Gamma — broadcast compatible
    HLG,
    /// Standard Dynamic Range fallback
    SDR,
}

/// Master configuration for a SPECTRA display pipeline instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub resolution: Resolution,
    pub hdr_mode: HdrMode,
    pub peak_brightness_nits: f64,
    pub refresh_rate_hz: u32,
    pub color_depth_bits: u8,
    pub panel_count: u32,
    /// Enable SENTIO AI integration for dynamic tone mapping
    pub sentio_integration: bool,
}

impl DisplayConfig {
    pub fn builder() -> DisplayConfigBuilder {
        DisplayConfigBuilder::default()
    }

    /// Returns the required bandwidth in gigabits per second.
    pub fn required_bandwidth_gbps(&self) -> f64 {
        let pixels = self.resolution.pixel_count() as f64;
        let bits_per_pixel = self.color_depth_bits as f64 * 3.0; // RGB
        let frames = self.refresh_rate_hz as f64;
        (pixels * bits_per_pixel * frames) / 1_000_000_000.0
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            resolution: Resolution::UHD_16K,
            hdr_mode: HdrMode::PQ,
            peak_brightness_nits: 10_000.0,
            refresh_rate_hz: 120,
            color_depth_bits: 12,
            panel_count: 1,
            sentio_integration: true,
        }
    }
}

/// Builder for `DisplayConfig`.
#[derive(Default)]
pub struct DisplayConfigBuilder {
    resolution: Option<Resolution>,
    hdr_mode: Option<HdrMode>,
    peak_brightness: Option<f64>,
    refresh_rate: Option<u32>,
    color_depth: Option<u8>,
    panel_count: Option<u32>,
    sentio: Option<bool>,
}

impl DisplayConfigBuilder {
    pub fn resolution(mut self, res: Resolution) -> Self {
        self.resolution = Some(res);
        self
    }

    pub fn hdr_mode(mut self, mode: HdrMode) -> Self {
        self.hdr_mode = Some(mode);
        self
    }

    pub fn peak_brightness(mut self, nits: f64) -> Self {
        self.peak_brightness = Some(nits);
        self
    }

    pub fn refresh_rate(mut self, hz: u32) -> Self {
        self.refresh_rate = Some(hz);
        self
    }

    pub fn panel_count(mut self, count: u32) -> Self {
        self.panel_count = Some(count);
        self
    }

    pub fn build(self) -> DisplayConfig {
        let default = DisplayConfig::default();
        DisplayConfig {
            resolution: self.resolution.unwrap_or(default.resolution),
            hdr_mode: self.hdr_mode.unwrap_or(default.hdr_mode),
            peak_brightness_nits: self.peak_brightness.unwrap_or(default.peak_brightness_nits),
            refresh_rate_hz: self.refresh_rate.unwrap_or(default.refresh_rate_hz),
            color_depth_bits: self.color_depth.unwrap_or(default.color_depth_bits),
            panel_count: self.panel_count.unwrap_or(default.panel_count),
            sentio_integration: self.sentio.unwrap_or(default.sentio_integration),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16k_resolution() {
        let res = Resolution::UHD_16K;
        assert_eq!(res.dimensions(), (15360, 8640));
        assert_eq!(res.pixel_count(), 132_710_400);
    }

    #[test]
    fn test_bandwidth_calculation() {
        let config = DisplayConfig::default();
        let bw = config.required_bandwidth_gbps();
        // 16K @ 120Hz @ 12-bit RGB = ~572 Gbps
        assert!(bw > 500.0);
    }
}
