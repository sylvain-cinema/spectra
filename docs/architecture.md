# SPECTRA Architecture

## Overview

SPECTRA is organized as a Rust workspace with four crates, each handling a distinct aspect of the MicroLED display pipeline.

## Crate Dependencies

```
spectra-core
├── spectra-color (color science, gamut mapping, HDR)
└── spectra-panel (hardware abstraction, tiling)

spectra-calibration (independent, used by spectra-core at runtime)
```

## Data Flow

1. **Ingest**: Content frames arrive from the content delivery system
2. **Color Science**: Gamut mapping (source → Rec.2020) and HDR tone mapping (PQ/HLG)
3. **Calibration**: Per-panel uniformity correction applied
4. **Panel Mapping**: Framebuffer regions distributed to tiled MicroLED panels
5. **Output**: Panel drivers write pixel data to hardware

## Integration Points

- **SENTIO**: Receives real-time commands to adjust tone mapping, brightness, and color emphasis based on narrative analysis
- **STRATUM**: Provides depth information for multi-plane compositing on the transparent display layer
- **Content Pipeline**: Receives pre-mastered content in Sylvain Certified format
