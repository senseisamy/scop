use super::App;
use crate::object_loader::{texture::Texture, Object, Vertexxx};
use anyhow::{Context, Result};
use std::{clone, sync::Arc};
use vulkano::{
    buffer::{
        allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo},
        Buffer, BufferCreateInfo, BufferUsage, Subbuffer,
    }, command_buffer::{allocator::{CommandBufferAllocator, StandardCommandBufferAllocator}, AutoCommandBufferBuilder}, descriptor_set::allocator::StandardDescriptorSetAllocator, device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue,
        QueueCreateInfo, QueueFlags,
    }, format::Format, image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator}, render_pass::RenderPass, swapchain::{Surface, Swapchain, SwapchainCreateInfo}, DeviceSize, VulkanLibrary
};
use winit::{dpi::PhysicalSize, event_loop::EventLoop};

impl App {
    pub fn new(event_loop: &EventLoop<()>, object: Object, texture: Option<Texture>) -> Result<Self> {
        // load the vulkan library and create an instance of it
        let library = VulkanLibrary::new()?;
        let instance = create_instance(library, event_loop)?;

        // selecting a physical device (eg. graphic card) and creating a Device and a queue from it that we will use to do all future operations
        let (device, queue) = create_device(instance.clone(), event_loop)?;

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            Default::default(),
        ));

        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));

        let uniform_buffer_allocator = SubbufferAllocator::new(
            memory_allocator.clone(),
            SubbufferAllocatorCreateInfo {
                buffer_usage: BufferUsage::UNIFORM_BUFFER,
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
        );

        let (vertex_buffer, index_buffer) = create_buffers(&memory_allocator, &object)?;

        let rcx = None;

        Ok(Self {
            instance,
            device,
            queue,
            memory_allocator,
            descriptor_set_allocator,
            command_buffer_allocator,
            uniform_buffer_allocator,
            vertex_buffer,
            index_buffer,
            object,
            rcx,
        })
    }

    pub fn create_swapchain(
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

    pub fn create_render_pass(&self, swapchain: &Swapchain) -> Result<Arc<RenderPass>> {
        let render_pass = vulkano::single_pass_renderpass!(
            self.device.clone(),
            attachments: {
                color: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
                depth_stencil: {
                    format: Format::D16_UNORM,
                    samples: 1,
                    load_op: Clear,
                    store_op: DontCare,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {depth_stencil},
            },
        )?;

        Ok(render_pass)
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

fn create_buffers(
    memory_allocator: &Arc<StandardMemoryAllocator>,
    object: &Object,
) -> Result<(Subbuffer<[Vertexxx]>, Subbuffer<[u32]>)> {
    let vertex_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        object.vertex.clone(),
    )?;

    let index_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::INDEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        object.indice.clone(),
    )?;

    Ok((vertex_buffer, index_buffer))
}

fn create_texture_image_view(
    texture: Texture,
    memory_allocator: &Arc<StandardMemoryAllocator>,
    command_buffer_allocator: &Arc<StandardCommandBufferAllocator>
) -> Result<Arc<ImageView>> {
    let mut uploads = AutoCommandBufferBuilder::primary(
        command_buffer_allocator.clone(),
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    let format = Format::R8G8B8_SRGB;
    let extent: [u32; 3] = [texture.width, texture.height, 1];

    let upload_buffer = Buffer::new_slice(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        (texture.width * texture.height * 3) as DeviceSize
    )?;

    let a: &[u8] = &mut upload_buffer.write().unwrap();
    a = texture.data.clone().as_slice();

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8_SRGB,
            extent,
            usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
            ..Default::default()
        },
        AllocationCreateInfo::default()
    )?;


}
