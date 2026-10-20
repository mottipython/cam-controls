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

- **Zero Heavy Dependencies:** Written in standard Python with native Tkinter—no heavy desktop frameworks required.
- **Real-Time Sliders:** Adjust Saturation, Contrast, Brightness, Sharpness, Gain, and White Balance Temperature live while watching your camera feed.
- **One-Click Presets:** "Apply Recommended" (vivid, natural profile) and "Reset to Defaults".
- **One-Click PipeWire Fix:** Resolves the `libcamera` UVC stride/green screen bug instantly.
- **Full CLI Support:** Can also be used headless or in scripts (e.g. `cam-controls --apply-recommended`).

---

## Installation

### Debian / Ubuntu / Pop!_OS / Linux Mint (`.deb`)

Download the latest `.deb` from the [GitHub Releases](https://github.com/mottipython/cam-controls/releases) page and install:

```bash
sudo dpkg -i cam-controls_1.0.0_all.deb || sudo apt-get install -f -y
```

### Fedora / Nobara / RHEL / openSUSE (`.rpm`)

Download the latest `.rpm` from the [GitHub Releases](https://github.com/mottipython/cam-controls/releases) page and install:

```bash
sudo dnf install ./cam-controls-1.0.0-1.*.noarch.rpm
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
# Print current camera settings
cam-controls --status

# Apply the recommended natural color profile
cam-controls --apply-recommended

# Reset hardware to factory defaults
cam-controls --reset-defaults

# Apply the WirePlumber PipeWire fix
cam-controls --fix-pipewire

# Target a specific video device
cam-controls -d /dev/video1 --status
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
