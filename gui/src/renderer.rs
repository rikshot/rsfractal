use pixels::wgpu::{self, util::DeviceExt};
use rsfractal_mandelbrot::mandelbrot::Mandelbrot;
use rsfractal_mandelbrot::range::Range;

const COLORING_SIZE: u32 = 256;

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Params {
    real_range: Range,
    imaginary_range: Range,
    viewport: [f32; 2],
    bailout: f32,
    max_iterations: u32,
    exponent: f32,
    _padding: [u8; 12],
}

fn bake_coloring_data(mandelbrot: &Mandelbrot) -> Vec<u8> {
    let mut data = vec![0u8; (COLORING_SIZE * 4) as usize];
    for i in 0..COLORING_SIZE {
        let s = i as f32 / (COLORING_SIZE - 1) as f32;
        let color = mandelbrot.color_at(s);
        data[i as usize * 4..i as usize * 4 + 4].copy_from_slice(&color.to_rgba8());
    }
    data
}

fn upload_coloring_texture(device: &wgpu::Device, queue: &wgpu::Queue, data: &[u8]) -> wgpu::Texture {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: COLORING_SIZE,
            height: 1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        data,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(COLORING_SIZE * 4),
            rows_per_image: None,
        },
        wgpu::Extent3d {
            width: COLORING_SIZE,
            height: 1,
            depth_or_array_layers: 1,
        },
    );

    texture
}

#[allow(dead_code)]
pub(crate) struct MandelbrotRenderer {
    params_buffer: wgpu::Buffer,
    coloring_texture: wgpu::Texture,
    coloring_texture_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    render_pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl MandelbrotRenderer {
    pub(crate) fn new(pixels: &pixels::Pixels, mandelbrot: &Mandelbrot) -> Self {
        let device = pixels.device();
        let queue = pixels.queue();
        let shader = wgpu::include_wgsl!("mandelbrot.wgsl");
        let module = device.create_shader_module(shader);

        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &[0u8; std::mem::size_of::<Params>()],
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let data = bake_coloring_data(mandelbrot);
        let coloring_texture = upload_coloring_texture(device, queue, &data);
        let coloring_texture_view = coloring_texture.create_view(&Default::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<Params>() as u64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&coloring_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: pixels.render_texture_format(),
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            params_buffer,
            coloring_texture,
            coloring_texture_view,
            sampler,
            render_pipeline,
            bind_group_layout,
            bind_group,
        }
    }

    pub(crate) fn update_coloring(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, mandelbrot: &Mandelbrot) {
        let data = bake_coloring_data(mandelbrot);
        self.coloring_texture = upload_coloring_texture(device, queue, &data);
        self.coloring_texture_view = self.coloring_texture.create_view(&Default::default());
        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&self.coloring_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
    }

    pub(crate) fn set_params(&self, queue: &wgpu::Queue, mandelbrot: &Mandelbrot, viewport_width: f32, viewport_height: f32) {
        let ranges = mandelbrot.ranges();
        let params = Params {
            real_range: ranges[2],
            imaginary_range: ranges[3],
            viewport: [viewport_width, viewport_height],
            bailout: mandelbrot.bailout,
            max_iterations: mandelbrot.max_iterations as u32,
            exponent: mandelbrot.exponent,
            _padding: [0u8; 12],
        };
        queue.write_buffer(&self.params_buffer, 0, bytemuck::bytes_of(&params));
    }

    pub(crate) fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        clip_rect: (u32, u32, u32, u32),
    ) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: render_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.set_scissor_rect(clip_rect.0, clip_rect.1, clip_rect.2, clip_rect.3);
        rpass.draw(0..3, 0..1);
    }
}
