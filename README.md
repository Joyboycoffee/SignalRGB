# SignalRGB Projects

A collection of custom plugins and lighting effects for [SignalRGB](https://signalrgb.com/).

**Author:** Gourav Kumar  
**GitHub:** [@Joyboycoffee](https://github.com/Joyboycoffee)  
**Releases:** [Download Pre-Built Executables](https://github.com/Joyboycoffee/SignalRGB/releases)

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

## 1. Effects: Wallpaper Sync

### [WallpaperSync](Effects/WallpaperSync/)
Smoothly synchronizes PC lighting with your Windows desktop wallpaper slideshow in real time:
- **9 Display Modes:** Solid Dominant, Horizontal Gradient, Vertical Gradient, Diagonal Flow, Radial Gradient, Breathing Ambient, Dual Breathe, Color Wave, and Ambient Drift.
- **Instant Slideshow Detection:** Reads Windows Registry wallpaper cache (`TranscodedImageCache`) with 250ms polling and multi-monitor support.
- **Ultra-Low Memory:** ~0.2 MB (210 KB) working set via proactive Win32 memory trimming; 0.0% CPU.
- **Automatic Silent Startup Options:**
  - `.\signalrgb-wallpaper-sync.exe --install` &mdash; One-click automatic setup that registers a silent Windows Startup shortcut and starts the background worker.
  - `.\signalrgb-wallpaper-sync.exe --companion` &mdash; Companion mode that runs while SignalRGB is open and automatically exits when SignalRGB closes.
  - `.\signalrgb-wallpaper-sync.exe --start` / `--stop` / `--status` &mdash; Background service control commands.

---

## 2. Plugins: Kreo Hive 98 Keyboard

### [Keyboard Plugin (Kreo Hive 98)](Plugins/Keyboard%20Plugin/)
Per-key RGB hardware driver for the **Kreo Hive 98** mechanical gaming keyboard:
- **Hardware IDs:** Vendor ID `0x320F`, Product ID `0x5055`, Interface `1`, Usage `0x0092`.
- **Custom EVision Checksum:** Reverse-engineered packet checksum formula for reliable USB communication.
- **Complete Layout Map (V2):** 18x6 canvas matrix covering all 98 addressable LEDs with proper positioning for arrow cluster, numpad, navigation keys, and function row.

---

## License

Distributed under the MIT License.
