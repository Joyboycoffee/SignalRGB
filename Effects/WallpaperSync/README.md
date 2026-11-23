# SignalRGB Wallpaper Slideshow Sync

[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-blue.svg)](https://github.com/Joyboycoffee/SignalRGB)
[![Language](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![SignalRGB](https://img.shields.io/badge/SignalRGB-Compatible-purple.svg)](https://signalrgb.com/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

A high-performance, zero-dependency native Windows utility and SignalRGB effect that automatically extracts dominant and accent colors from your Windows desktop wallpaper slideshow and smoothly synchronizes them with your PC's RGB ecosystem in real time.

**Author:** Gourav Kumar  
**GitHub:** [@Joyboycoffee](https://github.com/Joyboycoffee)  
**Releases:** [Download Pre-Built Executable](https://github.com/Joyboycoffee/SignalRGB/releases)

---

## Highlights

- **Standalone Single Binary:** No batch files, no VBS scripts, no Python runtime. Everything—including the SignalRGB effect HTML, Windows Startup manager, and background daemon—is compiled directly into a single lightweight executable (`signalrgb-wallpaper-sync.exe`).
- **Instant Slideshow Detection:** Directly hooks into the Windows Registry wallpaper cache (`TranscodedImageCache`) with 250ms polling and multi-monitor support (`Transcoded_000`, `Transcoded_001`, `Transcoded_002`), reacting the instant a new background appears.
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
  - **RAM:** ~0.2 MB (210 KB) via proactive Win32 working set memory trimming
  - **CPU:** 0.0% (sub-millisecond signature comparison)

---

## Automatic Startup & Running Silently

You have three simple ways to make the companion app run silently and automatically:

### Option 1: One-Click Automatic Setup (Recommended)
Run the executable once with `--install`:
```powershell
.\signalrgb-wallpaper-sync.exe --install
```
**What this does automatically:**
1. Installs `Wallpaper Sync.html` directly into `%USERPROFILE%\Documents\WhirlwindFX\Effects\`.
2. Registers a silent Windows Startup shortcut pointing to `signalrgb-wallpaper-sync.exe --daemon`.
3. Launches the background sync daemon immediately with zero visible window.
4. From now on, whenever your computer boots up, the sync daemon starts silently in the background, using only ~0.2 MB RAM and 0% CPU.

---

### Option 2: Auto-Run & Auto-Exit with SignalRGB (`--companion` Mode)
If you prefer the app to **only run when SignalRGB is active**, use `--companion`:
```powershell
.\signalrgb-wallpaper-sync.exe --companion
```
- In companion mode, the tool monitors SignalRGB's local communication port.
- As soon as you close or exit SignalRGB, the companion app **automatically terminates itself**, freeing all resources.
- You can add this command to any launcher, batch file, or task scheduler tied to SignalRGB's startup.

---

### Option 3: Manual Windows Startup
If you prefer to configure it manually without the installer:
1. Press `Win + R`, type `shell:startup`, and press Enter.
2. Right-click inside the folder &rarr; **New** &rarr; **Shortcut**.
3. Browse to `signalrgb-wallpaper-sync.exe`.
4. Right-click the newly created shortcut &rarr; **Properties**.
5. In the **Target** field, append ` --daemon` to the end:
   ```
   "C:\Path\To\signalrgb-wallpaper-sync.exe" --daemon
   ```
6. Set **Run** to `Minimized` (or leave as Normal; `--daemon` runs completely hidden).

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
  --start       Starts the sync service silently in background (zero window)
  --stop        Stops any running background sync instances
  --status      Inspects background daemon and startup registration status
  --daemon      Internal headless background worker (runs persistently at startup)
  --companion   Headless worker that automatically exits when SignalRGB is closed
  --help, -h    Displays help information
```

### Interactive Console View
Running without arguments launches interactive mode, printing real-time color decisions whenever the wallpaper rotates:
```
============================================================
  SignalRGB Wallpaper Slideshow Sync (Perceptual AI) v1.0.0
  Author: Gourav Kumar
============================================================

[Wallpaper Changed] Detected new slideshow frame!
  -> Source File    : C:\Users\Gourav\Pictures\Wallpapers\Tokyo_Night.png
  -> Primary Color  : [Green] RGB(103, 191,  76)  #67bf4c
  -> Secondary Color: [Cyan]  RGB(  0, 143, 212)  #008fd4
  [OK] Pushed vibrant colors to SignalRGB port 16034
```

---

## How It Works

```
+-------------------------------------------------------------+
|  Windows Desktop Slideshow / Registry Cache                 |
|  (HKCU\Control Panel\Desktop\TranscodedImageCache)          |
+------------------------------+------------------------------+
                               | (Instant Cache Change, 250ms)
                               v
+-------------------------------------------------------------+
|  signalrgb-wallpaper-sync.exe (Rust Daemon)                 |
|  - 64x64 Fast Downsampling                                  |
|  - HSV & Perceptual Photopic Scoring                        |
|  - LED Saturation & Value Boost                             |
|  - Periodic Heartbeat Sync (~4s)                            |
+------------------------------+------------------------------+
                               | HTTP POST (port 16034)
                               | /canvas/event?sender=WallpaperSync&event=r,g,b,sr,sg,sb
                               v
+-------------------------------------------------------------+
|  SignalRGB Canvas Engine                                    |
|  - Wallpaper Sync.html                                      |
|  - Smooth Cubic Ease-in-Out Interpolation                   |
|  - 9 Selectable Render Modes                                |
|  - Hardware Canvas Push to All Peripherals & Devices        |
+-------------------------------------------------------------+
```

---

## Building from Source

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (1.75+ recommended)
- Windows 10 or 11

### Build Steps
```powershell
git clone https://github.com/Joyboycoffee/SignalRGB.git
cd SignalRGB/Effects/WallpaperSync
cargo build --release
```
The compiled binary will be located at `target/release/signalrgb-wallpaper-sync.exe`.

---

## Contributing & License

Pull requests and feature suggestions are welcome! Distributed under the MIT License.
