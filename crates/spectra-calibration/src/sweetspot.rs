//! Sweet spot elimination algorithms.
//!
//! Traditional projection-based cinemas (IMAX, Dolby Cinema) have a "sweet spot"
//! where only 15-20% of seats in the center offer optimal viewing quality.
//! SPECTRA's self-emissive MicroLED technology inherently eliminates this by
//! maintaining uniform brightness and color at extreme viewing angles.
//!
//! This module quantifies and validates sweet spot elimination metrics.

use serde::{Deserialize, Serialize};

/// Seat position in the auditorium coordinate system.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SeatPosition {
    /// Horizontal angle from screen center in degrees (negative = left).
    pub horizontal_angle_deg: f64,
    /// Vertical angle from screen center in degrees (negative = below).
    pub vertical_angle_deg: f64,
    /// Distance from screen in meters.
    pub distance_m: f64,
}

/// Quality metrics for a single seat position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatQuality {
    pub position: SeatPosition,
    /// Luminance relative to center seat (1.0 = identical).
    pub relative_luminance: f64,
    /// Color shift as Delta E 2000 from reference.
    pub color_shift_delta_e: f64,
    /// Contrast ratio at this viewing angle.
    pub contrast_ratio: f64,
    /// Overall quality score (0.0 to 1.0).
    pub quality_score: f64,
}

/// Validates sweet spot elimination across an auditorium.
pub struct SweetSpotValidator {
    /// Minimum acceptable quality score (0.0 to 1.0).
    min_quality_threshold: f64,
    /// Maximum acceptable Delta E color shift.
    max_delta_e: f64,
}

impl SweetSpotValidator {
    /// Create validator with Sylvain reference standards.
    pub fn sylvain_reference() -> Self {
        Self {
            min_quality_threshold: 0.95,
            max_delta_e: 1.5, // Imperceptible to trained observers
        }
    }

    /// Evaluate quality at a given seat position for a MicroLED display.
    ///
    /// MicroLED self-emissive displays maintain >95% luminance uniformity
    /// at angles up to ±85°, compared to projection systems that degrade
    /// significantly beyond ±15°.
    pub fn evaluate_seat(&self, seat: &SeatPosition) -> SeatQuality {
        let angle = (seat.horizontal_angle_deg.powi(2) + seat.vertical_angle_deg.powi(2)).sqrt();

        // MicroLED angular luminance model (Lambertian-like)
        // Self-emissive displays follow approximately cosine law
        // but MicroLED maintains >95% to ±85°
        let relative_luminance = if angle <= 85.0 {
            1.0 - (angle / 85.0).powi(2) * 0.05
        } else {
            0.90 * (1.0 - ((angle - 85.0) / 10.0).min(1.0))
        };

        // Color shift model for MicroLED (minimal compared to projection)
        let color_shift = angle * 0.015; // ~1.3 Delta E at 85°

        // Contrast maintained with self-emissive (true black at any angle)
        let contrast_ratio = 1_000_000.0 * relative_luminance;

        let quality_score = self.compute_quality_score(relative_luminance, color_shift);

        SeatQuality {
            position: *seat,
            relative_luminance,
            color_shift_delta_e: color_shift,
            contrast_ratio,
            quality_score,
        }
    }

    fn compute_quality_score(&self, luminance: f64, delta_e: f64) -> f64 {
        let lum_score = luminance.min(1.0);
        let color_score = (1.0 - delta_e / 5.0).max(0.0);
        (lum_score * 0.6 + color_score * 0.4).clamp(0.0, 1.0)
    }

    /// Validate that all seats in the auditorium meet quality thresholds.
    pub fn validate_auditorium(&self, seats: &[SeatPosition]) -> ValidationResult {
        let qualities: Vec<SeatQuality> = seats.iter().map(|s| self.evaluate_seat(s)).collect();

        let passing = qualities
            .iter()
            .filter(|q| q.quality_score >= self.min_quality_threshold)
            .count();

        let sweet_spot_percentage = (passing as f64 / seats.len() as f64) * 100.0;

        ValidationResult {
            total_seats: seats.len(),
            passing_seats: passing,
            sweet_spot_percentage,
            min_quality: qualities
                .iter()
                .map(|q| q.quality_score)
                .fold(f64::MAX, f64::min),
            avg_quality: qualities.iter().map(|q| q.quality_score).sum::<f64>()
                / seats.len() as f64,
        }
    }
}

/// Result of auditorium validation.
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub total_seats: usize,
    pub passing_seats: usize,
    /// Percentage of seats in the "sweet spot" (meeting quality threshold).
    /// SPECTRA target: >98% (vs. IMAX ~20%, Dolby ~25%)
    pub sweet_spot_percentage: f64,
    pub min_quality: f64,
    pub avg_quality: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_seat_perfect() {
        let validator = SweetSpotValidator::sylvain_reference();
        let center = SeatPosition {
            horizontal_angle_deg: 0.0,
            vertical_angle_deg: 0.0,
            distance_m: 15.0,
        };
        let quality = validator.evaluate_seat(&center);
        assert!(quality.quality_score > 0.99);
        assert!(quality.relative_luminance > 0.99);
    }

    #[test]
    fn test_extreme_angle_still_good() {
        let validator = SweetSpotValidator::sylvain_reference();
        let edge = SeatPosition {
            horizontal_angle_deg: 70.0,
            vertical_angle_deg: 20.0,
            distance_m: 12.0,
        };
        let quality = validator.evaluate_seat(&edge);
        // Even at extreme angles, MicroLED maintains high quality
        assert!(quality.quality_score > 0.90);
    }
}
