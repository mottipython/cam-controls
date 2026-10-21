#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="1.2.2"
OUTPUT_DIR="$DIR/dist"
RPMBUILD_DIR="$DIR/build/rpmbuild-python"

rm -rf "$RPMBUILD_DIR"
mkdir -p "$RPMBUILD_DIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
mkdir -p "$OUTPUT_DIR"

# Copy spec
cp "$DIR/packaging/cam-controls-python.spec" "$RPMBUILD_DIR/SPECS/"

# Build RPM
rpmbuild -bb \
  --define "_topdir $RPMBUILD_DIR" \
  --define "_tmppath /tmp" \
  --define "_sourcedir $DIR" \
  --nodeps \
  "$RPMBUILD_DIR/SPECS/cam-controls-python.spec"

find "$RPMBUILD_DIR/RPMS" -name "*.rpm" -exec cp {} "$OUTPUT_DIR/" \;
echo "Created Python RPM package in $OUTPUT_DIR"
