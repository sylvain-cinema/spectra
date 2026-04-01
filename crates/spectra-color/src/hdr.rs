//! HDR tone mapping using SMPTE ST 2084 (PQ) and HLG transfer functions.
//!
//! SPECTRA uses PQ as the native HDR format, supporting up to 10,000 nits
//! peak brightness — matching the full capability of MicroLED panels.

/// SMPTE ST 2084 (PQ) constants.
pub mod pq {
    pub const M1: f64 = 0.1593017578125;
    pub const M2: f64 = 78.84375;
    pub const C1: f64 = 0.8359375;
    pub const C2: f64 = 18.8515625;
    pub const C3: f64 = 18.6875;
    /// PQ absolute peak luminance in cd/m²
    pub const L_MAX: f64 = 10_000.0;
}

/// Apply the PQ Electro-Optical Transfer Function (EOTF).
/// Converts PQ-encoded signal [0, 1] to linear luminance [0, 10000] cd/m².
pub fn pq_eotf(signal: f64) -> f64 {
    let signal = signal.clamp(0.0, 1.0);
    let e_pow = signal.powf(1.0 / pq::M2);
    let numerator = (e_pow - pq::C1).max(0.0);
    let denominator = pq::C2 - pq::C3 * e_pow;
    pq::L_MAX * (numerator / denominator).powf(1.0 / pq::M1)
}

/// Apply the PQ Opto-Electronic Transfer Function (OETF).
/// Converts linear luminance [0, 10000] cd/m² to PQ signal [0, 1].
pub fn pq_oetf(luminance: f64) -> f64 {
    let y = (luminance / pq::L_MAX).clamp(0.0, 1.0);
    let y_pow = y.powf(pq::M1);
    let numerator = pq::C1 + pq::C2 * y_pow;
    let denominator = 1.0 + pq::C3 * y_pow;
    (numerator / denominator).powf(pq::M2)
}

/// Apply the HLG (Hybrid Log-Gamma) OETF.
pub fn hlg_oetf(luminance: f64) -> f64 {
    let l = luminance.clamp(0.0, 1.0);
    if l <= 1.0 / 12.0 {
        (3.0 * l).sqrt()
    } else {
        let a = 0.17883277;
        let b = 1.0 - 4.0 * a;
        let c = 0.5 - a * (4.0 * a).ln();
        a * (12.0 * l - b).ln() + c
    }
}

/// Tone mapping operator for mapping HDR content to display capabilities.
pub struct ToneMapper {
    /// Target peak brightness in nits.
    target_peak_nits: f64,
    /// Source content peak brightness in nits.
    source_peak_nits: f64,
}

impl ToneMapper {
    pub fn new(source_peak_nits: f64, target_peak_nits: f64) -> Self {
        Self {
            target_peak_nits,
            source_peak_nits,
        }
    }

    /// Apply tone mapping to a linear luminance value.
    /// Uses a simple Reinhard-style operator with peak matching.
    pub fn map(&self, luminance: f64) -> f64 {
        let normalized = luminance / self.source_peak_nits;
        let mapped = normalized / (1.0 + normalized);
        mapped * self.target_peak_nits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pq_roundtrip() {
        let luminance = 1000.0; // 1000 nits
        let encoded = pq_oetf(luminance);
        let decoded = pq_eotf(encoded);
        assert!((decoded - luminance).abs() < 1.0);
    }

    #[test]
    fn test_pq_black() {
        assert!((pq_eotf(0.0)).abs() < 1e-6);
    }

    #[test]
    fn test_tone_mapper() {
        let mapper = ToneMapper::new(4000.0, 10000.0);
        let result = mapper.map(2000.0);
        assert!(result > 0.0);
        assert!(result <= 10000.0);
    }
}
