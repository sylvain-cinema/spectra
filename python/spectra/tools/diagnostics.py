"""Display diagnostics and health monitoring."""

from dataclasses import dataclass
from typing import List


@dataclass
class PanelHealth:
    """Health status of a single MicroLED panel."""
    panel_id: int
    temperature_celsius: float
    dead_pixel_count: int
    brightness_uniformity: float
    uptime_hours: float


@dataclass
class DisplayHealth:
    """Aggregated health of the entire display system."""
    panels: List[PanelHealth]
    total_panels: int
    online_panels: int
    avg_temperature: float
    total_dead_pixels: int

    @property
    def health_score(self) -> float:
        """Overall health score (0.0 to 1.0)."""
        if self.total_panels == 0:
            return 0.0
        online_ratio = self.online_panels / self.total_panels
        temp_score = max(0, 1.0 - (self.avg_temperature - 40) / 50)
        return min(online_ratio, temp_score)
