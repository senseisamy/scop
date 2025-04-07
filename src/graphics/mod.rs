pub mod app;
pub mod input;
pub mod view;

use crate::{
    math::Vec3,
    object_loader::{Object, Vertexxx},
    vec3,
};
use input::InputState;
use std::{sync::Arc, time::Instant};
use vulkano::{
    buffer::{allocator::SubbufferAllocator, BufferContents, Subbuffer}, command_buffer::allocator::StandardCommandBufferAllocator, descriptor_set::allocator::StandardDescriptorSetAllocator, device::{Device, Queue}, image::{sampler::Sampler, view::ImageView}, instance::Instance, memory::allocator::StandardMemoryAllocator, padded::Padded, pipeline::GraphicsPipeline, render_pass::{Framebuffer, RenderPass}, shader::EntryPoint, swapchain::Swapchain, sync::GpuFuture
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
    pub object: Object,
    pub texture: Arc<ImageView>,
    pub sampler: Arc<Sampler>,
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
    camera: Camera,
    light: Light,
    input_state: InputState,
    time_info: TimeInfo,
    use_texture: bool,
}

// data to be passed to the shaders
#[repr(C)]
#[derive(BufferContents, Copy, Clone)]
pub struct Data {
    world: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    proj: [[f32; 4]; 4],
    light_pos: Padded<[f32; 3], 4>,
    light_color: Padded<[f32; 3], 4>,
    ambient_light_color: [f32; 3],
    texture: u32
}

pub struct Camera {
    position: Vec3,
    target: Vec3,
    distance: f32,
    theta: f32, // horizontal angle
    phi: f32,   // vertical angle
}

pub struct Light {
    position: Vec3,
    pos_locked: bool,
    colors: Vec<Vec3>,
    color: (usize, f32),
    ambient_color: (usize, f32),
}

pub struct TimeInfo {
    time: Instant,
    dt: f32,
    dt_sum: f32,
    frame_count: f32,
    avg_fps: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: vec3!(0.0, 0.0, 0.0),
            target: vec3!(0.0, 0.0, 0.0),
            distance: 1.0,
            theta: std::f32::consts::FRAC_PI_2,
            phi: 0.0,
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        Self {
            position: vec3!(0.0, 0.0, 10.0),
            pos_locked: false,
            colors: vec![
                vec3!(1.0, 1.0, 1.0),
                vec3!(1.0, 0.0, 0.0),
                vec3!(0.0, 1.0, 0.0),
                vec3!(0.0, 0.0, 1.0),
                vec3!(1.0, 0.55294117647058823529, 0.63137254901960784313),
                vec3!(0.2941176471, 0.0, 0.5098039216),
            ],
            color: (0, 1.0),
            ambient_color: (0, 0.2),
        }
    }
}

impl Default for TimeInfo {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            dt: 0.0,
            dt_sum: 0.0,
            frame_count: 0.0,
            avg_fps: 0.0,
        }
    }
}
