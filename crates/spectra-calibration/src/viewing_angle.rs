//! Viewing angle response compensation.
//!
//! Compensates for the natural angular response of MicroLED emitters
//! to ensure consistent perceived brightness and color across all
//! auditorium seating positions.

/// Angular response model for a MicroLED emitter.
pub struct AngularResponse {
    /// Half-angle at which luminance drops to 50% (degrees).
    pub half_brightness_angle: f64,
}

impl AngularResponse {
    /// Standard MicroLED emitter response.
    pub fn standard_microled() -> Self {
        Self {
            half_brightness_angle: 85.0,
        }
    }

    /// Calculate relative luminance at a given viewing angle.
    pub fn luminance_at_angle(&self, angle_deg: f64) -> f64 {
        let angle_rad = angle_deg.abs().to_radians();
        // Modified Lambertian model for MicroLED
        let cos_factor = angle_rad.cos();
        let correction = 1.0 + 0.1 * (1.0 - cos_factor); // Slight super-Lambertian boost
        (cos_factor * correction).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_axis_full_brightness() {
        let ar = AngularResponse::standard_microled();
        let lum = ar.luminance_at_angle(0.0);
        assert!((lum - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_moderate_angle_high_brightness() {
        let ar = AngularResponse::standard_microled();
        let lum = ar.luminance_at_angle(45.0);
        assert!(lum > 0.70);
    }
}
