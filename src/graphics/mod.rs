pub mod graphics;
pub mod view;
pub mod window;
pub mod input;

use crate::{
    math::{Mat4, Vec3},
    object_loader::Vertexxx,
};
use std::{sync::Arc, time::Instant};
use input::InputState;
use vulkano::{
    buffer::{allocator::SubbufferAllocator, Subbuffer},
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::{Device, Queue},
    instance::Instance,
    memory::allocator::StandardMemoryAllocator,
    pipeline::GraphicsPipeline,
    render_pass::{Framebuffer, RenderPass},
    shader::EntryPoint,
    swapchain::Swapchain,
    sync::GpuFuture,
};
use winit::window::Window;

pub struct App {
    pub instance: Arc<Instance>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub uniform_buffer_allocator: SubbufferAllocator,
    pub vertex_buffer: Subbuffer<[Vertexxx]>,
    pub index_buffer: Subbuffer<[u32]>,
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
    world: View,
    input_state: InputState,
    time: Instant,
    dt: f32,
    dt_sum: f32,
    frame_count: f32,
    avg_fps: f32,
}

pub struct View {
    world_transformation: Mat4,
    camera: Camera,
}

pub struct Camera {
    position: Vec3,
    direction: Vec3,
}
