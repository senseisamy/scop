use crate::vulkan::App;
use anyhow::{Context, Result};
use std::sync::Arc;
use vulkano::{
    device, image::{Image, ImageUsage}, render_pass::RenderPass, swapchain::{self, Surface, Swapchain, SwapchainCreateInfo}
};
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event_loop::ActiveEventLoop, window::Window,
};

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let surface = Surface::from_window(self.instance.clone(), window.clone())
            .expect("Could not create surface");
        let window_size = window.inner_size();
        let (swapchain, images) = self
            .create_swapchain(surface.clone(), window_size)
            .expect("Could not create swapchain");

        mod vs {
            vulkano_shaders::shader! {
                ty: "vertex",
                src: r"
                    #version 450

                    layout(location = 0) in vec2 position;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                    }
                ",
            }
        }

        mod fs {
            vulkano_shaders::shader! {
                ty: "fragment",
                src: r"
                    #version 450

                    layout(location = 0) out vec4 f_color;

                    void main() {
                        f_color = vec4(1.0, 0.0, 0.0, 1.0);
                    }
                ",
            }
        }

        let render_pass = self.create_render_pass(&swapchain)
            .expect("failed to create render pass");
        let framebuffers = window_size_dependent_setup(&images, &render_pass);
    }
}

impl App {
    fn create_swapchain(
        &mut self,
        surface: Arc<Surface>,
        window_size: PhysicalSize<u32>,
    ) -> Result<(Arc<Swapchain>, Vec<Arc<Image>>)> {
        let surface_capabilities = self
            .device
            .physical_device()
            .surface_capabilities(&surface, Default::default())?;

        let (image_format, _) = self
            .device
            .physical_device()
            .surface_formats(&surface, Default::default())?[0];

        let swapchain = Swapchain::new(
            self.device.clone(),
            surface,
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count.max(2),
                image_format,
                image_extent: window_size.into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: surface_capabilities
                    .supported_composite_alpha
                    .into_iter()
                    .next()
                    .context("failed to get surface supported composite alpha")?,
                ..Default::default()
            },
        )?;

        Ok(swapchain)
    }

    fn create_render_pass(self, swapchain: &Swapchain) -> Result<Arc<RenderPass>> {
        let render_pass = vulkano::single_pass_renderpass!(
            self.device.clone(),
            attachments: {
                color: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )?;

        Ok(render_pass)
    }
}
