target/release/gradecalc:
	cargo build --release

.PHONY: target/release/gradecalc install uninstall

install: target/release/gradecalc
	mv target/release/gradecalc /usr/local/bin
	chmod 644 /usr/local/bin/gradecalc

uninstall: target/release/gradecalc
	rm -rf /usr/local/bin/gradecalc
