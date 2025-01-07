#![allow(dead_code, unused)]

mod vulkan;
mod window;
mod object;

use anyhow::{bail, Result};
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
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1])?;
    let object = match Object::from_str(&file) {
        Ok(object) => object,
        Err(_) => bail!("Failed to parse the obj file"),
    };

    Ok(())
}
