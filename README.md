# SignalRGB Custom Suite

A curated suite of custom hardware plugins and visual effects for [SignalRGB](https://signalrgb.com/), engineered for high performance, zero bloat, and seamless PC RGB synchronization.

**Author:** Gourav Kumar  
**GitHub:** [@Joyboycoffee](https://github.com/Joyboycoffee)

---

## Suite Contents

```
SignalRGB/
├── Effects/
│   └── Wallpaper-Sync/         # Dynamic wallpaper slideshow color sync engine & effect
│       ├── Wallpaper Sync.html # HTML Canvas effect with 9 display modes
│       ├── src/main.rs         # High-efficiency Rust companion background service
│       └── README.md
└── Plugins/
    └── Kreo-Hive-98/           # Per-key RGB device driver for Kreo Hive 98 keyboard
        ├── kreo_hive_98_v2.js  # Recommended V2 driver with full 18x6 matrix mapping
        ├── kreo_hive_98.js     # Legacy V1 driver
        └── README.md
```

---

## 1. Wallpaper Slideshow Sync (`Effects/Wallpaper-Sync`)

An automated bridge that watches your Windows desktop wallpaper slideshow and smoothly synchronizes your PC's RGB lighting with the active wallpaper's dominant and accent colors in real time.

### Highlights:
- **Zero-Dependency Single Binary:** All functionality—including the embedded HTML canvas effect, Windows Startup registration, and background daemon—is compiled into one standalone executable (`signalrgb-wallpaper-sync.exe`).
- **Human Perceptual Color Engine:** 8 perceptual color bands with photopic luminance weighting ($0.299R + 0.587G + 0.114B$) and spatial composition weighting so greens and focal subjects are correctly prioritized over washed-out backgrounds.
- **Ultra-Low Memory Footprint:** Uses active Windows working set trimming (`SetProcessWorkingSetSize`), dropping idle physical memory usage to **~200 KB (0.2 MB)** with **0.0% CPU**.
- **9 Dynamic Canvas Modes:**
  - `Solid Dominant`
  - `Horizontal Gradient`
  - `Vertical Gradient`
  - `Diagonal Flow`
  - `Radial Gradient`
  - `Breathing Ambient`
  - `Dual Breathe`
  - `Color Wave`
  - `Ambient Drift`
- **One-Click Installation:**
  ```powershell
  .\signalrgb-wallpaper-sync.exe --install
  ```

---

## 2. Kreo Hive 98 Keyboard Plugin (`Plugins/Kreo-Hive-98`)

Custom device driver enabling per-key lighting synchronization for the **Kreo Hive 98** mechanical keyboard.

### Highlights:
- **Hardware IDs:** Vendor ID `0x320F`, Product ID `0x5055`, Interface `1`, Usage `0x0092`.
- **EVision Protocol Reverse-Engineering:** Resolves the EVision checksum algorithm required for packet integrity:
  `calculatedValue = 18 + bytesToSend + bsLow + bsHigh + packetSum`
- **18x6 Canvas Layout Matrix:** Exact coordinate mapping for all 98 addressable LEDs including Function keys, Arrow cluster, and 10-key Numeric Pad.
- **Installation:**
  Copy `kreo_hive_98_v2.js` to `%USERPROFILE%\Documents\WhirlwindFX\Plugins\Kreo\`.

---

## Contributing & License

Created by Gourav Kumar. Distributed under the MIT License.
