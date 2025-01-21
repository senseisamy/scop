all: build copy

build:
	@cargo build --release

copy:
	@cp target/release/scop .

clean:
	@rm -f scop
	@cargo clean
