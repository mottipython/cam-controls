PREFIX ?= /usr
DESTDIR ?=

all:
	cargo build --release

install:
	install -d $(DESTDIR)$(PREFIX)/bin
	install -d $(DESTDIR)$(PREFIX)/share/applications
	install -d $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps
	install -d $(DESTDIR)$(PREFIX)/share/cam-controls

	install -m 755 target/release/cam-controls $(DESTDIR)$(PREFIX)/bin/cam-controls
	install -m 644 data/cam-controls.desktop $(DESTDIR)$(PREFIX)/share/applications/cam-controls.desktop
	install -m 644 data/icons/hicolor/scalable/apps/cam-controls.svg $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps/cam-controls.svg
	for size in 16 24 32 48 64 128 256; do \
		install -d $(DESTDIR)$(PREFIX)/share/icons/hicolor/$${size}x$${size}/apps; \
		install -m 644 data/icons/hicolor/$${size}x$${size}/apps/cam-controls.png $(DESTDIR)$(PREFIX)/share/icons/hicolor/$${size}x$${size}/apps/cam-controls.png; \
	done
	install -m 644 data/99-disable-libcamera.conf $(DESTDIR)$(PREFIX)/share/cam-controls/99-disable-libcamera.conf

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/cam-controls
	rm -f $(DESTDIR)$(PREFIX)/share/applications/cam-controls.desktop
	rm -f $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps/cam-controls.svg
	for size in 16 24 32 48 64 128 256; do \
		rm -f $(DESTDIR)$(PREFIX)/share/icons/hicolor/$${size}x$${size}/apps/cam-controls.png; \
	done
	rm -rf $(DESTDIR)$(PREFIX)/share/cam-controls

deb:
	bash packaging/build-deb.sh

rpm:
	bash packaging/build-rpm.sh

deb-python:
	bash packaging/build-deb-python.sh

rpm-python:
	bash packaging/build-rpm-python.sh

packages: deb rpm deb-python rpm-python

clean:
	cargo clean
	rm -rf build dist

.PHONY: all install uninstall deb rpm deb-python rpm-python packages clean
