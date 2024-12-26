#![allow(dead_code, unused)]

mod winit_app;
mod vulkan_app;
mod object;

use anyhow::{bail, Result};
use std::env;
use std::fs;
use std::str::FromStr;
use winit::event_loop::ControlFlow;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;
use object::Object;
use vulkan_app::App;

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1])?;
    let object = match Object::from_str(&file) {
        Ok(object) => object,
        Err(_) => bail!("Failed to parse the obj file"),
    };

    let event_loop = EventLoop::new()?;
	event_loop.set_control_flow(ControlFlow::Poll);
	let mut app = App::default();
	event_loop.run_app(&mut app)?;

    Ok(())
}
