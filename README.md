# SCOP
A simple .obj 3D viewer with Vulkan written in rust


## Compilation

### With make

Simply run `make release`

### Without make

You'll need to compile the shaders in spv before running cargo  
Run `cargo build -r`. The executable will be in ./target/release


## Usage

### Starting the program

`./scop object.obj (texture.ppm)`  

### Keybinds

| Key    | Description        |
|--------|--------------------|
| W      | Zoom in            |
| S      | Zoom out           |
| A      | Rotate left        |
| D      | Rotate right       |
| Space  | Move up            |
| Shift  | Move down          |
| L      | Lock/unlock light  |
| C      | Change light color |
| T      | Toggle texture     |
| Escape | Quit               |

You can also use the mouse to rotate and zoom in/out