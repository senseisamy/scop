pub mod graphics;
pub mod window;

use crate::object_loader::Position;
use std::sync::Arc;
use vulkano::{
    buffer::Subbuffer, command_buffer::allocator::StandardCommandBufferAllocator, device::{Device, Queue}, instance::Instance, memory::allocator::StandardMemoryAllocator, pipeline::GraphicsPipeline, render_pass::{Framebuffer, RenderPass}, shader::EntryPoint, swapchain::Swapchain, sync::GpuFuture
};
use winit::window::Window;

pub struct App {
    pub instance: Arc<Instance>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub vertex_buffer: Subbuffer<[Position]>,
    pub index_buffer: Subbuffer<[u16]>,
    pub rcx: Option<RenderContext>,
}

pub struct RenderContext {
    window: Arc<Window>,
    swapchain: Arc<Swapchain>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    vs: EntryPoint,
    fs: EntryPoint,
    pipeline: Arc<GraphicsPipeline>,
    recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
}
