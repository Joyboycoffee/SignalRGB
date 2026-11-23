# SignalRGB Projects

A collection of custom plugins and lighting effects for [SignalRGB](https://signalrgb.com/).

**Author:** Gourav Kumar  
**GitHub:** [@Joyboycoffee](https://github.com/Joyboycoffee)

---

## Directory Overview

```
SignalRGB/
├── Effects/
│   └── WallpaperSync/      # Dynamic desktop wallpaper slideshow sync effect & Rust engine
└── Plugins/
    └── Keyboard Plugin/    # Kreo Hive 98 mechanical keyboard RGB hardware driver
```

---

## 1. Effects

### [WallpaperSync](Effects/WallpaperSync/)
Smoothly synchronizes PC lighting with your Windows desktop wallpaper slideshow in real time:
- **9 Display Modes:** Solid Dominant, Horizontal Gradient, Vertical Gradient, Diagonal Flow, Radial Gradient, Breathing Ambient, Dual Breathe, Color Wave, and Ambient Drift.
- **Content-Aware Signature Engine:** Detects slideshow rotations instantly, even with older preserved image timestamps.
- **Ultra-Low Memory:** ~0.2 MB (220 KB) working set via proactive Win32 memory trimming; 0.0% CPU.
- **Embedded Binary:** Self-contained executable with one-click installation (`--install`).

---

## 2. Plugins

### [Keyboard Plugin (Kreo Hive 98)](Plugins/Keyboard%20Plugin/)
Per-key RGB hardware driver for the **Kreo Hive 98** mechanical keyboard:
- **Hardware IDs:** Vendor ID `0x320F`, Product ID `0x5055`, Interface `1`, Usage `0x0092`.
- **Custom EVision Checksum:** Reverse-engineered packet checksum formula.
- **Complete Layout Map (V2):** 18x6 canvas matrix covering all 98 addressable LEDs.

---

## License

Distributed under the MIT License.
