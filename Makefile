target/release/gradecalc:
	cargo build --release

.PHONY: target/release/gradecalc install uninstall

install: target/release/gradecalc
	mv target/release/gradecalc /usr/local/bin
	chmod 755 /usr/local/bin/gradecalc

uninstall:
	rm -rf /usr/local/bin/gradecalc
