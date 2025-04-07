mod graphics;
mod math;
mod object_loader;

use graphics::App;
use object_loader::texture::Texture;
use object_loader::Object;
use std::env;
use std::fs;
use winit::event_loop::EventLoop;

const BG_COLOR: (f32, f32, f32) = (40.0, 40.0, 40.0);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("This program expects 2 arguments");
    }

    let object = {
        let objfile = fs::read_to_string(&args[1]).expect("obj file not found");
        match Object::parse(&objfile) {
            Err(e) => panic!("failed to parse the obj file: {e}"),
            Ok(obj) => obj
        }
    };

    let texture = {
        let textfile = fs::read_to_string(&args[2]).expect("ppm file not found");
        match Texture::parse_ppm(&textfile) {
            Err(e) => panic!("failed to parse the ppm file: {e}"),
            Ok(tex) => tex
        }
    };

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(&event_loop, object, texture).unwrap();

    event_loop.run_app(&mut app).unwrap();
}
