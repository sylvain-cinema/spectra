//! MicroLED panel hardware abstraction.
//!
//! Provides a trait-based interface for communicating with MicroLED
//! panel controllers, abstracting over different hardware vendors.

/// Panel connection status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelStatus {
    Connected,
    Disconnected,
    Error,
    Calibrating,
}

/// Hardware abstraction trait for MicroLED panels.
pub trait MicroLedPanel: Send + Sync {
    /// Get the panel's unique identifier.
    fn panel_id(&self) -> u32;

    /// Get the panel's native resolution.
    fn native_resolution(&self) -> (u32, u32);

    /// Get the panel's current status.
    fn status(&self) -> PanelStatus;

    /// Send a framebuffer region to the panel.
    fn write_region(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        data: &[u8],
    ) -> Result<(), PanelDriverError>;

    /// Set the panel brightness (0.0 to 1.0).
    fn set_brightness(&mut self, level: f64) -> Result<(), PanelDriverError>;

    /// Get the current panel temperature in Celsius.
    fn temperature_celsius(&self) -> f64;

    /// Perform panel self-test.
    fn self_test(&mut self) -> Result<PanelDiagnostics, PanelDriverError>;
}

/// Diagnostics from a panel self-test.
#[derive(Debug, Clone)]
pub struct PanelDiagnostics {
    pub dead_pixels: u32,
    pub temperature_celsius: f64,
    pub brightness_uniformity: f64,
    pub color_accuracy_delta_e: f64,
    pub firmware_version: String,
}

/// Panel driver errors.
#[derive(Debug, thiserror::Error)]
pub enum PanelDriverError {
    #[error("panel not connected: panel_id={0}")]
    NotConnected(u32),

    #[error("communication timeout after {0}ms")]
    Timeout(u64),

    #[error("invalid framebuffer region: ({x},{y}) {width}x{height}")]
    InvalidRegion {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },

    #[error("panel overtemperature: {0}°C")]
    OverTemperature(f64),

    #[error("hardware error: {0}")]
    Hardware(String),
}
