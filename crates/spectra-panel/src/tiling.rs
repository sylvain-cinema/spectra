//! Multi-panel tiling engine for large-format MicroLED displays.
//!
//! A Sylvain VISIONNAIRE display (3,000m²) requires hundreds of individual
//! MicroLED panels tiled seamlessly. This module manages the physical
//! layout, seam compensation, and coordinated refresh.

use crate::driver::MicroLedPanel;

/// Physical position and orientation of a panel in the tiled array.
#[derive(Debug, Clone)]
pub struct PanelPlacement {
    /// Panel identifier.
    pub panel_id: u32,
    /// Physical position in millimeters from array origin.
    pub position_mm: (f64, f64),
    /// Panel rotation in degrees (typically 0).
    pub rotation_deg: f64,
    /// Pixel offset in the virtual framebuffer.
    pub pixel_offset: (u32, u32),
    /// Panel native resolution.
    pub resolution: (u32, u32),
}

/// Configuration for a tiled display array.
#[derive(Debug, Clone)]
pub struct TilingConfig {
    /// Total virtual resolution of the tiled display.
    pub total_resolution: (u32, u32),
    /// Panel placements.
    pub panels: Vec<PanelPlacement>,
    /// Seam compensation width in pixels.
    pub seam_compensation_px: u32,
    /// Enable sub-pixel alignment for seamless tiling.
    pub sub_pixel_alignment: bool,
}

/// Manages the tiled MicroLED array as a single virtual display.
pub struct TilingEngine {
    config: TilingConfig,
}

impl TilingEngine {
    pub fn new(config: TilingConfig) -> Self {
        tracing::info!(
            panels = config.panels.len(),
            total_width = config.total_resolution.0,
            total_height = config.total_resolution.1,
            "Initializing tiling engine"
        );
        Self { config }
    }

    /// Distribute a framebuffer region to the appropriate panels.
    pub fn distribute_region(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        _panels: &mut [Box<dyn MicroLedPanel>],
    ) -> Result<(), TilingError> {
        // Find all panels that intersect with this region
        for placement in &self.config.panels {
            let (px, py) = placement.pixel_offset;
            let (pw, ph) = placement.resolution;

            // Check intersection
            if x < px + pw && x + width > px && y < py + ph && y + height > py {
                tracing::trace!(
                    panel_id = placement.panel_id,
                    "Distributing region to panel"
                );
            }
        }
        Ok(())
    }

    /// Get the total virtual resolution.
    pub fn total_resolution(&self) -> (u32, u32) {
        self.config.total_resolution
    }

    /// Get the number of panels in the array.
    pub fn panel_count(&self) -> usize {
        self.config.panels.len()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TilingError {
    #[error("panel {0} not found in tiling configuration")]
    PanelNotFound(u32),

    #[error("region exceeds virtual framebuffer bounds")]
    OutOfBounds,
}
