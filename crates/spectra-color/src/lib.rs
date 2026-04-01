//! # SPECTRA Color Science
//!
//! Color space management, gamut mapping, and HDR tone mapping for the
//! SPECTRA display system. Supports Rec.709, DCI-P3, and Rec.2020+ color spaces
//! with perceptual gamut mapping to maximize visible color volume.

pub mod calibration;
pub mod gamut;
pub mod hdr;
