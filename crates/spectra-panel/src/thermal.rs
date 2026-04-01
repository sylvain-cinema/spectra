//! Thermal management for MicroLED panels.
//!
//! High-brightness MicroLED operation generates significant heat.
//! This module monitors panel temperatures and implements protective
//! measures to prevent damage while maintaining visual quality.

/// Thermal zone classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalZone {
    /// Normal operating temperature (<60°C)
    Normal,
    /// Elevated but safe (<75°C) — enable active cooling
    Warm,
    /// Approaching limits (<85°C) — reduce brightness
    Hot,
    /// Critical (>85°C) — emergency shutdown
    Critical,
}

/// Thermal management policy.
pub struct ThermalManager {
    /// Temperature threshold for each zone in Celsius.
    warm_threshold: f64,
    hot_threshold: f64,
    critical_threshold: f64,
}

impl ThermalManager {
    pub fn new() -> Self {
        Self {
            warm_threshold: 60.0,
            hot_threshold: 75.0,
            critical_threshold: 85.0,
        }
    }

    /// Classify the current temperature into a thermal zone.
    pub fn classify(&self, temp_celsius: f64) -> ThermalZone {
        if temp_celsius >= self.critical_threshold {
            ThermalZone::Critical
        } else if temp_celsius >= self.hot_threshold {
            ThermalZone::Hot
        } else if temp_celsius >= self.warm_threshold {
            ThermalZone::Warm
        } else {
            ThermalZone::Normal
        }
    }

    /// Calculate brightness reduction factor for thermal protection.
    /// Returns 1.0 for normal, reduced values for hot/critical.
    pub fn brightness_factor(&self, temp_celsius: f64) -> f64 {
        match self.classify(temp_celsius) {
            ThermalZone::Normal | ThermalZone::Warm => 1.0,
            ThermalZone::Hot => {
                let overshoot = temp_celsius - self.hot_threshold;
                let range = self.critical_threshold - self.hot_threshold;
                1.0 - (overshoot / range) * 0.3 // Reduce up to 30%
            }
            ThermalZone::Critical => 0.5, // Emergency 50% reduction
        }
    }
}

impl Default for ThermalManager {
    fn default() -> Self {
        Self::new()
    }
}
