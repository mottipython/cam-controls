#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="2.0.0"
OUTPUT_DIR="$DIR/dist"
RPMBUILD_DIR="$DIR/build/rpmbuild"

echo "Building release binary with cargo..."
(cd "$DIR" && cargo build --release)

rm -rf "$RPMBUILD_DIR"
mkdir -p "$RPMBUILD_DIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
mkdir -p "$OUTPUT_DIR"

# Copy spec
cp "$DIR/packaging/cam-controls.spec" "$RPMBUILD_DIR/SPECS/"

# Build RPM
rpmbuild -bb \
  --define "_topdir $RPMBUILD_DIR" \
  --define "_tmppath /tmp" \
  --define "_sourcedir $DIR" \
  "$RPMBUILD_DIR/SPECS/cam-controls.spec"

find "$RPMBUILD_DIR/RPMS" -name "*.rpm" -exec cp {} "$OUTPUT_DIR/" \;
echo "Created RPM package in $OUTPUT_DIR"
