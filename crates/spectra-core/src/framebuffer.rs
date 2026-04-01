//! 16K framebuffer management.
//!
//! Manages the high-resolution framebuffer required for SPECTRA's 16K MicroLED
//! output. Supports multi-plane compositing for STRATUM integration and
//! efficient memory management for real-time rendering.

use crate::pipeline::PipelineError;

/// A single pixel in the SPECTRA framebuffer.
/// Uses 16-bit per channel for internal processing (48-bit RGB).
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Pixel {
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl Pixel {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self {
        r: u16::MAX,
        g: u16::MAX,
        b: u16::MAX,
    };

    /// Convert from normalized floating-point [0.0, 1.0] values.
    pub fn from_float(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: (r.clamp(0.0, 1.0) * u16::MAX as f32) as u16,
            g: (g.clamp(0.0, 1.0) * u16::MAX as f32) as u16,
            b: (b.clamp(0.0, 1.0) * u16::MAX as f32) as u16,
        }
    }

    /// Convert to normalized floating-point values.
    pub fn to_float(&self) -> (f32, f32, f32) {
        (
            self.r as f32 / u16::MAX as f32,
            self.g as f32 / u16::MAX as f32,
            self.b as f32 / u16::MAX as f32,
        )
    }
}

/// High-resolution framebuffer for SPECTRA display output.
pub struct Framebuffer {
    width: u32,
    height: u32,
    color_depth: u8,
    /// Pixel data stored in row-major order.
    /// For 16K (15360 × 8640), this is ~796 MB at 48-bit/pixel.
    data: Vec<Pixel>,
}

impl Framebuffer {
    /// Allocate a new framebuffer.
    pub fn new(width: u32, height: u32, color_depth: u8) -> Result<Self, PipelineError> {
        let pixel_count = width as usize * height as usize;
        if pixel_count == 0 {
            return Err(PipelineError::FramebufferError(
                "zero-size framebuffer".into(),
            ));
        }

        tracing::info!(
            width,
            height,
            color_depth,
            size_mb = (pixel_count * std::mem::size_of::<Pixel>()) / (1024 * 1024),
            "Allocating SPECTRA framebuffer"
        );

        Ok(Self {
            width,
            height,
            color_depth,
            data: vec![Pixel::BLACK; pixel_count],
        })
    }

    /// Clear the framebuffer to black.
    pub fn clear(&mut self) {
        self.data.fill(Pixel::BLACK);
    }

    /// Get a pixel at (x, y).
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&Pixel> {
        if x < self.width && y < self.height {
            Some(&self.data[(y as usize * self.width as usize) + x as usize])
        } else {
            None
        }
    }

    /// Set a pixel at (x, y).
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        if x < self.width && y < self.height {
            self.data[(y as usize * self.width as usize) + x as usize] = pixel;
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn color_depth(&self) -> u8 {
        self.color_depth
    }

    /// Raw pixel data slice.
    pub fn data(&self) -> &[Pixel] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_conversion() {
        let p = Pixel::from_float(0.5, 0.0, 1.0);
        let (r, g, b) = p.to_float();
        assert!((r - 0.5).abs() < 0.001);
        assert!((g - 0.0).abs() < 0.001);
        assert!((b - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_framebuffer_small() {
        let fb = Framebuffer::new(1920, 1080, 12).unwrap();
        assert_eq!(fb.width(), 1920);
        assert_eq!(fb.height(), 1080);
    }
}
