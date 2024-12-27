#![allow(dead_code, unused)]

mod app;
mod object;

use anyhow::{bail, Result};
use log::*;
use app::App;
use object::Object;
use std::env;
use std::fs;
use std::str::FromStr;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;

fn main() -> Result<()> {
    pretty_env_logger::init();

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
