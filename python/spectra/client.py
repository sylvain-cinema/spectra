"""SPECTRA display client for Python applications."""

from dataclasses import dataclass
from enum import Enum
from typing import Optional


class Resolution(Enum):
    """Display resolution presets."""
    UHD_8K = (7680, 4320)
    UHD_16K = (15360, 8640)


class HdrMode(Enum):
    """HDR transfer function."""
    PQ = "pq"
    HLG = "hlg"
    SDR = "sdr"


class ColorSpace(Enum):
    """Color space."""
    REC709 = "rec709"
    DCI_P3 = "dci_p3"
    REC2020 = "rec2020"


@dataclass
class DisplayConfig:
    """Configuration for a SPECTRA display instance."""
    resolution: Resolution = Resolution.UHD_16K
    hdr_mode: HdrMode = HdrMode.PQ
    color_space: ColorSpace = ColorSpace.REC2020
    peak_brightness_nits: float = 10_000.0
    refresh_rate_hz: int = 120


class SpectraClient:
    """Client for controlling a SPECTRA display system."""

    def __init__(self, host: str, port: int = 9100, config: Optional[DisplayConfig] = None):
        self.host = host
        self.port = port
        self.config = config or DisplayConfig()
        self._connected = False

    def connect(self) -> None:
        """Connect to the SPECTRA display controller."""
        self._connected = True

    def disconnect(self) -> None:
        """Disconnect from the display controller."""
        self._connected = False

    @property
    def is_connected(self) -> bool:
        return self._connected
