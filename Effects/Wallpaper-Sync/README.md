# SignalRGB Wallpaper Slideshow Sync

[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-blue.svg)](https://github.com/Joyboycoffee/signalrgb-wallpaper-sync)
[![Language](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![SignalRGB](https://img.shields.io/badge/SignalRGB-Compatible-purple.svg)](https://signalrgb.com/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

A high-performance, zero-dependency native Windows utility and SignalRGB effect that automatically extracts dominant and accent colors from your Windows desktop wallpaper slideshow and smoothly synchronizes them with your PC's RGB ecosystem in real time.

**Author:** Gourav Kumar  
**GitHub:** [@Joyboycoffee](https://github.com/Joyboycoffee)

---

## Highlights

- **Standalone Single Binary:** No batch files, no VBS scripts, no Python runtime. Everything—including the SignalRGB effect HTML, Windows Startup manager, and background daemon—is compiled directly into a single lightweight executable (`signalrgb-wallpaper-sync.exe`).
- **Human Perceptual Color Engine:**
  - 8-band perceptual color binning (Green, Yellow, Orange, Red, Magenta, Purple, Cyan/Teal, Blue).
  - Photopic luminance weighting ($0.299R + 0.587G + 0.114B$) tuned to human visual sensitivity.
  - Spatial composition and saliency weighting prioritizing the focal subject.
  - Hardware LED gamut optimization (boosts saturation and minimum luminance so LEDs produce vivid, saturated colors rather than murky shadows).
- **9 Dynamic Display Modes:**
  1. **Solid Dominant:** Immersive solid illumination matching the primary wallpaper hue.
  2. **Horizontal Gradient:** Smooth left-to-right blend from primary to accent color across your canvas.
  3. **Vertical Gradient:** Top-to-bottom color gradient.
  4. **Diagonal Flow:** 45-degree angled color transition.
  5. **Radial Gradient:** Dynamic focal burst expanding outward from the center.
  6. **Breathing Ambient:** Gentle harmonic luminance pulse of the dominant color.
  7. **Dual Breathe:** Mesmerizing cross-fade breathing between primary and accent hues.
  8. **Color Wave:** Traveling sinusoidal gradient wave traversing your RGB devices.
  9. **Ambient Drift:** Celestial orbital plasma drift creating organic fluid motion across lighting zones.
- **Ultra-Low Resource Footprint:**
  - **RAM:** ~7 MB
  - **CPU:** 0.0% (efficient file-system polling with sub-millisecond sleep intervals)

---

## Quick Start (Pre-Built Binary)

1. Download `signalrgb-wallpaper-sync.exe` from the [Releases](https://github.com/Joyboycoffee/signalrgb-wallpaper-sync/releases) page.
2. Open a terminal (PowerShell or Command Prompt) and run:
   ```powershell
   .\signalrgb-wallpaper-sync.exe --install
   ```
   **What `--install` does automatically:**
   - Deploys the embedded `Wallpaper Sync.html` effect directly to `%USERPROFILE%\Documents\WhirlwindFX\Effects\`.
   - Creates a Windows Startup shortcut pointing to `signalrgb-wallpaper-sync.exe --daemon` so it boots silently with your PC.
   - Spawns the background sync worker immediately (zero visible window).

3. In **SignalRGB**, navigate to **Lighting** -> **My Effects** -> Select **Wallpaper Sync**.

---

## CLI Reference

`signalrgb-wallpaper-sync.exe` can be used either as an interactive diagnostic console or as a silent background service manager:

```
USAGE:
  signalrgb-wallpaper-sync.exe [COMMAND]

COMMANDS:
  (no args)     Run interactively with real-time color telemetry logs
  --install     One-click setup: deploys effect, registers Windows Startup, starts daemon
  --uninstall   Removes Windows Startup shortcut and stops background service
  --start       Starts the sync service silently in background (CREATE_NO_WINDOW)
  --stop        Stops any running background sync instances
  --status      Inspects background daemon and startup registration status
  --daemon      Internal headless background worker (used by Startup shortcut)
  --help, -h    Displays help information
```

### Interactive Console View
Running without arguments launches interactive mode, printing real-time color decisions whenever the wallpaper rotates:
```
============================================================
  SignalRGB Wallpaper Slideshow Sync (Perceptual AI) v1.0.0
  Author: Gourav Kumar
============================================================
Target Wallpaper Path: C:\Users\Gourav\AppData\Roaming\Microsoft\Windows\Themes\TranscodedWallpaper

[Wallpaper Changed] Detected new slideshow frame!
  -> Primary Color  : [Green] RGB( 88, 191,  76)  #58bf4c
  -> Secondary Color: [Blue]  RGB(  0, 137, 205)  #0089cd
  [OK] Pushed vibrant colors to SignalRGB port 16034
```

---

## How It Works

```
+------------------------------------+
|  Windows Desktop Slideshow         |
|  (%APPDATA%\...\TranscodedWallpaper)
+-----------------+------------------+
                  | (Timestamp Change)
                  v
+------------------------------------+
|  signalrgb-wallpaper-sync.exe      |
|  - 64x64 Fast Downsampling         |
|  - HSV & Perceptual Photopic Score |
|  - LED Saturation & Value Boost    |
+-----------------+------------------+
                  | HTTP GET (port 16034)
                  | /canvas/event?sender=WallpaperSync&event=r,g,b,sr,sg,sb
                  v
+------------------------------------+
|  SignalRGB Canvas Engine           |
|  - Wallpaper Sync.html             |
|  - Cubic Ease-in-Out Interpolation |
|  - 9 Selectable Render Modes       |
|  - Hardware Canvas Push to Devices |
+------------------------------------+
```

---

## Building from Source

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (1.75+ recommended)
- Windows 10 or 11

### Build Steps
```powershell
git clone https://github.com/Joyboycoffee/signalrgb-wallpaper-sync.git
cd signalrgb-wallpaper-sync
cargo build --release
```
The compiled binary will be located at `target/release/signalrgb-wallpaper-sync.exe`.

---

## Contributing & License

Pull requests and feature suggestions are welcome! Distributed under the MIT License.
