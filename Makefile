all: build copy

release:
	@cargo build --release
	@ln -s -f target/release/scop ./scop

build:
	@cargo build

copy:
	@ln -s -f target/debug/scop ./scop

check:
	@cargo check

clean:
	@rm -f scop
	@cargo clean
