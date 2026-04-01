//! Display pipeline orchestrator.
//!
//! The `DisplayPipeline` manages the end-to-end flow from content ingest
//! through color science processing to MicroLED panel output.

use crate::config::DisplayConfig;
use crate::framebuffer::Framebuffer;

/// Pipeline processing stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineStage {
    /// Content ingest and decode
    Ingest,
    /// Color space conversion and gamut mapping
    ColorScience,
    /// HDR tone mapping (PQ/HLG)
    ToneMapping,
    /// Panel-level pixel mapping and tiling
    PanelMapping,
    /// Final output to MicroLED drivers
    Output,
}

/// The main display pipeline orchestrating all rendering stages.
pub struct DisplayPipeline {
    config: DisplayConfig,
    framebuffer: Framebuffer,
    current_stage: PipelineStage,
    running: bool,
}

impl DisplayPipeline {
    /// Create a new display pipeline with the given configuration.
    pub fn new(config: DisplayConfig) -> Result<Self, PipelineError> {
        let (width, height) = config.resolution.dimensions();
        let framebuffer = Framebuffer::new(width, height, config.color_depth_bits)?;

        Ok(Self {
            config,
            framebuffer,
            current_stage: PipelineStage::Ingest,
            running: false,
        })
    }

    /// Start the pipeline processing loop.
    pub fn start(&mut self) -> Result<(), PipelineError> {
        if self.running {
            return Err(PipelineError::AlreadyRunning);
        }

        tracing::info!(
            resolution = ?self.config.resolution,
            hdr = ?self.config.hdr_mode,
            brightness = self.config.peak_brightness_nits,
            "Starting SPECTRA display pipeline"
        );

        self.running = true;
        self.current_stage = PipelineStage::Ingest;
        Ok(())
    }

    /// Stop the pipeline.
    pub fn stop(&mut self) {
        self.running = false;
        tracing::info!("SPECTRA display pipeline stopped");
    }

    /// Process a single frame through all pipeline stages.
    pub fn process_frame(&mut self) -> Result<FrameResult, PipelineError> {
        if !self.running {
            return Err(PipelineError::NotRunning);
        }

        // Stage 1: Ingest
        self.current_stage = PipelineStage::Ingest;
        self.framebuffer.clear();

        // Stage 2: Color Science
        self.current_stage = PipelineStage::ColorScience;
        // Color gamut mapping handled by spectra-color

        // Stage 3: Tone Mapping
        self.current_stage = PipelineStage::ToneMapping;
        // HDR processing handled by spectra-color

        // Stage 4: Panel Mapping
        self.current_stage = PipelineStage::PanelMapping;
        // Tiling handled by spectra-panel

        // Stage 5: Output
        self.current_stage = PipelineStage::Output;

        Ok(FrameResult {
            frame_number: 0, // TODO: frame counter
            stage: self.current_stage,
            latency_us: 0,   // TODO: actual timing
        })
    }

    /// Get current pipeline stage.
    pub fn current_stage(&self) -> PipelineStage {
        self.current_stage
    }

    /// Check if pipeline is running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get a reference to the active framebuffer.
    pub fn framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }
}

/// Result of processing a single frame.
#[derive(Debug)]
pub struct FrameResult {
    pub frame_number: u64,
    pub stage: PipelineStage,
    pub latency_us: u64,
}

/// Pipeline errors.
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("display pipeline is already running")]
    AlreadyRunning,

    #[error("display pipeline is not running")]
    NotRunning,

    #[error("framebuffer allocation failed: {0}")]
    FramebufferError(String),

    #[error("panel communication error: {0}")]
    PanelError(String),
}
