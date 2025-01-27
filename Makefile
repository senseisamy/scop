all: build copy

build:
	@cargo build --release

copy:
	@cp target/release/scop .

check:
	@cargo check

clean:
	@rm -f scop
	@cargo clean
