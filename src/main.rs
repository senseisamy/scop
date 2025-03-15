mod graphics;
mod math;
mod object_loader;

use anyhow::{anyhow, Result};
use graphics::App;
use object_loader::Object;
use std::env;
use std::fs;
use winit::event_loop::EventLoop;

const BG_COLOR: (f32, f32, f32) = (40.0, 40.0, 40.0);

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("This program expect at least one argument"));
    }

    let object = {
        let objfile = fs::read_to_string(&args[1])?;
        if args.len() >= 3 {
            let textfile = fs::read_to_string(&args[2])?;
            Object::parse(&objfile, Some(&textfile))?
        } else {
            Object::parse(&objfile, None)?
        }
    };

    let event_loop = EventLoop::new()?;
    let mut app = App::new(&event_loop, object)?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
