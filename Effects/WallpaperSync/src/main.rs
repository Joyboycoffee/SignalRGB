use image::imageops::FilterType;
use image::GenericImageView;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const SIGNALRGB_EVENT_PORT: u16 = 16034;
const POLL_INTERVAL: Duration = Duration::from_millis(250);
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(windows)]
mod win_mem {
    extern "system" {
        fn SetProcessWorkingSetSize(
            h_process: *mut std::ffi::c_void,
            dw_minimum_working_set_size: usize,
            dw_maximum_working_set_size: usize,
        ) -> i32;
        fn GetCurrentProcess() -> *mut std::ffi::c_void;
    }

    pub fn trim_working_set() {
        unsafe {
            // usize::MAX flushes unneeded physical working set pages back to standby
            SetProcessWorkingSetSize(GetCurrentProcess(), usize::MAX, usize::MAX);
        }
    }
}

#[cfg(not(windows))]
mod win_mem {
    pub fn trim_working_set() {}
}

#[cfg(windows)]
mod win_reg {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::path::PathBuf;

    const HKEY_CURRENT_USER: usize = 0x80000001;
    const KEY_READ: u32 = 0x20019;

    #[link(name = "advapi32")]
    extern "system" {
        fn RegOpenKeyExW(
            h_key: usize,
            sub_key: *const u16,
            options: u32,
            sam_desired: u32,
            phk_result: *mut usize,
        ) -> i32;

        fn RegQueryValueExW(
            h_key: usize,
            value_name: *const u16,
            reserved: *mut u32,
            type_: *mut u32,
            data: *mut u8,
            cb_data: *mut u32,
        ) -> i32;

        fn RegCloseKey(h_key: usize) -> i32;
    }

    /// Read the active wallpaper path from Windows Registry (HKCU\Control Panel\Desktop\TranscodedImageCache)
    /// This updates immediately when Windows rotates wallpaper, even before disk transcoding completes.
    pub fn get_transcoded_image_cache_path() -> Option<PathBuf> {
        let sub_key: Vec<u16> = "Control Panel\\Desktop\0".encode_utf16().collect();
        let val_name: Vec<u16> = "TranscodedImageCache\0".encode_utf16().collect();

        let mut h_key: usize = 0;
        unsafe {
            if RegOpenKeyExW(HKEY_CURRENT_USER, sub_key.as_ptr(), 0, KEY_READ, &mut h_key) != 0 {
                return None;
            }

            let mut len: u32 = 0;
            if RegQueryValueExW(
                h_key,
                val_name.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut len,
            ) != 0
                || len < 26
            {
                RegCloseKey(h_key);
                return None;
            }

            let mut buf = vec![0u8; len as usize];
            if RegQueryValueExW(
                h_key,
                val_name.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                buf.as_mut_ptr(),
                &mut len,
            ) != 0
            {
                RegCloseKey(h_key);
                return None;
            }

            RegCloseKey(h_key);

            // TranscodedImageCache header is 24 bytes, followed by UTF-16LE null-terminated path
            let path_bytes = &buf[24..];
            let u16_slice: &[u16] =
                std::slice::from_raw_parts(path_bytes.as_ptr() as *const u16, path_bytes.len() / 2);
            let end_idx = u16_slice.iter().position(|&c| c == 0).unwrap_or(u16_slice.len());
            let os_str = OsString::from_wide(&u16_slice[..end_idx]);
            let path = PathBuf::from(os_str);
            if path.exists() {
                Some(path)
            } else {
                None
            }
        }
    }
}

#[cfg(not(windows))]
mod win_reg {
    use std::path::PathBuf;
    pub fn get_transcoded_image_cache_path() -> Option<PathBuf> {
        None
    }
}

// Embed the complete HTML effect directly inside the compiled binary
const EMBEDDED_EFFECT_HTML: &str = include_str!("../Wallpaper Sync.html");

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Clone, Copy)]
struct DominantColors {
    primary: RgbColor,
    secondary: RgbColor,
    primary_name: &'static str,
    secondary_name: &'static str,
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let v = max;
    let s = if max == 0.0 { 0.0 } else { delta / max };

    let h = if delta == 0.0 {
        0.0
    } else if max == rf {
        60.0 * (((gf - bf) / delta) % 6.0)
    } else if max == gf {
        60.0 * (((bf - rf) / delta) + 2.0)
    } else {
        60.0 * (((rf - gf) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };
    (h, s, v)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> RgbColor {
    let s = s.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);
    let c = v * s;
    let hp = (h % 360.0) / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = if hp < 1.0 {
        (c, x, 0.0)
    } else if hp < 2.0 {
        (x, c, 0.0)
    } else if hp < 3.0 {
        (0.0, c, x)
    } else if hp < 4.0 {
        (0.0, x, c)
    } else if hp < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    RgbColor {
        r: ((r1 + m) * 255.0).round() as u8,
        g: ((g1 + m) * 255.0).round() as u8,
        b: ((b1 + m) * 255.0).round() as u8,
    }
}

fn optimize_for_leds(color: RgbColor, min_sat: f32, min_val: f32) -> RgbColor {
    let (h, s, v) = rgb_to_hsv(color.r, color.g, color.b);
    let s_boosted = (s * 1.4).max(min_sat).min(1.0);
    let v_boosted = (v * 1.3).max(min_val).min(1.0);
    hsv_to_rgb(h, s_boosted, v_boosted)
}

struct BandDef {
    name: &'static str,
    low: f32,
    high: f32,
    boost: f32,
}

const BANDS: [BandDef; 8] = [
    BandDef { name: "Green",      low: 70.0,  high: 165.0, boost: 2.00 },
    BandDef { name: "Yellow",     low: 45.0,  high: 70.0,  boost: 1.40 },
    BandDef { name: "Orange",     low: 20.0,  high: 45.0,  boost: 1.30 },
    BandDef { name: "Red",        low: 345.0, high: 20.0,  boost: 1.30 },
    BandDef { name: "Magenta",    low: 315.0, high: 345.0, boost: 1.10 },
    BandDef { name: "Purple",     low: 265.0, high: 315.0, boost: 1.00 },
    BandDef { name: "Cyan/Teal",  low: 165.0, high: 205.0, boost: 0.95 },
    BandDef { name: "Blue",       low: 205.0, high: 265.0, boost: 0.85 },
];

#[derive(Default, Clone, Copy)]
struct PixelEntry {
    r: u8,
    g: u8,
    b: u8,
    score: f32,
}

#[derive(Default)]
struct BandScore {
    total_score: f32,
    pixels: Vec<PixelEntry>,
}

fn extract_colors(img_bytes: &[u8]) -> Result<DominantColors, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(img_bytes)?;
    let (thumb_w, thumb_h) = (64u32, 64u32);
    let thumb = img.resize_exact(thumb_w, thumb_h, FilterType::Nearest);

    let mut band_scores: Vec<BandScore> = (0..BANDS.len()).map(|_| BandScore::default()).collect();
    let mut total_r = 0u64;
    let mut total_g = 0u64;
    let mut total_b = 0u64;
    let mut non_black_count = 0u64;

    for (x, y, pixel) in thumb.pixels() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        let max_c = r.max(g).max(b);
        let min_c = r.min(g).min(b);

        if max_c < 22 {
            continue;
        }

        total_r += r as u64;
        total_g += g as u64;
        total_b += b as u64;
        non_black_count += 1;

        let chroma = (max_c - min_c) as f32 / 255.0;
        if chroma < 0.04 {
            continue;
        }

        let (h_deg, _s, _v) = rgb_to_hsv(r, g, b);

        let nx = (x as f32 - thumb_w as f32 / 2.0) / (thumb_w as f32 / 2.0);
        let ny = (y as f32 - thumb_h as f32 * 0.55) / (thumb_h as f32 * 0.45);
        let spatial_dist_sq = nx * nx + ny * ny;
        let spatial_wt = 0.65 + 0.35 * (-spatial_dist_sq / 2.0).exp();

        let lum = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;

        for (band_idx, band) in BANDS.iter().enumerate() {
            let matched = if band.low > band.high {
                h_deg >= band.low || h_deg < band.high
            } else {
                h_deg >= band.low && h_deg < band.high
            };

            if matched {
                let salience = chroma.powf(0.80) * lum.powf(0.60) * spatial_wt * band.boost;
                let entry = &mut band_scores[band_idx];
                entry.total_score += salience;
                entry.pixels.push(PixelEntry { r, g, b, score: salience });
                break;
            }
        }
    }

    let mut ranked_indices: Vec<usize> = (0..BANDS.len()).collect();
    ranked_indices.sort_by(|&a, &b| {
        band_scores[b].total_score.partial_cmp(&band_scores[a].total_score).unwrap_or(std::cmp::Ordering::Equal)
    });

    let get_rep_color = |band_idx: usize| -> Option<RgbColor> {
        let entry = &band_scores[band_idx];
        if entry.pixels.is_empty() {
            return None;
        }
        let mut px = entry.pixels.clone();
        px.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        let top_count = (px.len() / 5).max(5).min(px.len());
        let slice = &px[..top_count];
        let sum_r: u64 = slice.iter().map(|p| p.r as u64).sum();
        let sum_g: u64 = slice.iter().map(|p| p.g as u64).sum();
        let sum_b: u64 = slice.iter().map(|p| p.b as u64).sum();
        let raw = RgbColor {
            r: (sum_r / top_count as u64) as u8,
            g: (sum_g / top_count as u64) as u8,
            b: (sum_b / top_count as u64) as u8,
        };
        Some(optimize_for_leds(raw, 0.60, 0.75))
    };

    let first_idx = ranked_indices[0];
    if band_scores[first_idx].total_score > 0.5 {
        let primary = get_rep_color(first_idx).unwrap();
        let primary_name = BANDS[first_idx].name;

        let secondary_choice = ranked_indices.iter().skip(1).find_map(|&idx| {
            if band_scores[idx].total_score > 0.5 {
                get_rep_color(idx).map(|col| (col, BANDS[idx].name))
            } else {
                None
            }
        });

        let (secondary, secondary_name) = secondary_choice.unwrap_or((primary, primary_name));

        Ok(DominantColors {
            primary,
            secondary,
            primary_name,
            secondary_name,
        })
    } else {
        let avg_r = (total_r / non_black_count.max(1)) as u8;
        let avg_g = (total_g / non_black_count.max(1)) as u8;
        let avg_b = (total_b / non_black_count.max(1)) as u8;
        let col = RgbColor {
            r: avg_r.max(60),
            g: avg_g.max(60),
            b: avg_b.max(60),
        };
        Ok(DominantColors {
            primary: col,
            secondary: col,
            primary_name: "Neutral",
            secondary_name: "Neutral",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WallpaperSignature {
    path: PathBuf,
    len: u64,
    mod_time: SystemTime,
    sample_hash: u64,
}

fn compute_sample_hash(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn get_active_wallpaper() -> Option<WallpaperSignature> {
    let mut candidates: Vec<PathBuf> = Vec::new();

    // 1. Primary check: Exact original wallpaper path from Windows Registry cache
    // This updates immediately on "Next desktop background" or slideshow transition
    if let Some(reg_path) = win_reg::get_transcoded_image_cache_path() {
        candidates.push(reg_path);
    }

    // 2. Candidate files in Themes directory
    let appdata = env::var("APPDATA").unwrap_or_else(|_| "C:\\Users\\Default\\AppData\\Roaming".to_string());
    let themes_dir = Path::new(&appdata).join("Microsoft").join("Windows").join("Themes");

    candidates.push(themes_dir.join("TranscodedWallpaper"));
    candidates.push(themes_dir.join("Transcoded_000"));
    candidates.push(themes_dir.join("Transcoded_001"));
    candidates.push(themes_dir.join("Transcoded_002"));

    for path in &candidates {
        if path.exists() {
            if let Ok(meta) = std::fs::metadata(path) {
                let len = meta.len();
                if len > 512 {
                    let mod_time = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);

                    // Sample the first 8KB for fast content-aware signature hashing
                    let mut sample = vec![0u8; 8192.min(len as usize)];
                    if let Ok(mut f) = File::open(path) {
                        let _ = f.read_exact(&mut sample);
                    }

                    let sample_hash = compute_sample_hash(&sample);
                    return Some(WallpaperSignature {
                        path: path.clone(),
                        len,
                        mod_time,
                        sample_hash,
                    });
                }
            }
        }
    }
    None
}

fn read_with_retry(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    for _ in 0..8 {
        match File::open(path) {
            Ok(mut file) => {
                let mut buf = Vec::new();
                if file.read_to_end(&mut buf).is_ok() && buf.len() > 1024 {
                    return Ok(buf);
                }
            }
            Err(_) => {}
        }
        sleep(Duration::from_millis(50));
    }
    std::fs::read(path)
}

fn is_signalrgb_active() -> bool {
    use std::net::{SocketAddr, TcpStream};
    let addr: SocketAddr = "127.0.0.1:16034".parse().unwrap();
    TcpStream::connect_timeout(&addr, Duration::from_millis(50)).is_ok()
}

fn send_to_signalrgb(colors: &DominantColors) -> Result<(), Box<dyn std::error::Error>> {
    let p = colors.primary;
    let s = colors.secondary;

    let target_url = format!(
        "http://localhost:{}/canvas/event?sender=WallpaperSync&event={},{},{},{},{},{}",
        SIGNALRGB_EVENT_PORT,
        p.r, p.g, p.b,
        s.r, s.g, s.b
    );

    let resp = ureq::post(&target_url)
        .timeout(Duration::from_millis(1500))
        .call()?;

    if resp.status() == 200 {
        Ok(())
    } else {
        Err(format!("SignalRGB returned HTTP {}", resp.status()).into())
    }
}

// -------------------------------------------------------------
// Professional Management Subcommands (No VBS / No BAT needed!)
// -------------------------------------------------------------

fn get_signalrgb_effects_path() -> PathBuf {
    let user_profile = env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string());
    Path::new(&user_profile).join("Documents").join("WhirlwindFX").join("Effects")
}

fn get_startup_shortcut_path() -> PathBuf {
    let appdata = env::var("APPDATA").unwrap_or_else(|_| "C:\\Users\\Default\\AppData\\Roaming".to_string());
    Path::new(&appdata)
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("Startup")
        .join("SignalRGB-Wallpaper-Sync.lnk")
}

fn kill_other_instances() {
    let my_pid = std::process::id();
    let _ = Command::new("taskkill")
        .args([
            "/F",
            "/FI",
            &format!("PID ne {}", my_pid),
            "/IM",
            "signalrgb-wallpaper-sync.exe",
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
}

fn is_other_instance_running() -> bool {
    let my_pid = std::process::id();
    let output = Command::new("tasklist")
        .args([
            "/FI",
            "IMAGENAME eq signalrgb-wallpaper-sync.exe",
            "/FO",
            "CSV",
            "/NH",
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            if line.contains("signalrgb-wallpaper-sync.exe") && !line.contains(&format!("\"{}\"", my_pid)) {
                return true;
            }
        }
    }
    false
}

fn spawn_daemon() -> bool {
    let exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("signalrgb-wallpaper-sync.exe"));
    let ps_cmd = format!(
        "Start-Process -FilePath '{}' -ArgumentList '--daemon' -WindowStyle Hidden",
        exe_path.display()
    );
    let status = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &ps_cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

fn cmd_install() {
    println!("============================================================");
    println!("  SignalRGB Wallpaper Sync - One-Click Installer");
    println!("  Author: Gourav Kumar");
    println!("============================================================");

    // 1. Install Effect HTML
    let effects_dir = get_signalrgb_effects_path();
    let _ = std::fs::create_dir_all(&effects_dir);
    let target_html = effects_dir.join("Wallpaper Sync.html");
    match std::fs::write(&target_html, EMBEDDED_EFFECT_HTML) {
        Ok(_) => println!("[1/3] Installed 'Wallpaper Sync.html' into SignalRGB Effects library."),
        Err(e) => eprintln!("[1/3] ERROR installing HTML effect: {}", e),
    }

    // 2. Configure Windows Startup Shortcut
    let exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("signalrgb-wallpaper-sync.exe"));
    let exe_dir = exe_path.parent().unwrap_or(Path::new("."));
    let shortcut_cmd = format!(
        "$wsh = New-Object -ComObject WScript.Shell; \
         $startup = [System.Environment]::GetFolderPath('Startup'); \
         $lnk = $wsh.CreateShortcut((Join-Path $startup 'SignalRGB-Wallpaper-Sync.lnk')); \
         $lnk.TargetPath = '{}'; \
         $lnk.Arguments = '--daemon'; \
         $lnk.WorkingDirectory = '{}'; \
         $lnk.Description = 'SignalRGB Wallpaper Slideshow Sync'; \
         $lnk.Save()",
        exe_path.display(),
        exe_dir.display()
    );

    let res = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &shortcut_cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    // Also register in HKCU\Software\Microsoft\Windows\CurrentVersion\Run alongside SignalRGB
    let reg_cmd = format!(
        "Set-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run' -Name 'SignalRGB-Wallpaper-Sync' -Value '\"{}\" --daemon'",
        exe_path.display()
    );
    let _ = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &reg_cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    if res.is_ok() {
        println!("[2/3] Configured Windows Startup & Run registry (runs silently on boot alongside SignalRGB).");
    } else {
        eprintln!("[2/3] WARNING: Could not register Windows Startup automatically.");
    }

    // 3. Start background daemon
    kill_other_instances();
    sleep(Duration::from_millis(400));
    if spawn_daemon() {
        println!("[3/3] Started background sync daemon.");
    } else {
        eprintln!("[3/3] Could not spawn background daemon.");
    }

    println!("\nSUCCESS: SignalRGB Wallpaper Sync is installed and active!");
    println!("Open SignalRGB -> Lighting -> My Effects -> select 'Wallpaper Sync'.");
}

fn cmd_uninstall() {
    println!("Uninstalling SignalRGB Wallpaper Sync...");
    kill_other_instances();
    println!("[OK] Stopped background sync daemon.");

    let shortcut = get_startup_shortcut_path();
    if shortcut.exists() {
        let _ = std::fs::remove_file(shortcut);
        println!("[OK] Removed Windows Startup shortcut.");
    }

    let unreg_cmd = "Remove-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run' -Name 'SignalRGB-Wallpaper-Sync' -ErrorAction SilentlyContinue";
    let _ = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", unreg_cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    println!("[OK] Removed Windows Run registry entry.");

    println!("Uninstallation complete.");
}

fn cmd_start() {
    if is_other_instance_running() {
        println!("SignalRGB Wallpaper Sync daemon is already running.");
        return;
    }
    if spawn_daemon() {
        println!("[OK] Started background sync daemon.");
    } else {
        eprintln!("[ERROR] Could not start background sync daemon.");
    }
}

fn cmd_stop() {
    kill_other_instances();
    println!("[OK] Stopped all background sync instances.");
}

fn cmd_status() {
    let running = is_other_instance_running();
    let effect_installed = get_signalrgb_effects_path().join("Wallpaper Sync.html").exists();
    let startup_configured = get_startup_shortcut_path().exists();

    let reg_check_cmd = "(Get-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run' -Name 'SignalRGB-Wallpaper-Sync' -ErrorAction SilentlyContinue) -ne $null";
    let reg_configured = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", reg_check_cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "True")
        .unwrap_or(false);

    println!("SignalRGB Wallpaper Sync Status:");
    println!("  Background Daemon Running : {}", if running { "YES" } else { "NO" });
    println!("  SignalRGB Effect Installed: {}", if effect_installed { "YES" } else { "NO" });
    println!("  Windows Startup Shortcut  : {}", if startup_configured { "YES" } else { "NO" });
    println!("  Windows Registry Autostart: {}", if reg_configured { "YES" } else { "NO" });
}

fn print_help() {
    println!("SignalRGB Wallpaper Slideshow Sync v1.0.0");
    println!("Author: Gourav Kumar\n");
    println!("USAGE:");
    println!("  signalrgb-wallpaper-sync.exe [COMMAND]\n");
    println!("COMMANDS:");
    println!("  (no args)     Run interactively with real-time console logs");
    println!("  --install     One-click install: embeds effect, registers Windows startup, and starts service");
    println!("  --uninstall   Removes Windows startup registration and stops service");
    println!("  --start       Starts the sync service silently in the background (zero window)");
    println!("  --stop        Stops the running background service");
    println!("  --status      Checks if the background service is active");
    println!("  --daemon      Internal headless background worker (persistent)");
    println!("  --companion   Headless background worker that auto-exits when SignalRGB closes");
    println!("  --help, -h    Displays this help menu");
}

// -------------------------------------------------------------
// Main Loop
// -------------------------------------------------------------

fn run_sync_loop(is_daemon: bool, is_companion: bool) {
    if !is_daemon {
        println!("============================================================");
        println!("  SignalRGB Wallpaper Slideshow Sync (Perceptual AI) v1.0.0");
        println!("  Author: Gourav Kumar");
        println!("============================================================");
    }

    let mut last_sig: Option<WallpaperSignature> = None;
    let mut last_primary: Option<RgbColor> = None;
    let mut last_secondary: Option<RgbColor> = None;
    let mut last_colors: Option<DominantColors> = None;
    let mut srgb_unreachable_count = 0u32;
    let mut heartbeat_ticks = 0u32;
    let mut was_srgb_active = false;

    loop {
        let srgb_active = is_signalrgb_active();

        // In companion mode, auto-terminate if SignalRGB port is down for multiple consecutive cycles
        if is_companion {
            if !srgb_active {
                srgb_unreachable_count += 1;
                if srgb_unreachable_count >= 12 {
                    // SignalRGB closed, cleanly terminate companion
                    exit(0);
                }
            } else {
                srgb_unreachable_count = 0;
            }
        }

        // When SignalRGB is NOT active (and not companion auto-exiting):
        // Idle peacefully with ultra-low resource consumption
        if !srgb_active {
            was_srgb_active = false;
            win_mem::trim_working_set();
            sleep(Duration::from_millis(1500));
            continue;
        }

        // The exact moment SignalRGB turns ON (transition from inactive -> active):
        // Force an instant detection and push so SignalRGB gets the wallpaper colors immediately!
        if !was_srgb_active {
            was_srgb_active = true;
            last_sig = None;
            if !is_daemon {
                println!("[SignalRGB Connected] SignalRGB detected! Synchronizing wallpaper colors immediately...");
            }
        }

        heartbeat_ticks += 1;

        if let Some(sig) = get_active_wallpaper() {
            let signature_changed = match &last_sig {
                Some(prev) => prev != &sig,
                None => true,
            };

            if signature_changed {
                sleep(Duration::from_millis(50));

                match read_with_retry(&sig.path) {
                    Ok(bytes) => match extract_colors(&bytes) {
                        Ok(colors) => {
                            let colors_changed = match (last_primary, last_secondary) {
                                (Some(lp), Some(ls)) => lp != colors.primary || ls != colors.secondary,
                                _ => true,
                            };

                            let mut push_success = false;
                            if colors_changed {
                                let p = colors.primary;
                                let s = colors.secondary;
                                if !is_daemon {
                                    println!("\n[Wallpaper Changed] Detected new slideshow frame!");
                                    println!("  -> Source File    : {}", sig.path.display());
                                    println!(
                                        "  -> Primary Color  : [{}] RGB({:>3}, {:>3}, {:>3})  #{:02x}{:02x}{:02x}",
                                        colors.primary_name, p.r, p.g, p.b, p.r, p.g, p.b
                                    );
                                    println!(
                                        "  -> Secondary Color: [{}] RGB({:>3}, {:>3}, {:>3})  #{:02x}{:02x}{:02x}",
                                        colors.secondary_name, s.r, s.g, s.b, s.r, s.g, s.b
                                    );
                                }

                                match send_to_signalrgb(&colors) {
                                    Ok(_) => {
                                        if !is_daemon {
                                            println!("  [OK] Pushed vibrant colors to SignalRGB port {}", SIGNALRGB_EVENT_PORT);
                                        }
                                        last_primary = Some(colors.primary);
                                        last_secondary = Some(colors.secondary);
                                        last_colors = Some(colors);
                                        push_success = true;
                                        heartbeat_ticks = 0;
                                    }
                                    Err(e) => {
                                        if !is_daemon {
                                            eprintln!("  [WARN] Could not send to SignalRGB (is SignalRGB open?): {}", e);
                                        }
                                    }
                                }
                            } else {
                                push_success = true;
                            }

                            // Only record signature as processed if we successfully notified SignalRGB
                            // (or colors didn't change). If SignalRGB was unreachable, we will retry immediately!
                            if push_success {
                                last_sig = Some(sig);
                            }
                        }
                        Err(e) => {
                            if !is_daemon {
                                eprintln!("[ERROR] Extracting colors failed: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        if !is_daemon {
                            eprintln!("[ERROR] Reading wallpaper file failed: {}", e);
                        }
                    }
                }
            } else if heartbeat_ticks >= 16 {
                // Heartbeat every ~4 seconds (16 ticks * 250ms)
                // Ensures that if SignalRGB reloads the effect or starts up, the active colors are pushed
                heartbeat_ticks = 0;
                if let Some(ref colors) = last_colors {
                    let _ = send_to_signalrgb(colors);
                }
            }
        }

        // Proactively trim working set physical RAM before sleeping
        win_mem::trim_working_set();
        sleep(POLL_INTERVAL);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--install" => {
                cmd_install();
                exit(0);
            }
            "--uninstall" => {
                cmd_uninstall();
                exit(0);
            }
            "--start" => {
                cmd_start();
                exit(0);
            }
            "--stop" => {
                cmd_stop();
                exit(0);
            }
            "--status" => {
                cmd_status();
                exit(0);
            }
            "--daemon" => {
                run_sync_loop(true, false);
            }
            "--companion" => {
                run_sync_loop(true, true);
            }
            "--help" | "-h" => {
                print_help();
                exit(0);
            }
            other => {
                eprintln!("Unknown option: {}", other);
                print_help();
                exit(1);
            }
        }
    } else {
        run_sync_loop(false, false);
    }
}
