target/release/gradecalc:
	cargo build --release

.PHONY: target/release/gradecalc install uninstall

INSTALL_DIR=/usr/local/bin

install: target/release/gradecalc
	mv target/release/gradecalc $(INSTALL_DIR)
	chmod 755 $(INSTALL_DIR)/gradecalc

uninstall:
	rm -rf $(INSTALL_DIR)/gradecalc
