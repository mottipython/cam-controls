PREFIX ?= /usr
DESTDIR ?=

all:
	@echo "Run 'make install' to install on your system"
	@echo "Run 'make deb' to build Debian package (.deb)"
	@echo "Run 'make rpm' to build RPM package (.rpm)"

install:
	install -d $(DESTDIR)$(PREFIX)/bin
	install -d $(DESTDIR)$(PREFIX)/share/applications
	install -d $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps
	install -d $(DESTDIR)$(PREFIX)/share/cam-controls

	install -m 755 bin/cam-controls $(DESTDIR)$(PREFIX)/bin/cam-controls
	install -m 644 data/cam-controls.desktop $(DESTDIR)$(PREFIX)/share/applications/cam-controls.desktop
	install -m 644 data/icons/hicolor/scalable/apps/cam-controls.svg $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps/cam-controls.svg
	install -m 644 data/99-disable-libcamera.conf $(DESTDIR)$(PREFIX)/share/cam-controls/99-disable-libcamera.conf

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/cam-controls
	rm -f $(DESTDIR)$(PREFIX)/share/applications/cam-controls.desktop
	rm -f $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps/cam-controls.svg
	rm -rf $(DESTDIR)$(PREFIX)/share/cam-controls

deb:
	bash packaging/build-deb.sh

rpm:
	bash packaging/build-rpm.sh

packages: deb rpm

clean:
	rm -rf build dist

.PHONY: all install uninstall deb rpm packages clean
