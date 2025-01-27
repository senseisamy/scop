all: build copy

build:
	@cargo build --release

copy:
	@ln -s -f target/release/scop ./scop

check:
	@cargo check

clean:
	@rm -f scop
	@cargo clean
