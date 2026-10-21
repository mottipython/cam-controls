Name:           cam-controls
Version:        1.2.2
Release:        1%{?dist}
Summary:        Camera Quick Controls & PipeWire fix for Linux webcams (Python Edition)
License:        MIT
URL:            https://github.com/mottipython/cam-controls
BuildArch:      noarch

Requires:       python3
Requires:       python3-tkinter

%description
Universal Python edition of Camera Quick Controls for Linux webcams.

%prep

%build

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/scalable/apps
for size in 16 24 32 48 64 128 256; do
    mkdir -p %{buildroot}%{_datadir}/icons/hicolor/${size}x${size}/apps
done
mkdir -p %{buildroot}%{_datadir}/cam-controls

install -m 755 %{_sourcedir}/python/cam-controls %{buildroot}%{_bindir}/cam-controls
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
