release: shaders
	cargo build --release
	ln -s -f target/release/scop ./scop

debug: shaders
	cargo build
	ln -s -f target/debug/scop ./scop

shaders: src/shaders/vertex.spv src/shaders/fragment.spv

src/shaders/vertex.spv:
	glslangValidator src/shaders/vertex.glsl -V -S vert -o src/shaders/vertex.spv

src/shaders/fragment.spv:
	glslangValidator src/shaders/fragment.glsl -V -S frag -o src/shaders/fragment.spv

check:
	cargo check

clean:
	rm -f scop src/shaders/vertex.spv src/shaders/fragment.spv
	cargo clean

.PHONY: release debug check clean shaders
.SILENT:
