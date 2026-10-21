Name:           cam-controls
Version:        1.1.1
Release:        1%{?dist}
Summary:        Camera Quick Controls & PipeWire video fix for Linux webcams
License:        MIT
URL:            https://github.com/mottipython/cam-controls
BuildArch:      noarch

Requires:       python3
Requires:       python3-tkinter

%description
Camera Quick Controls provides simple, visual sliders for webcam saturation,
contrast, brightness, sharpness, and white balance, with one-click fixes for
washed-out colors (especially on Logitech C310/C920 webcams) and an automatic
WirePlumber fix for garbled or green video on PipeWire systems.

%prep
# Nothing to prep

%build
# Nothing to build

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/scalable/apps
for size in 16 24 32 48 64 128 256; do
    mkdir -p %{buildroot}%{_datadir}/icons/hicolor/${size}x${size}/apps
done
mkdir -p %{buildroot}%{_datadir}/cam-controls

install -m 755 %{_sourcedir}/bin/cam-controls %{buildroot}%{_bindir}/cam-controls
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
* Tue Oct 20 2026 Motti Schleider <motti.schleider@gmail.com> - 1.1.1-1
- Modern webcam icon and embedded titlebar icon
