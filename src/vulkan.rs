use anyhow::{Context, Result};
use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::allocator::StandardCommandBufferAllocator,
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue,
        QueueCreateInfo, QueueFlags,
    },
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{vertex_input::Vertex, viewport::Viewport},
        GraphicsPipeline,
    },
    render_pass::{Framebuffer, RenderPass},
    swapchain::{Surface, Swapchain},
    sync::GpuFuture,
    VulkanLibrary,
};
use winit::{event_loop::EventLoop, window::Window};

pub struct App {
    pub instance: Arc<Instance>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub vertex_buffer: Subbuffer<[MyVertex]>,
    pub rcx: Option<RenderContext>,
}

struct RenderContext {
    window: Arc<Window>,
    swapchain: Arc<Swapchain>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    pipeline: Arc<GraphicsPipeline>,
    viewport: Viewport,
    recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
}

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        let library = VulkanLibrary::new()?;
        let instance = create_instance(library, event_loop)?;
        let (device, queue) = create_device(instance.clone(), event_loop)?;
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));
        let vertex_buffer = create_vertex_buffer(device.clone())?;
        let rcx = None;

        Ok(Self {
            instance,
            device,
            queue,
            command_buffer_allocator,
            vertex_buffer,
            rcx,
        })
    }
}

fn create_instance(
    library: Arc<VulkanLibrary>,
    event_loop: &EventLoop<()>,
) -> Result<Arc<Instance>> {
    let required_extensions = Surface::required_extensions(event_loop)?;

    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )?;

    Ok(instance)
}

fn create_device(
    instance: Arc<Instance>,
    event_loop: &EventLoop<()>,
) -> Result<(Arc<Device>, Arc<Queue>)> {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()?
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.presentation_support(i as u32, event_loop).unwrap()
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .context("no suitable physical device found")?;

    println!(
        "Using physical device: {} (type: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type
    );

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )?;

    let queue = queues.next().context("could not create a queue")?;

    Ok((device, queue))
}

fn create_vertex_buffer(device: Arc<Device>) -> Result<Subbuffer<[MyVertex]>> {
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device));

    let vertices = [
        MyVertex {
            position: [-0.5, -0.25],
        },
        MyVertex {
            position: [0.0, 0.5],
        },
        MyVertex {
            position: [0.25, -0.1],
        },
    ];

    let vertex_buffer = Buffer::from_iter(
        memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        vertices,
    )?;

    Ok(vertex_buffer)
}
