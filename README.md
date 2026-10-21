# Camera Quick Controls (`cam-controls`)

[![CI Package & Installation Tests](https://github.com/mottipython/cam-controls/actions/workflows/test.yml/badge.svg)](https://github.com/mottipython/cam-controls/actions/workflows/test.yml)
[![Release](https://img.shields.io/github/v/release/mottipython/cam-controls)](https://github.com/mottipython/cam-controls/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight, zero-dependency camera tuning utility and PipeWire fix for Linux webcams (especially **Logitech C310**, **C920**, and standard UVC webcams).

![Camera Quick Controls](data/icons/hicolor/scalable/apps/cam-controls.svg)

---

## What Problem Does This Solve?

### 1. Fixes Garbled / Sheared / Green Video on PipeWire
Recent Linux releases (Nobara, Fedora, Arch, Ubuntu 24.04+) with PipeWire and WirePlumber attempt to route standard USB webcams through `libcamera` instead of native `v4l2`. `libcamera`'s experimental UVC handler miscalculates frame buffer strides and capability ranges, producing **green tints, diagonal shear, scrambled analog-like static, or GStreamer range negotiation crashes** in GNOME Snapshot, browsers, and OBS.

`cam-controls` includes an automatic one-click WirePlumber fix that bypasses `libcamera` and restores PipeWire's rock-solid native V4L2 backend.

### 2. Fixes Washed-Out, Faded, and Pale Webcam Colors
Many USB webcams (particularly the Logitech C310) ship with factory defaults that set **Saturation** to only ~12% (`32` out of `255`) and **Contrast** to `32`, resulting in a gray, hazy, and washed-out image.

`cam-controls` provides simple TV-style visual sliders and a single **"Apply Recommended"** button to instantly restore vivid, natural colors, depth, and skin tones.

---

## Features

- **Zero Heavy Dependencies:** Written in standard Python with native Tkinter—no heavy desktop frameworks or runtimes required.
- **Smart Camera Switching:** Automatically detects all connected webcams (with hardware model names) and lets you switch between them on the fly.
- **Real-Time Sliders:** Adjust Saturation, Contrast, Brightness, Sharpness, Gain, Exposure Time, and White Balance Temperature live while watching your camera feed.
- **Lighting Anti-Flicker:** Instantly eliminates rolling horizontal light bands from indoor fluorescent and LED lighting (`50 Hz` or `60 Hz`).
- **Smart Exposure & White Balance:** One-click toggles for Auto White Balance and Auto Exposure, or unlock manual controls for studio environments.
- **Lighting Profiles & Presets:** Switch between pre-configured presets (*Recommended (Vivid)*, *Daylight / Office*, *Night / Warm Lamp*, *Cool / Studio*) or save your own custom profiles.
- **Persistent Startup Defaults:** Single-click **"Save as Default"** automatically restores your preferred camera settings on system login or when the camera is plugged in.
- **Dual PipeWire System Fixes:**
  - **Video Fix:** Bypasses experimental `libcamera` UVC pipeline to fix garbled, green, or diagonally sheared video.
  - **Audio Fix:** Resolves the notorious Logitech C310 16kHz microphone firmware reporting bug that causes fast/robotic "chipmunk" audio in Discord and Zoom.
- **Full CLI Support:** Completely scriptable and headless-friendly (`cam-controls --status`, `cam-controls --apply-recommended`, `cam-controls --restore`).

---

## Installation

### Fedora / Nobara / RHEL / openSUSE (`.rpm`)

Install directly from terminal:

```bash
sudo dnf install https://github.com/mottipython/cam-controls/releases/download/v1.0.0/cam-controls-1.0.0-1.noarch.rpm
```

### Debian / Ubuntu / Pop!_OS / Linux Mint (`.deb`)

Install directly from terminal:

```bash
curl -LO https://github.com/mottipython/cam-controls/releases/download/v1.0.0/cam-controls_1.0.0_all.deb
sudo dpkg -i cam-controls_1.0.0_all.deb || sudo apt-get install -f -y
```

### From Source

```bash
git clone https://github.com/mottipython/cam-controls.git
cd cam-controls
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

# Reset hardware to factory defaults
cam-controls --reset-defaults

# Target a specific camera by index or device path
cam-controls -d 0 --status
cam-controls -d /dev/video1 --apply-recommended
```

---

## Building Packages Locally

To build Debian (`.deb`) and RPM (`.rpm`) packages on your own machine:

```bash
# Build Debian package
make deb

# Build RPM package
make rpm

# Build both
make packages
```
The output packages will be created in the `dist/` folder.

---

## License

MIT License. See [LICENSE](LICENSE) for details.
