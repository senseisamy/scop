release:
	cargo build --release
	ln -s -f target/release/scop ./scop

debug:
	cargo build
	ln -s -f target/debug/scop ./scop

check:
	cargo check

clean:
	rm -f scop
	cargo clean

.PHONY: release debug check clean
.SILENT:
