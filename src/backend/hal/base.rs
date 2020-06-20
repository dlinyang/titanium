use log::{debug, error, info, trace, warn};
use std::cell::RefCell;
use std::rc::Rc;
use std::mem::size_of;
use std::{fs,iter,ptr};

#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;

use gfx_hal::{
    buffer,
    command,
    format,
    image,
    memory,
    pass,
    pool,
    pso,
    window::Extent2D,
    Adapter,
    Backend,
    CommandPool,
    DescriptorPool,
    Device,
    Instance,
    Limits,
    MemoryType,
    PhysicalDevice,
    Primitive,
    QueueGroup,
    Surface,
    Swapchain,
    SwapchainConfig,
    General,
    Graphics,
    Compute,
};

use gfx_hal::command::{ClearColor,ClearValue};
use gfx_hal::format::{AsFormat,ChannelType, Rgba8Srgb, Swizzle};
use gfx_hal::pass::Subpass;
use gfx_hal::image::{Access,Layout,SubresourceRange, ViewKind};
use gfx_hal::pso::{PipelineStage, ShaderStageFlags, VertexInputRate, BlendState, ColorBlendDesc, ColorMask, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet, Rasterizer, Rect, Viewport};
use gfx_hal::queue::{Submission,QueueType};

use winit::{Event,WindowEvent,ControlFlow};
use super::window::WindowState;

use super::geometry::Vertex;

const COLOR_RANGE: image::SubresourceRange = image::SubresourceRange {
    aspects: format::Aspects::COLOR,
    levels: 0 .. 1,
    layers: 0 .. 1,
};

struct InitState<B: Backend> {
    instance: back::Instance,
    surface: B::Surface,
    adapter: AdapterState<B>,
}

impl InitState<back::Backend> {
    //Graphics or General supports
    fn new(windowState: &WindowState) -> Self {
        let instance = back::Instance::create("roaster", 1);
        let surface = instance.create_surface(&windowState.window);
        let mut adapters = instance.enumerate_adapters();
        (
            InitState{
                adapter: AdapterState::default(&mut adapters),
                surface,
                instance,
            }
        )
    }

    pub fn adapters_info(&self) {
        let adapters = self.instance.enumerate_adapters();

        for adapter in adapters {
            println!("{:?}", adapter.info);
        }
    }
    // only compute
}

struct AdapterState<B: Backend>{
    adapter: Option<Adapter<B>>,
    memory_types: Vec<MemoryType>,
    limits: Limits,
}

impl<B:Backend> AdapterState<B> {
    //choose adapter
    fn new(adapters: &mut Vec<Adapter<B>>,num: usize) -> Self {
        //choose the adapter
        let adapter = adapters.remove(num);
        let memory_types = adapter.physical_device.memory_properties().memory_types;
        let limits = adapter.physical_device.limits();

        AdapterState{
            adapter: Some(adapter),
            memory_types,
            limits,
        }
    }

    //choose the first adapter
    fn default(adapters: &mut Vec<Adapter<B>>) -> Self{
        Self::new(adapters, 0)
    }

    fn limits_info(&self) {
        println!("{:?}", self.limits)
    }
}

struct DeviceState<B: Backend> {
    device: B::Device,
    physical_device: B::PhysicalDevice,
    queue_group: QueueGroup<B, Graphics>,
}

impl<B: Backend> DeviceState<B> {
    //only support graphics
    fn new(adapter: Adapter<B>, surface: &B::Surface) -> Self{
        let (device, queue_group) = adapter
            .open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
            .unwrap();
    
    DeviceState {
        device,
        queue_group,
        physical_device: adapter.physical_device,
        }
    }
}

struct RenderPassState<B: Backend> {
    render_pass: Option<B::RenderPass>,
    device: Rc<RefCell<DeviceState<B>>>,
}

impl<B: Backend> RenderPassState<B> {
    unsafe fn new(swapchain: &SwapchainState<B>, device: Rc<RefCell<DeviceState<B>>>) -> Self {
        let render_pass = {
            let attachment = pass::Attachment {
                format: Some(swapchain.format.clone()),
                samples: 1,
                ops: pass::AttachmentOps::new(
                    pass::AttachmentLoadOp::Clear,
                    pass::AttachmentStoreOp::Store,
                ),
                stencil_ops: pass::AttachmentOps::DONT_CARE,
                layouts: image::Layout::Undefined .. image::Layout::Present,
            };

            let subpass = pass::SubpassDesc {
                colors: &[(0, image::Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            let dependency = pass::SubpassDependency {
                passes: pass::SubpassRef::External .. pass::SubpassRef::Pass(0),
                stages: pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT
                    .. pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
                accesses: image::Access::empty()
                    .. (image::Access::COLOR_ATTACHMENT_READ | image::Access::COLOR_ATTACHMENT_WRITE),
            };

            device
                .borrow()
                .device
                .create_render_pass(&[attachment], &[subpass], &[dependency])
                .ok()
        };

        RenderPassState {
            render_pass,
            device,
        }
    }
}

impl<B: Backend> Drop for RenderPassState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_render_pass(self.render_pass.take().unwrap());
        }
    }
}

struct SwapchainState<B: Backend> {
    swapchain: Option<B::Swapchain>,
    backbuffer: Option<Vec<B::Image>>,
    device: Rc<RefCell<DeviceState<B>>>,
    extent: image::Extent,
    format: format::Format,
}

impl<B: Backend> SwapchainState<B> {
    unsafe fn new(backend: &mut InitState<B>,device: Rc<RefCell<DeviceState<B>>>) -> Self{
        let (caps, formats, _present_modes) = backend
            .surface
            .compatibility(&device.borrow().physical_device);

        let format = formats.map_or(format::Format::Rgba8Srgb, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .map(|format| *format)
                .unwrap_or(formats[0])
        });

        let swap_config = SwapchainConfig::from_caps(&caps, format, Extent2D{ width: 1024, height: 768});
        let extent = swap_config.extent.to_extent();
        let (swapchain, backbuffer) = device
            .borrow()
            .device
            .create_swapchain(&mut backend.surface, swap_config, None)
            .expect("Can't create swapchain");

        SwapchainState {
            swapchain: Some(swapchain),
            backbuffer: Some(backbuffer),
            device,
            extent,
            format,
        }
    }

    fn format_info(&self) {
        println!("{:?}", self.format)
    }
}

impl<B: Backend> Drop for SwapchainState<B> {
    fn drop(&mut self) {
        unsafe {
            self.device
                .borrow()
                .device
                .destroy_swapchain(self.swapchain.take().unwrap());
        }
    }
}

struct PipelineState<B: Backend> {
    pipeline: Option<B::GraphicsPipeline>,
    pipeline_layout: Option<B::PipelineLayout>,
    device: Rc<RefCell<DeviceState<B>>>,
}

impl<B: Backend> PipelineState<B> {
    unsafe fn new<IS>(
        desc_layouts: IS,
        render_pass: &B::RenderPass,
        device_ptr: Rc<RefCell<DeviceState<B>>>,
    ) -> Self
    where
        IS: IntoIterator,
        IS::Item: std::borrow::Borrow<B::DescriptorSetLayout>,
    {
        let device = &device_ptr.borrow().device;
        let pipeline_layout = device
            .create_pipeline_layout(desc_layouts, &[(pso::S)])
            .expect("Can't create pipeline layout");

        let pipeline = {
            let vertex_shader = {
                let spirv = pso::read_spirv(include_bytes!("shader/ui.vert.spv")).unwrap();
                device.create_shader_module(&spriv).unwrap()
            };

            let fragment_shader = {
                let spirv = include_bytes!("shader/ui.frag.spv");
                device.create_shader_module(spirv).unwrap()
            };

            let pipeline = {
                let (vs_entry, fs_entry) = (
                    pso::EntryPoint::<B>{
                        entry: "main",
                        module: &vertex_shader,
                        specialization: gfx_hal::spec_const_list![0.8f32],
                    },
                    pso::EntryPoint::<B>{
                        entry: "main",
                        module: &fragment_shader,
                        specialization: pso::Specialization::default(),
                    },
                );

                let shader_entries = GraphicsShaderSet{
                    vertex: vs_entry,
                    hull: None,
                    domain: None,
                    geometry: None,
                    fragment: Some(fs_entry),
                };

                let subpass = Subpass{
                    index: 0,
                    main_pass: render_pass,
                };

                let mut pipeline_desc = GraphicsPipelineDesc::new(
                    shader_entries,
                    Primitive::TriangleList,
                    Rasterizer::FILL,
                    &pipeline_layout,
                    subpass,
                );

                pipeline_desc.blender.targets.push(ColorBlendDesc{
                    mask: pso::ColorMask::ALL,
                    blend: Some(pso::BlendState::ALPHA),
                });

                //vertex 
                pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc{
                    binding: 0,
                    stride: size_of::<Vertex>() as u32,
                    rate: pso::VertexInputRate::Vertex,
                });

                pipeline_desc.attributes.push(pso::AttributeDesc {
                    location: 0,
                    binding: 0,
                    element: pso::Element {
                        format: format::Format::Rg32Sfloat,
                        offset: 0,
                    },
                });

                pipeline_desc.attributes.push(pso::AttributeDesc {
                    location: 1,
                    binding: 0,
                    element: pso::Element {
                        format: format::Format::Rg32Sfloat,
                        offset: 8,
                    },
                });

                device.create_graphics_pipeline(&pipeline_desc,None)
            };

        device.destroy_shader_module(vertex_shader);
        device.destroy_shader_module(fragment_shader);

        pipeline.unwrap()
    };

     PipelineState {
            pipeline: Some(pipeline),
            pipeline_layout: Some(pipeline_layout),
            device: Rc::clone(&device_ptr),
        }
    }
}

struct BufferState<B: Backend> {
    memory: Option<B::Memory>,
    buffer: Option<B::Buffer>,
    device: Rc<RefCell<DeviceState<B>>>,
    size: u64,
}

impl<B: Backend> BufferState<B> {
    fn get_buffer(&self) -> &B::Buffer {
        self.buffer.as_ref().unwrap()
    }

    unsafe fn new<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        data_source: &[T],
        usage: buffer::Usage,
        memory_types: &[MemoryType],
    ) -> Self
    where
        T: Copy,
    {
        let memory: B::Memory;
        let mut buffer: B::Buffer;
        let size: u64;

        let stride = size_of::<T>();
        let upload_size = data_source.len() * stride;

        {
            let device = &device_ptr.borrow().device;

            buffer = device.create_buffer(upload_size as u64, usage).unwrap();
            let mem_req = device.get_buffer_requirements(&buffer);

            // A note about performance: Using CPU_VISIBLE memory is convenient because it can be
            // directly memory mapped and easily updated by the CPU, but it is very slow and so should
            // only be used for small pieces of data that need to be updated very frequently. For something like
            // a vertex buffer that may be much larger and should not change frequently, you should instead
            // use a DEVICE_LOCAL buffer that gets filled by copying data from a CPU_VISIBLE staging buffer.
            let upload_type = memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    mem_req.type_mask & (1 << id) != 0
                        && mem_type.properties.contains(m::Properties::CPU_VISIBLE | m::Properties::COHERENT)
                })
                .unwrap()
                .into();

            memory = device.allocate_memory(upload_type, mem_req.size).unwrap();
            device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
            size = mem_req.size;

            // TODO: check transitions: read/write mapping and vertex buffer read
            let mapping = device.map_memory(&memory, 0 .. size).unwrap();
            ptr::copy_nonoverlapping(data_source.as_ptr() as *const u8, mapping, upload_size);
            device.unmap_memory(&memory);
        }

        BufferState {
            memory: Some(memory),
            buffer: Some(buffer),
            device: device_ptr,
            size,
        }
    }

    fn update_data<T>(&mut self, offset: u64, data_source: &[T])
    where
        T: Copy,
    {
        let device = &self.device.borrow().device;

        let stride = size_of::<T>();
        let upload_size = data_source.len() * stride;

        assert!(offset + upload_size as u64 <= self.size);
        let memory = self.memory.as_ref().unwrap();

        unsafe {
            let mapping = device.map_memory(memory, offset .. self.size).unwrap();
            ptr::copy_nonoverlapping(data_source.as_ptr() as *const u8, mapping, upload_size);
            device.unmap_memory(memory);
        }
    }

/*     unsafe fn new_texture(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        device: &B::Device,
        img: &::image::ImageBuffer<::image::Rgba<u8>, Vec<u8>>,
        adapter: &AdapterState<B>,
        usage: buffer::Usage,
    ) -> (Self, Dimensions<u32>, u32, usize) {
        let (width, height) = img.dimensions();

        let row_alignment_mask = adapter.limits.optimal_buffer_copy_pitch_alignment as u32 - 1;
        let stride = 4usize;

        let row_pitch = (width * stride as u32 + row_alignment_mask) & !row_alignment_mask;
        let upload_size = (height * row_pitch) as u64;

        let memory: B::Memory;
        let mut buffer: B::Buffer;
        let size: u64;

        {
            buffer = device.create_buffer(upload_size, usage).unwrap();
            let mem_reqs = device.get_buffer_requirements(&buffer);

            let upload_type = adapter
                .memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    mem_reqs.type_mask & (1 << id) != 0
                        && mem_type.properties.contains(m::Properties::CPU_VISIBLE | m::Properties::COHERENT)
                })
                .unwrap()
                .into();

            memory = device.allocate_memory(upload_type, mem_reqs.size).unwrap();
            device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
            size = mem_reqs.size;

            // copy image data into staging buffer
            let mapping = device.map_memory(&memory, 0 .. size).unwrap();
            for y in 0 .. height as usize {
                let data_source_slice = &(**img)
                    [y * (width as usize) * stride .. (y + 1) * (width as usize) * stride];
                ptr::copy_nonoverlapping(
                    data_source_slice.as_ptr(),
                    mapping.offset(y as isize * row_pitch as isize),
                    data_source_slice.len(),
                );
            }
            device.unmap_memory(&memory);
        }

        (
            BufferState {
                memory: Some(memory),
                buffer: Some(buffer),
                device: device_ptr,
                size,
            },
            Dimensions { width, height },
            row_pitch,
            stride,
        )
    } */
}

impl<B: Backend> Drop for BufferState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_buffer(self.buffer.take().unwrap());
            device.free_memory(self.memory.take().unwrap());
        }
    }
}

struct Uniform<B: Backend> {
    buffer: Option<BufferState<B>>,
    desc: Option<DescSet<B>>,
}

impl<B: Backend> Uniform<B> {
    unsafe fn new<T>(
        device: Rc<RefCell<DeviceState<B>>>,
        memory_types: &[MemoryType],
        data: &[T],
        mut desc: DescSet<B>,
        binding: u32,
    ) -> Self
    where
        T: Copy,
    {
        let buffer = BufferState::new(
            Rc::clone(&device),
            &data,
            buffer::Usage::UNIFORM,
            memory_types,
        );
        let buffer = Some(buffer);

        desc.write_to_state(
            vec![DescSetWrite {
                binding,
                array_offset: 0,
                descriptors: Some(pso::Descriptor::Buffer(
                    buffer.as_ref().unwrap().get_buffer(),
                    None .. None,
                )),
            }],
            &mut device.borrow_mut().device,
        );

        Uniform {
            buffer,
            desc: Some(desc),
        }
    }

    fn get_layout(&self) -> &B::DescriptorSetLayout {
        self.desc.as_ref().unwrap().get_layout()
    }
}

struct DescSetLayout<B: Backend> {
    layout: Option<B::DescriptorSetLayout>,
    device: Rc<RefCell<DeviceState<B>>>,
}

struct DescSet<B: Backend>{
    set: Option<B::DescriptorSet>,
    layout: DescSetLayout<B>,
}

impl<B: Backend> DescSetLayout<B> {
    unsafe fn new(device: Rc<RefCell<DeviceState<B>>>,bindings: Vec<pso::DescriptorSetLayoutBinding>) -> Self {
        let desc_set_layout = device
            .borrow()
            .device
            .create_descriptor_set_layout(bindings, &[])
            .ok();
        
        DescSetLayout{
            layout: desc_set_layout,
            device,
        }
    }
} 

struct ImageState<B: Backend> {
    desc: DescSet<B>,
    buffer: Option<BufferState<B>>,
    sampler: Option<B::Sampler>,
    image_view: Option<B::ImageView>,
    image: Option<B::Image>,
    memory: Option<B::Memory>,
    transfered_image_fence: Option<B::Fence>,
}

impl<B: Backend> ImageState<B> {
    unsafe fn new(
        mut desc: DescSet<B>,
        img: &image::ImageBuffer<::image::Rgba<u8>, Vec<u8>>,
        adapter: &AdapterState<B>,
        usage: buffer::Usage,
        device_state: &mut DeviceState<B>,
        staging_pool: &mut B::CommandPool,
    ) -> Self {
        let (buffer, dims, row_pitch, stride) = BufferState::new_texture(
            Rc::clone(&desc.layout.device),
            &mut device_state.device,
            img,
            adapter,
            usage,
        );

        let buffer = Some(buffer);
        let device = &mut device_state.device;

        let kind = image::Kind::D2(dims.width as i::Size, dims.height as i::Size, 1, 1);
        let mut image = device
            .create_image(
                kind,
                1,
                ColorFormat::SELF,
                image::Tiling::Optimal,
                image::Usage::TRANSFER_DST | i::Usage::SAMPLED,
                image::ViewCapabilities::empty(),
            )
            .unwrap(); // TODO: usage
        let req = device.get_image_requirements(&image);

        let device_type = adapter
            .memory_types
            .iter()
            .enumerate()
            .position(|(id, memory_type)| {
                req.type_mask & (1 << id) != 0
                    && memory_type.properties.contains(m::Properties::DEVICE_LOCAL)
            })
            .unwrap()
            .into();

        let memory = device.allocate_memory(device_type, req.size).unwrap();

        device.bind_image_memory(&memory, 0, &mut image).unwrap();
        let image_view = device
            .create_image_view(
                &image,
                i::ViewKind::D2,
                ColorFormat::SELF,
                f::Swizzle::NO,
                COLOR_RANGE.clone(),
            )
            .unwrap();

        let sampler = device
            .create_sampler(i::SamplerInfo::new(i::Filter::Linear, i::WrapMode::Clamp))
            .expect("Can't create sampler");

        desc.write_to_state(
            vec![
                DescSetWrite {
                    binding: 0,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Image(
                        &image_view,
                        i::Layout::ShaderReadOnlyOptimal,
                    )),
                },
                DescSetWrite {
                    binding: 1,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Sampler(&sampler)),
                },
            ],
            device,
        );

        let mut transfered_image_fence = device.create_fence(false).expect("Can't create fence");

        // copy buffer to texture
        {
            let mut cmd_buffer = staging_pool.allocate_one(command::Level::Primary);
            cmd_buffer.begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);

            let image_barrier = m::Barrier::Image {
                states: (i::Access::empty(), i::Layout::Undefined)
                    .. (i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal),
                target: &image,
                families: None,
                range: COLOR_RANGE.clone(),
            };

            cmd_buffer.pipeline_barrier(
                pso::PipelineStage::TOP_OF_PIPE .. pso::PipelineStage::TRANSFER,
                m::Dependencies::empty(),
                &[image_barrier],
            );

            cmd_buffer.copy_buffer_to_image(
                buffer.as_ref().unwrap().get_buffer(),
                &image,
                i::Layout::TransferDstOptimal,
                &[command::BufferImageCopy {
                    buffer_offset: 0,
                    buffer_width: row_pitch / (stride as u32),
                    buffer_height: dims.height as u32,
                    image_layers: i::SubresourceLayers {
                        aspects: f::Aspects::COLOR,
                        level: 0,
                        layers: 0 .. 1,
                    },
                    image_offset: i::Offset { x: 0, y: 0, z: 0 },
                    image_extent: i::Extent {
                        width: dims.width,
                        height: dims.height,
                        depth: 1,
                    },
                }],
            );

            let image_barrier = m::Barrier::Image {
                states: (i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal)
                    .. (i::Access::SHADER_READ, i::Layout::ShaderReadOnlyOptimal),
                target: &image,
                families: None,
                range: COLOR_RANGE.clone(),
            };
            cmd_buffer.pipeline_barrier(
                pso::PipelineStage::TRANSFER .. pso::PipelineStage::FRAGMENT_SHADER,
                m::Dependencies::empty(),
                &[image_barrier],
            );

            cmd_buffer.finish();

            device_state.queues.queues[0].submit_without_semaphores(
                iter::once(&cmd_buffer),
                Some(&mut transfered_image_fence),
            );
        }

        ImageState {
            desc: desc,
            buffer: buffer,
            sampler: Some(sampler),
            image_view: Some(image_view),
            image: Some(image),
            memory: Some(memory),
            transfered_image_fence: Some(transfered_image_fence),
        }
    }

    fn wait_for_transfer_completion(&self) {
        let device = &self.desc.layout.device.borrow().device;
        unsafe {
            device
                .wait_for_fence(self.transfered_image_fence.as_ref().unwrap(), !0)
                .unwrap();
        }
    }

    fn get_layout(&self) -> &B::DescriptorSetLayout {
        self.desc.get_layout()
    }
}

impl<B: Backend> Drop for ImageState<B> {
    fn drop(&mut self) {
        unsafe {
            let device = &self.desc.layout.device.borrow().device;

            let fence = self.transfered_image_fence.take().unwrap();
            device.wait_for_fence(&fence, !0).unwrap();
            device.destroy_fence(fence);

            device.destroy_sampler(self.sampler.take().unwrap());
            device.destroy_image_view(self.image_view.take().unwrap());
            device.destroy_image(self.image.take().unwrap());
            device.free_memory(self.memory.take().unwrap());
        }

        self.buffer.take().unwrap();
    }
}

struct FramebufferState<B: Backend> {
    framebuffers: Option<Vec<B::Framebuffer>>,
    framebuffer_fences: Option<Vec<B::Fence>>,
    command_pools: Option<Vec<B::CommandPool>>,
    command_buffer_lists: Vec<Vec<B::CommandBuffer>>,
    frame_images: Option<Vec<(B::Image, B::ImageView)>>,
    acquire_semaphores: Option<Vec<B::Semaphore>>,
    present_semaphores: Option<Vec<B::Semaphore>>,
    last_ref: usize,
    device: Rc<RefCell<DeviceState<B>>>,
}

impl<B: Backend> FramebufferState<B> {
    unsafe fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        render_pass: &RenderPassState<B>,
        swapchain: &mut SwapchainState<B>,
    ) -> Self {
        let (frame_images, framebuffers) = {
            let extent = i::Extent {
                width: swapchain.extent.width as _,
                height: swapchain.extent.height as _,
                depth: 1,
            };
            let pairs = swapchain
                .backbuffer
                .take()
                .unwrap()
                .into_iter()
                .map(|image| {
                    let rtv = device
                        .borrow()
                        .device
                        .create_image_view(
                            &image,
                            i::ViewKind::D2,
                            swapchain.format,
                            f::Swizzle::NO,
                            COLOR_RANGE.clone(),
                        )
                        .unwrap();
                    (image, rtv)
                })
                .collect::<Vec<_>>();
            let fbos = pairs
                .iter()
                .map(|&(_, ref rtv)| {
                    device
                        .borrow()
                        .device
                        .create_framebuffer(
                            render_pass.render_pass.as_ref().unwrap(),
                            Some(rtv),
                            extent,
                        )
                        .unwrap()
                })
                .collect();
            (pairs, fbos)
        };

        let iter_count = if frame_images.len() != 0 {
            frame_images.len()
        } else {
            1 // GL can have zero
        };

        let mut fences: Vec<B::Fence> = vec![];
        let mut command_pools: Vec<_> = vec![];
        let mut command_buffer_lists = Vec::new();
        let mut acquire_semaphores: Vec<B::Semaphore> = vec![];
        let mut present_semaphores: Vec<B::Semaphore> = vec![];

        for _ in 0 .. iter_count {
            fences.push(device.borrow().device.create_fence(true).unwrap());
            command_pools.push(
                device
                    .borrow()
                    .device
                    .create_command_pool(
                        device.borrow().queues.family,
                        pool::CommandPoolCreateFlags::empty(),
                    )
                    .expect("Can't create command pool"),
            );
            command_buffer_lists.push(Vec::new());

            acquire_semaphores.push(device.borrow().device.create_semaphore().unwrap());
            present_semaphores.push(device.borrow().device.create_semaphore().unwrap());
        }

        FramebufferState {
            frame_images: Some(frame_images),
            framebuffers: Some(framebuffers),
            framebuffer_fences: Some(fences),
            command_pools: Some(command_pools),
            command_buffer_lists,
            present_semaphores: Some(present_semaphores),
            acquire_semaphores: Some(acquire_semaphores),
            device,
            last_ref: 0,
        }
    }

    fn next_acq_pre_pair_index(&mut self) -> usize {
        if self.last_ref >= self.acquire_semaphores.as_ref().unwrap().len() {
            self.last_ref = 0
        }

        let ret = self.last_ref;
        self.last_ref += 1;
        ret
    }

    fn get_frame_data(
        &mut self,
        frame_id: Option<usize>,
        sem_index: Option<usize>,
    ) -> (
        Option<(
            &mut B::Fence,
            &mut B::Framebuffer,
            &mut B::CommandPool,
            &mut Vec<B::CommandBuffer>,
        )>,
        Option<(&mut B::Semaphore, &mut B::Semaphore)>,
    ) {
        (
            if let Some(fid) = frame_id {
                Some((
                    &mut self.framebuffer_fences.as_mut().unwrap()[fid],
                    &mut self.framebuffers.as_mut().unwrap()[fid],
                    &mut self.command_pools.as_mut().unwrap()[fid],
                    &mut self.command_buffer_lists[fid],
                ))
            } else {
                None
            },
            if let Some(sid) = sem_index {
                Some((
                    &mut self.acquire_semaphores.as_mut().unwrap()[sid],
                    &mut self.present_semaphores.as_mut().unwrap()[sid],
                ))
            } else {
                None
            },
        )
    }
}

impl<B: Backend> Drop for FramebufferState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;

        unsafe {
            for fence in self.framebuffer_fences.take().unwrap() {
                device.wait_for_fence(&fence, !0).unwrap();
                device.destroy_fence(fence);
            }

            for (mut command_pool, comamnd_buffer_list) in self
                .command_pools
                .take()
                .unwrap()
                .into_iter()
                .zip(self.command_buffer_lists.drain(..))
            {
                command_pool.free(comamnd_buffer_list);
                device.destroy_command_pool(command_pool);
            }

            for acquire_semaphore in self.acquire_semaphores.take().unwrap() {
                device.destroy_semaphore(acquire_semaphore);
            }

            for present_semaphore in self.present_semaphores.take().unwrap() {
                device.destroy_semaphore(present_semaphore);
            }

            for framebuffer in self.framebuffers.take().unwrap() {
                device.destroy_framebuffer(framebuffer);
            }

            for (_, rtv) in self.frame_images.take().unwrap() {
                device.destroy_image_view(rtv);
            }
        }
    }
}

pub struct RenderEngine<B : Backend> {
    initState: InitState<B>,
    device: Rc<RefCell<DeviceState<B>>>,
    swapchain: Option<SwapchainState<B>>,
    image_desc_pool: Option<B::DescriptorPool>,
    windowState:WindowState,
}

impl RenderEngine<back::Backend> {
    pub fn new() -> Self {
        let windowState = WindowState::default();
        let mut initState = InitState::new(&windowState);
        let device = Rc::new(RefCell::new(DeviceState::new(initState.adapter.adapter.take().unwrap(), &initState.surface)));

        let swapchain = unsafe { Some(SwapchainState::new(&mut initState, Rc::clone(&device))) };
        
        let image_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![
                pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::SampledImage,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: pso::DescriptorType::Sampler,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
            ],
        );

        let uniform_desc = DescSetLayout::new(
            Rc::clone(&device),
            vec![pso::DescriptorSetLayoutBinding {
                binding: 0,
                ty: pso::DescriptorType::UniformBuffer,
                count: 1,
                stage_flags: pso::ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            }],
        );

        let mut img_desc_pool = device
            .borrow()
            .device
            .create_descriptor_pool(
                1, // # of sets
                &[
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::SampledImage,
                        count: 1,
                    },
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::Sampler,
                        count: 1,
                    },
                ],
                pso::DescriptorPoolCreateFlags::empty(),
            )
            .ok();

        RenderEngine{
            initState,
            device,
            swapchain,
            windowState,
            image_desc_pool,
        }
    }

    pub fn info(&self) {
        self.initState.adapters_info();
        self.initState.adapter.limits_info();
    }

    pub fn main_loop(&mut self) {
        
        let mut running = true;
        while running {
            //
            self.windowState.event_loop.poll_events(|event|{
                match event {
                    Event::WindowEvent {event: WindowEvent::CloseRequested,..} => {
                        println!{"The close button was pressed; stopping"};
                        running = false;
                    },
                    _ => (),
                }
            });
            
        }
    }
}

impl<B:Backend> Drop for RenderEngine<B> {
    fn drop(& mut self) {
        self.device.borrow().device.wait_idle().unwrap();
        unsafe{
            self.swapchain.take();
        }
    }
}