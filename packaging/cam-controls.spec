Name:           cam-controls
Version:        2.0.0
Release:        1%{?dist}
Summary:        High-performance camera image tuning & PipeWire fix for Linux
License:        MIT
URL:            https://github.com/mottipython/cam-controls

%description
Camera Quick Controls (Rust Edition) is a high-performance, native Linux
camera control utility and PipeWire fix. Features instant V4L2 ioctl control,
custom profiles, startup persistence, and one-click fixes for video distortion
and USB microphone sample rate issues.

%prep
# Nothing to prep

%build
# Binary is compiled before packaging

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/scalable/apps
for size in 16 24 32 48 64 128 256; do
    mkdir -p %{buildroot}%{_datadir}/icons/hicolor/${size}x${size}/apps
done
mkdir -p %{buildroot}%{_datadir}/cam-controls

install -m 755 %{_sourcedir}/target/release/cam-controls %{buildroot}%{_bindir}/cam-controls
install -m 644 %{_sourcedir}/data/cam-controls.desktop %{buildroot}%{_datadir}/applications/cam-controls.desktop
install -m 644 %{_sourcedir}/data/icons/hicolor/scalable/apps/cam-controls.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/cam-controls.svg
for size in 16 24 32 48 64 128 256; do
    install -m 644 %{_sourcedir}/data/icons/hicolor/${size}x${size}/apps/cam-controls.png %{buildroot}%{_datadir}/icons/hicolor/${size}x${size}/apps/cam-controls.png
done
install -m 644 %{_sourcedir}/data/99-disable-libcamera.conf %{buildroot}%{_datadir}/cam-controls/99-disable-libcamera.conf

%files
%{_bindir}/cam-controls
%{_datadir}/applications/cam-controls.desktop
%{_datadir}/icons/hicolor/scalable/apps/cam-controls.svg
%{_datadir}/icons/hicolor/*/apps/cam-controls.png
%{_datadir}/cam-controls/99-disable-libcamera.conf

%changelog
* Tue Oct 20 2026 Motti Schleider <motti.schleider@gmail.com> - 2.0.0-1
- Complete rewrite in Rust for blazing speed, zero overhead, and native Wayland/X11 GUI
