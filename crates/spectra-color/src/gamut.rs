//! Color gamut definitions and mapping for cinema display systems.

use nalgebra::Matrix3;

/// Supported color spaces in the SPECTRA pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    /// ITU-R BT.709 — standard HD
    Rec709,
    /// DCI-P3 — digital cinema standard
    DciP3,
    /// ITU-R BT.2020 — ultra-wide gamut (SPECTRA native)
    Rec2020,
    /// ACES AP0 — Academy Color Encoding System, scene-referred
    AcesAp0,
}

/// CIE 1931 xy chromaticity coordinates for color space primaries.
#[derive(Debug, Clone, Copy)]
pub struct Chromaticity {
    pub x: f64,
    pub y: f64,
}

/// Defines a color space by its RGB primaries and white point.
#[derive(Debug, Clone)]
pub struct ColorSpaceDef {
    pub name: &'static str,
    pub red: Chromaticity,
    pub green: Chromaticity,
    pub blue: Chromaticity,
    pub white: Chromaticity,
}

impl ColorSpace {
    /// Get the chromaticity definition for this color space.
    pub fn definition(&self) -> ColorSpaceDef {
        match self {
            ColorSpace::Rec709 => ColorSpaceDef {
                name: "Rec.709",
                red: Chromaticity { x: 0.640, y: 0.330 },
                green: Chromaticity { x: 0.300, y: 0.600 },
                blue: Chromaticity { x: 0.150, y: 0.060 },
                white: Chromaticity { x: 0.3127, y: 0.3290 }, // D65
            },
            ColorSpace::DciP3 => ColorSpaceDef {
                name: "DCI-P3",
                red: Chromaticity { x: 0.680, y: 0.320 },
                green: Chromaticity { x: 0.265, y: 0.690 },
                blue: Chromaticity { x: 0.150, y: 0.060 },
                white: Chromaticity { x: 0.3140, y: 0.3510 },
            },
            ColorSpace::Rec2020 => ColorSpaceDef {
                name: "Rec.2020",
                red: Chromaticity { x: 0.708, y: 0.292 },
                green: Chromaticity { x: 0.170, y: 0.797 },
                blue: Chromaticity { x: 0.131, y: 0.046 },
                white: Chromaticity { x: 0.3127, y: 0.3290 }, // D65
            },
            ColorSpace::AcesAp0 => ColorSpaceDef {
                name: "ACES AP0",
                red: Chromaticity { x: 0.7347, y: 0.2653 },
                green: Chromaticity { x: 0.0000, y: 1.0000 },
                blue: Chromaticity { x: 0.0001, y: -0.0770 },
                white: Chromaticity { x: 0.32168, y: 0.33767 },
            },
        }
    }

    /// Compute the 3x3 color space conversion matrix from this space to XYZ.
    pub fn to_xyz_matrix(&self) -> Matrix3<f64> {
        let def = self.definition();
        // Compute NPM (Normalized Primary Matrix) from chromaticity coordinates
        compute_npm(&def)
    }
}

/// Compute the Normalized Primary Matrix for a color space definition.
fn compute_npm(def: &ColorSpaceDef) -> Matrix3<f64> {
    let xr = def.red.x;
    let yr = def.red.y;
    let xg = def.green.x;
    let yg = def.green.y;
    let xb = def.blue.x;
    let yb = def.blue.y;

    // Build the XYZ matrix from primaries
    let m = Matrix3::new(
        xr / yr, xg / yg, xb / yb,
        1.0,     1.0,     1.0,
        (1.0 - xr - yr) / yr, (1.0 - xg - yg) / yg, (1.0 - xb - yb) / yb,
    );

    // White point XYZ
    let xw = def.white.x / def.white.y;
    let yw = 1.0;
    let zw = (1.0 - def.white.x - def.white.y) / def.white.y;
    let w = nalgebra::Vector3::new(xw, yw, zw);

    // Solve for scaling factors
    if let Some(m_inv) = m.try_inverse() {
        let s = m_inv * w;
        Matrix3::new(
            s[0] * m[(0, 0)], s[1] * m[(0, 1)], s[2] * m[(0, 2)],
            s[0] * m[(1, 0)], s[1] * m[(1, 1)], s[2] * m[(1, 2)],
            s[0] * m[(2, 0)], s[1] * m[(2, 1)], s[2] * m[(2, 2)],
        )
    } else {
        Matrix3::identity()
    }
}

/// Map colors from one gamut to another using perceptual intent.
pub struct GamutMapper {
    source: ColorSpace,
    target: ColorSpace,
    forward_matrix: Matrix3<f64>,
}

impl GamutMapper {
    pub fn new(source: ColorSpace, target: ColorSpace) -> Self {
        let src_to_xyz = source.to_xyz_matrix();
        let xyz_to_tgt = target
            .to_xyz_matrix()
            .try_inverse()
            .unwrap_or(Matrix3::identity());
        let forward_matrix = xyz_to_tgt * src_to_xyz;

        Self {
            source,
            target,
            forward_matrix,
        }
    }

    /// Map a linear RGB triplet from source to target color space.
    pub fn map(&self, r: f64, g: f64, b: f64) -> (f64, f64, f64) {
        let input = nalgebra::Vector3::new(r, g, b);
        let output = self.forward_matrix * input;
        (output[0], output[1], output[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rec709_primaries() {
        let def = ColorSpace::Rec709.definition();
        assert_eq!(def.name, "Rec.709");
        assert!((def.red.x - 0.640).abs() < 1e-6);
    }

    #[test]
    fn test_identity_mapping() {
        let mapper = GamutMapper::new(ColorSpace::Rec2020, ColorSpace::Rec2020);
        let (r, g, b) = mapper.map(0.5, 0.3, 0.8);
        assert!((r - 0.5).abs() < 1e-6);
        assert!((g - 0.3).abs() < 1e-6);
        assert!((b - 0.8).abs() < 1e-6);
    }
}
