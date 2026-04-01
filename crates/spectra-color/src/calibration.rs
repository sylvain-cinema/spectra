//! Per-panel color calibration for uniform display output.
//!
//! MicroLED panels exhibit slight manufacturing variations. This module
//! provides per-panel calibration to achieve display-wide uniformity
//! critical to SPECTRA's "every seat is the best seat" promise.

/// Per-panel calibration data.
#[derive(Debug, Clone)]
pub struct PanelCalibration {
    /// Panel identifier within the tiled display.
    pub panel_id: u32,
    /// White point offset from target (delta x, delta y in CIE 1931).
    pub white_point_offset: (f64, f64),
    /// Per-channel gain correction factors [R, G, B].
    pub gain_correction: [f64; 3],
    /// Per-channel gamma correction.
    pub gamma_correction: [f64; 3],
    /// Brightness uniformity map (normalized 0.0-1.0 per zone).
    pub uniformity_map: Vec<f64>,
}

impl PanelCalibration {
    /// Apply calibration correction to an RGB triplet.
    pub fn correct(&self, r: f64, g: f64, b: f64) -> (f64, f64, f64) {
        let r_corrected = (r * self.gain_correction[0])
            .powf(self.gamma_correction[0])
            .clamp(0.0, 1.0);
        let g_corrected = (g * self.gain_correction[1])
            .powf(self.gamma_correction[1])
            .clamp(0.0, 1.0);
        let b_corrected = (b * self.gain_correction[2])
            .powf(self.gamma_correction[2])
            .clamp(0.0, 1.0);
        (r_corrected, g_corrected, b_corrected)
    }
}

/// Calibration profile for the entire tiled display.
pub struct DisplayCalibrationProfile {
    pub panels: Vec<PanelCalibration>,
    /// Target white point in CIE 1931 xy.
    pub target_white_point: (f64, f64),
    /// Target peak luminance in nits.
    pub target_peak_nits: f64,
}

impl DisplayCalibrationProfile {
    /// Create a default profile targeting D65 white point at 10,000 nits.
    pub fn default_cinema() -> Self {
        Self {
            panels: Vec::new(),
            target_white_point: (0.3127, 0.3290), // D65
            target_peak_nits: 10_000.0,
        }
    }
}
