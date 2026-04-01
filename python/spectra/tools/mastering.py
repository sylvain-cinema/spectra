"""Content mastering utilities for SPECTRA format output."""

from dataclasses import dataclass
from enum import Enum
from typing import Optional


class VenueTier(Enum):
    """Sylvain venue tier determines output specifications."""
    SANCTUM = "sanctum"         # 16K, bespoke
    VISIONNAIRE = "visionnaire" # 16K x 16K
    ETOILEE = "etoilee"         # 8K
    ATELIER = "atelier"         # Variable


@dataclass
class MasteringSpec:
    """Output specification for a venue tier."""
    tier: VenueTier
    resolution: tuple[int, int]
    peak_nits: float
    color_space: str
    hdr_mode: str

    @classmethod
    def for_tier(cls, tier: VenueTier) -> "MasteringSpec":
        specs = {
            VenueTier.SANCTUM: cls(tier, (15360, 8640), 10000.0, "rec2020", "pq"),
            VenueTier.VISIONNAIRE: cls(tier, (15360, 8640), 10000.0, "rec2020", "pq"),
            VenueTier.ETOILEE: cls(tier, (7680, 4320), 4000.0, "rec2020", "pq"),
            VenueTier.ATELIER: cls(tier, (3840, 2160), 1000.0, "dci_p3", "pq"),
        }
        return specs[tier]
