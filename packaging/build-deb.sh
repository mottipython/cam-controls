#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="1.2.2"
PKG_NAME="cam-controls"
BUILD_DIR="$DIR/build/deb"
OUTPUT_DIR="$DIR/dist"

rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/DEBIAN"
mkdir -p "$BUILD_DIR/usr/bin"
mkdir -p "$BUILD_DIR/usr/share/applications"
mkdir -p "$BUILD_DIR/usr/share/icons/hicolor/scalable/apps"
for size in 16 24 32 48 64 128 256; do
    mkdir -p "$BUILD_DIR/usr/share/icons/hicolor/${size}x${size}/apps"
done
mkdir -p "$BUILD_DIR/usr/share/cam-controls"
mkdir -p "$OUTPUT_DIR"

# Copy files
install -m 755 "$DIR/bin/cam-controls" "$BUILD_DIR/usr/bin/cam-controls"
install -m 644 "$DIR/data/cam-controls.desktop" "$BUILD_DIR/usr/share/applications/cam-controls.desktop"
install -m 644 "$DIR/data/icons/hicolor/scalable/apps/cam-controls.svg" "$BUILD_DIR/usr/share/icons/hicolor/scalable/apps/cam-controls.svg"
for size in 16 24 32 48 64 128 256; do
    install -m 644 "$DIR/data/icons/hicolor/${size}x${size}/apps/cam-controls.png" "$BUILD_DIR/usr/share/icons/hicolor/${size}x${size}/apps/cam-controls.png"
done
install -m 644 "$DIR/data/99-disable-libcamera.conf" "$BUILD_DIR/usr/share/cam-controls/99-disable-libcamera.conf"

# Generate control file
cat << EOF > "$BUILD_DIR/DEBIAN/control"
Package: $PKG_NAME
Version: $VERSION
Section: video
Priority: optional
Architecture: all
Depends: python3, python3-tk
Maintainer: Motti Schleider <motti.schleider@gmail.com>
Description: Camera Quick Controls & PipeWire video fix for Linux webcams
 Camera Quick Controls provides simple visual sliders for webcam saturation,
 contrast, brightness, sharpness, and white balance, with one-click fixes for
 washed-out colors and an automatic WirePlumber fix for garbled or green video
 on PipeWire systems.
EOF

DEB_FILE="$OUTPUT_DIR/${PKG_NAME}_${VERSION}_all.deb"

if command -v dpkg-deb >/dev/null 2>&1; then
    dpkg-deb --build --root-owner-group "$BUILD_DIR" "$DEB_FILE"
else
    echo "dpkg-deb not found, assembling .deb package using tar and ar..."
    TMP_PKG=$(mktemp -d)
    echo "2.0" > "$TMP_PKG/debian-binary"
    (cd "$BUILD_DIR/DEBIAN" && tar --owner=0 --group=0 -czf "$TMP_PKG/control.tar.gz" .)
    (cd "$BUILD_DIR" && tar --owner=0 --group=0 --exclude="./DEBIAN" -czf "$TMP_PKG/data.tar.gz" ./usr)
    (cd "$TMP_PKG" && ar rcs "$DEB_FILE" debian-binary control.tar.gz data.tar.gz)
    rm -rf "$TMP_PKG"
fi

echo "Created Debian package: $DEB_FILE"
