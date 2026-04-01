//! Brightness uniformity correction across the tiled MicroLED display.

/// Uniformity measurement point on the display surface.
#[derive(Debug, Clone)]
pub struct UniformityPoint {
    pub x_normalized: f64,
    pub y_normalized: f64,
    pub measured_luminance: f64,
}

/// Generates a uniformity correction map for the display.
pub struct UniformityCorrector {
    target_luminance: f64,
    points: Vec<UniformityPoint>,
}

impl UniformityCorrector {
    pub fn new(target_luminance: f64) -> Self {
        Self {
            target_luminance,
            points: Vec::new(),
        }
    }

    pub fn add_measurement(&mut self, point: UniformityPoint) {
        self.points.push(point);
    }

    /// Compute correction factor at a given normalized position.
    pub fn correction_at(&self, x: f64, y: f64) -> f64 {
        if self.points.is_empty() {
            return 1.0;
        }
        // Inverse-distance weighted interpolation of correction factors
        let mut weight_sum = 0.0;
        let mut correction_sum = 0.0;
        for p in &self.points {
            let dist = ((p.x_normalized - x).powi(2) + (p.y_normalized - y).powi(2)).sqrt();
            let weight = 1.0 / (dist + 0.001);
            let correction = self.target_luminance / p.measured_luminance;
            weight_sum += weight;
            correction_sum += weight * correction;
        }
        (correction_sum / weight_sum).clamp(0.8, 1.2)
    }
}
