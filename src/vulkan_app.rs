use std::default;

use anyhow::{anyhow, Result};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use vulkanalia::loader::{self, LibloadingLoader, LIBRARY};
use vulkanalia::window as vk_window;
use vulkanalia::prelude::v1_3::*;

/// Our Vulkan and winit app.
#[derive(Debug)]
pub struct App {
    window: Option<Window>,
	entry: Entry,
	instance: Instance
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes()
					.with_title("scop")
					.with_inner_size(LogicalSize::new(1024, 768))
				)
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.
				unsafe {self.render()}.unwrap();

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl App {
    /// Creates our Vulkan app.
    unsafe fn create() -> Result<Self> {
		let window = ;
        let loader = LibloadingLoader::new(LIBRARY)?;
		let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
		let instance = create_instance(window, &entry)?;
		Ok(Self{
			window,
			entry,
			instance
		})
    }

    /// Renders a frame for our Vulkan app.
    unsafe fn render(&mut self) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    unsafe fn destroy(&mut self) {}
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}

unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
	let application_info = vk::ApplicationInfo::builder()
		.application_name(b"scop\0")
		.application_version(vk::make_version(1, 0, 0))
		.engine_name(b"No Engine\0")
		.engine_version(vk::make_version(1, 0, 0))
		.api_version(vk::make_version(1, 0, 0));

	let extensions = vk_window::get_required_instance_extensions(window)
		.iter()
		.map(|e| e.as_ptr())
		.collect::<Vec<_>>();

	let info = vk::InstanceCreateInfo::builder()
		.application_info(&application_info)
		.enabled_extension_names(&extensions);

	Ok(entry.create_instance(&info, None)?)
}