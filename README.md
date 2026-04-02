<div align="center">

<img src="https://img.shields.io/badge/SYLVAIN-SPECTRA-000000?style=for-the-badge&labelColor=f59e0b&color=000000" alt="SPECTRA" height="40"/>

### The Living Canvas

**16K MicroLED Display Engine**

<br/>

[![CI](https://github.com/sylvain-cinema/spectra/actions/workflows/ci.yml/badge.svg)](https://github.com/sylvain-cinema/spectra/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-f59e0b?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024-dea584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.11+-3776AB?style=flat-square&logo=python&logoColor=white)](https://python.org)
[![Docs](https://img.shields.io/badge/docs-sylvain.github.io-f97316?style=flat-square)](https://sylvain-cinema.github.io)

<br/>

*A morphable 16K self-emissive MicroLED display that eliminates the cinema sweet spot problem.*
*Consistent brightness and color fidelity across a near-180° viewing cone.*

<br/>

**Every Seat is the Best Seat.**

</div>

<br/>

---

<br/>

## Overview

SPECTRA is Sylvain's proprietary display engine — a self-emissive MicroLED system that fundamentally solves the viewing angle problem plaguing cinema for over a century. Unlike projection-based systems (IMAX, Dolby Cinema) where only 15–20% of seats offer optimal viewing, SPECTRA delivers reference-quality imagery to **every seat in the auditorium**.

<br/>

## Key Specifications

<table>
<tr><td><strong>Resolution</strong></td><td>16K × 16K (268 million pixels)</td></tr>
<tr><td><strong>Peak Brightness</strong></td><td>12,000 nits (4× brighter than Dolby Vision)</td></tr>
<tr><td><strong>Contrast Ratio</strong></td><td>∞ : 1 (true black, pixel-level control)</td></tr>
<tr><td><strong>Viewing Angle</strong></td><td>178° uniform luminance</td></tr>
<tr><td><strong>Color Gamut</strong></td><td>100% Rec.2020 coverage</td></tr>
<tr><td><strong>HDR Format</strong></td><td>PQ (SMPTE ST 2084) / HLG</td></tr>
<tr><td><strong>Refresh Rate</strong></td><td>240 Hz native</td></tr>
<tr><td><strong>Pixel Pitch</strong></td><td>Sub-millimeter (venue-dependent)</td></tr>
<tr><td><strong>Panel Lifetime</strong></td><td>100,000+ hours</td></tr>
</table>

<br/>

## Architecture

```mermaid
flowchart LR
    subgraph INPUT["🎬 Content Ingest"]
        direction TB
        SRC["Source Stream\n4K / 8K / 16K"]
        DEC["Decoder\nHEVC · AV1 · ProRes"]
        SRC --> DEC
    end

    subgraph COLOR["🎨 spectra-color"]
        direction TB
        GAM["Gamut Mapping\nRec.709 → Rec.2020"]
        HDR["HDR Tone Mapping\nPQ · ST 2084"]
        CAL_C["Per-Panel\nColor Calibration"]
        GAM --> HDR --> CAL_C
    end

    subgraph CORE["⚡ spectra-core"]
        direction TB
        FB["16K Framebuffer\n268M pixels · 48-bit RGB"]
        PIPE["Pipeline Orchestrator\n240 Hz render loop"]
        FB --> PIPE
    end

    subgraph PANEL["🖥️ spectra-panel"]
        direction TB
        TILE["Tiling Engine\nMulti-panel seam compensation"]
        DRV["MicroLED Drivers\nHardware abstraction"]
        THERM["Thermal Manager\nReal-time monitoring"]
        TILE --> DRV
        TILE --> THERM
    end

    subgraph CALIB["🔧 spectra-calibration"]
        direction TB
        SS["Sweet Spot Eliminator\n178° uniform quality"]
        UNI["Uniformity Corrector\nBrightness · Color"]
        ANG["Angle Compensator\nAngular response model"]
        SS --> UNI --> ANG
    end

    subgraph OUTPUT["✨ Display Output"]
        LED["MicroLED Array\n12,000 nits · ∞:1 contrast"]
    end

    subgraph SENTIO_IN["🧠 SENTIO"]
        CMD["Narrative Commands\nBrightness · Color temp · Emphasis"]
    end

    DEC --> GAM
    CAL_C --> FB
    PIPE --> TILE
    DRV --> LED
    ANG --> CAL_C
    ANG --> DRV
    CMD -.->|real-time| HDR
    CMD -.->|real-time| PIPE

    style INPUT fill:#1a1a2e,stroke:#f59e0b,color:#fff
    style COLOR fill:#1a1a2e,stroke:#f59e0b,color:#fff
    style CORE fill:#1a1a2e,stroke:#f97316,color:#fff
    style PANEL fill:#1a1a2e,stroke:#ea580c,color:#fff
    style CALIB fill:#1a1a2e,stroke:#fcd34d,color:#fff
    style OUTPUT fill:#0a0a0a,stroke:#f59e0b,color:#fcd34d,stroke-width:3px
    style SENTIO_IN fill:#1a1a2e,stroke:#a855f7,color:#fff
    style LED fill:#f59e0b,stroke:#f59e0b,color:#000
```

> **Data flow**: Source content enters through the decoder, passes through color science (gamut mapping → HDR → calibration), is rendered into the 16K framebuffer at 240 Hz, distributed across the tiled MicroLED array, with SENTIO providing real-time narrative adjustments throughout the pipeline.

<br/>

## Workspace Crates

| Crate | Description |
|:------|:------------|
| **`spectra-core`** | Rendering pipeline orchestrator · 16K framebuffer management |
| **`spectra-color`** | Rec.2020+ color gamut mapping · PQ/HLG HDR tone mapping · Per-panel calibration |
| **`spectra-panel`** | MicroLED hardware abstraction · Multi-panel tiling engine · Thermal management |
| **`spectra-calibration`** | Sweet spot elimination algorithms · Brightness uniformity · Viewing angle compensation |

<br/>

## Quick Start

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Python bindings
cd python && pip install -e .
```

```rust
use spectra_core::{DisplayPipeline, DisplayConfig, Resolution};
use spectra_color::gamut::ColorSpace;

let config = DisplayConfig::builder()
    .resolution(Resolution::UHD_16K)
    .color_space(ColorSpace::Rec2020)
    .hdr_mode(HdrMode::PQ)
    .peak_brightness(12_000.0)
    .build();

let pipeline = DisplayPipeline::new(config)?;
pipeline.start()?;
```

<br/>

## Sylvain Ecosystem

<table>
<tr><td>🟡</td><td><strong>spectra</strong></td><td>16K MicroLED Display Engine</td><td><em>← you are here</em></td></tr>
<tr><td>🔵</td><td><a href="https://github.com/sylvain-cinema/sonora"><strong>sonora</strong></a></td><td>Wave Field Synthesis Audio Engine</td><td></td></tr>
<tr><td>🟣</td><td><a href="https://github.com/sylvain-cinema/sentio"><strong>sentio</strong></a></td><td>Empathic AI Narrative Intelligence</td><td></td></tr>
<tr><td>⚪</td><td><a href="https://github.com/sylvain-cinema/stratum"><strong>stratum</strong></a></td><td>Volumetric Display System</td><td></td></tr>
<tr><td>🟠</td><td><a href="https://github.com/sylvain-cinema/sylvain-sdk"><strong>sylvain-sdk</strong></a></td><td>Unified Developer SDK</td><td></td></tr>
<tr><td>🔴</td><td><a href="https://github.com/sylvain-cinema/sylvain-core"><strong>sylvain-core</strong></a></td><td>Platform Core Services</td><td></td></tr>
<tr><td>🟢</td><td><a href="https://github.com/sylvain-cinema/sylvain-cloud"><strong>sylvain-cloud</strong></a></td><td>Cloud Infrastructure</td><td></td></tr>
<tr><td>🔶</td><td><a href="https://github.com/sylvain-cinema/content-pipeline"><strong>content-pipeline</strong></a></td><td>Content Mastering Pipeline</td><td></td></tr>
<tr><td>📊</td><td><a href="https://github.com/sylvain-cinema/research"><strong>research</strong></a></td><td>Technical Papers &amp; Specs</td><td></td></tr>
<tr><td>📖</td><td><a href="https://github.com/sylvain-cinema/sylvain.github.io"><strong>docs</strong></a></td><td>Developer Documentation</td><td></td></tr>
</table>

<br/>

## License

Licensed under the [Apache License, Version 2.0](LICENSE).

<br/>

---

<div align="center">
<br/>

<img src="https://img.shields.io/badge/SYLVAIN-The_Future_of_Cinematic_Storytelling-000000?style=for-the-badge&labelColor=f59e0b&color=111111" alt="Sylvain"/>

<sub>Every Seat is the Best Seat</sub>

</div>
