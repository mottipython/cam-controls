# Camera Quick Controls (`cam-controls`)

[![CI Package & Installation Tests](https://github.com/mottipython/cam-controls/actions/workflows/test.yml/badge.svg)](https://github.com/mottipython/cam-controls/actions/workflows/test.yml)
[![Release](https://img.shields.io/github/v/release/mottipython/cam-controls)](https://github.com/mottipython/cam-controls/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, native Rust camera tuning utility and PipeWire fix for Linux webcams (especially **Logitech C310**, **C920**, and standard UVC webcams).

![Camera Quick Controls](data/icons/hicolor/scalable/apps/cam-controls.svg)

---

## What Problem Does This Solve?

### 1. Fixes Garbled / Sheared / Green Video on PipeWire
Recent Linux releases (Nobara, Fedora, Arch, Ubuntu 24.04+) with PipeWire and WirePlumber attempt to route standard USB webcams through `libcamera` instead of native `v4l2`. `libcamera`'s experimental UVC handler miscalculates frame buffer strides and capability ranges, producing **green tints, diagonal shear, scrambled analog-like static, or GStreamer range negotiation crashes** in GNOME Snapshot, browsers, and OBS.

`cam-controls` includes an automatic one-click WirePlumber fix that bypasses `libcamera` and restores PipeWire's rock-solid native V4L2 backend.

### 2. Fixes Washed-Out, Faded, and Pale Webcam Colors
Many USB webcams (particularly the Logitech C310) ship with factory defaults that set **Saturation** to only ~12% (`32` out of `255`) and **Contrast** to `32`, resulting in a gray, hazy, and washed-out image.

`cam-controls` provides simple visual sliders and a single **"Apply Recommended"** button to instantly restore vivid, natural colors, depth, and skin tones.

---

## Features

- **Blazing Native Rust Performance:** 100% compiled Rust with zero runtime dependencies. Instant startup (<15ms) and minimal RAM footprint.
- **Zero-Lock Async V4L2 Engine:** Hardware `ioctl` commands execute non-blockingly on an asynchronous worker thread with immediate handle release (`O_RDWR` -> `ioctl` -> `close`), completely avoiding USB EP0 deadlocks when external camera apps (Snapshot, OBS, Discord) are streaming simultaneously.
- **Native Wayland & X11 GUI:** Responsive hardware-accelerated UI that adapts seamlessly to desktop themes, with full Wayland application matching and high-resolution icons.
- **Smart Camera Switching:** Automatically discovers all connected video capture webcams (with hardware model names) and resolves symlinks to stable nodes.
- **Real-Time Sliders:** Adjust Saturation, Contrast, Brightness, Sharpness, Gain, Exposure Time, and White Balance Temperature live while watching your camera feed.
- **Lighting Anti-Flicker:** Instantly eliminates rolling horizontal light bands from indoor fluorescent and LED lighting (`50 Hz` or `60 Hz`).
- **Smart Exposure & White Balance:** One-click toggles for Auto White Balance and Auto Exposure, or unlock manual controls for studio environments.
- **Lighting Profiles & Presets:** Switch between pre-configured presets (*Recommended (Vivid)*, *Daylight / Office*, *Night / Warm Lamp*, *Cool / Studio*) or save your own custom profiles.
- **Persistent Startup Defaults:** Single-click **"Save as Default"** automatically restores your preferred camera settings on system login or when the camera is plugged in.
- **Dual PipeWire System Fixes:**
  - **Video Fix:** Bypasses experimental `libcamera` UVC pipeline to fix garbled, green, or diagonally sheared video.
  - **Audio Fix:** Resolves the notorious Logitech C310 16kHz microphone firmware reporting bug that causes fast/robotic "chipmunk" audio in Discord and Zoom.
- **Full CLI Support:** Completely scriptable and headless-friendly (`cam-controls --status`, `--apply-recommended`, `--restore`, `--save-default`).

---

## Installation

### Fedora / Nobara / RHEL / openSUSE (`.rpm`)

Install directly from terminal:

```bash
sudo dnf install https://github.com/mottipython/cam-controls/releases/download/v2.0.0/cam-controls-2.0.0-1.x86_64.rpm
```

### Debian / Ubuntu / Pop!_OS / Linux Mint (`.deb`)

Install directly from terminal:

```bash
curl -LO https://github.com/mottipython/cam-controls/releases/download/v2.0.0/cam-controls_2.0.0_amd64.deb
sudo dpkg -i cam-controls_2.0.0_amd64.deb || sudo apt-get install -f -y
```

### From Source

Requires standard `cargo` and `rustc`:

```bash
git clone https://github.com/mottipython/cam-controls.git
cd cam-controls
make
sudo make install
```

---

## Usage

### Graphical Interface (GUI)
Launch **Camera Quick Controls** from your application menu or run:
```bash
cam-controls
```

### Command Line Interface (CLI)
```bash
# List all detected video cameras
cam-controls --list-devices

# Print current camera settings
cam-controls --status

# Apply the recommended vivid color profile
cam-controls --apply-recommended

# Save current settings as default (and enable login autostart)
cam-controls --save-default

# Silently restore saved settings (used by login autostart)
cam-controls --restore

# Fix garbled/green video by bypassing libcamera in PipeWire
cam-controls --fix-pipewire

# Fix robotic/fast microphone audio (Logitech 16kHz bug)
cam-controls --fix-mic
```

---

## License

MIT License. See [LICENSE](LICENSE) for details.
