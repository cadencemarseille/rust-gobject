.PHONY: clean install test

clean:
	$(RM) -r .rust bin build lib

install:
	rustpkg build glib
	rustpkg install gobject
	rustpkg install gi
	rustpkg install rustgigen
