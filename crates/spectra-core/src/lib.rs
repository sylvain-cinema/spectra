//! # SPECTRA Core
//!
//! The display pipeline orchestrator for Sylvain's 16K MicroLED display system.
//! Manages the end-to-end rendering pipeline from content ingest through to
//! MicroLED panel output, coordinating color science, HDR tone mapping,
//! and panel-level control.

pub mod config;
pub mod framebuffer;
pub mod pipeline;

pub use config::{DisplayConfig, HdrMode, Resolution};
pub use pipeline::DisplayPipeline;
