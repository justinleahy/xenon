use super::{RenderCamera, RenderError, RenderScene, RenderSprite};
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

const MAX_SPRITE_INSTANCES: usize = 1024;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];

    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

const QUAD_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
];

const QUAD_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct SpriteInstance {
    translation: [f32; 2],
    scale: [f32; 2],
    color: [f32; 4],
}

impl SpriteInstance {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![1 => Float32x2, 2 => Float32x2, 3 => Float32x4];

    fn from_sprite(
        sprite: RenderSprite,
        camera: RenderCamera,
        surface_width: u32,
        surface_height: u32,
    ) -> Self {
        let width = surface_width.max(1) as f32;
        let height = surface_height.max(1) as f32;

        let relative_world = [
            sprite.position[0] - camera.position[0],
            sprite.position[1] - camera.position[1],
        ];

        let screen_pixels = [
            relative_world[0] * camera.pixels_per_world_unit,
            relative_world[1] * camera.pixels_per_world_unit,
        ];

        let size_pixels = [
            sprite.size[0] * camera.pixels_per_world_unit,
            sprite.size[1] * camera.pixels_per_world_unit,
        ];

        Self {
            translation: [
                screen_pixels[0] / (width * 0.5),
                -screen_pixels[1] / (height * 0.5),
            ],
            scale: [size_pixels[0] / width, size_pixels[1] / height],
            color: sprite.color,
        }
    }

    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub struct Renderer<'window> {
    _instance: wgpu::Instance,
    surface: wgpu::Surface<'window>,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    quad_pipeline: wgpu::RenderPipeline,
    quad_vertex_buffer: wgpu::Buffer,
    quad_index_buffer: wgpu::Buffer,
    quad_index_count: u32,
    sprite_instance_buffer: wgpu::Buffer,
    sprite_instance_capacity: usize,
}

impl<'window> Renderer<'window> {
    pub async fn new(
        window: impl Into<wgpu::SurfaceTarget<'window>>,
        width: u32,
        height: u32,
    ) -> Result<Self, RenderError> {
        Self::new_with_shader_source(window, width, height, include_str!("shaders/colored.wgsl"))
            .await
    }

    pub async fn new_with_shader_source(
        window: impl Into<wgpu::SurfaceTarget<'window>>,
        width: u32,
        height: u32,
        shader_source: &str,
    ) -> Result<Self, RenderError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Xenon Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await?;

        let mut config = surface
            .get_default_config(&adapter, width, height)
            .ok_or(RenderError::UnsupportedSurface)?;

        config.present_mode = wgpu::PresentMode::Fifo;

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Xenon Triangle Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Xenon Triangle Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let quad_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Xenon Quad Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::layout(), SpriteInstance::layout()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Xenon Quad Vertex Buffer"),
            contents: bytemuck::cast_slice(QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let quad_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Xenon Quad Index Buffer"),
            contents: bytemuck::cast_slice(QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let quad_index_count = QUAD_INDICES.len() as u32;

        let sprite_instance_capacity = MAX_SPRITE_INSTANCES;

        let sprite_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Xenon Sprite Instance Buffer"),
            size: (std::mem::size_of::<SpriteInstance>() * sprite_instance_capacity)
                as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Self {
            _instance: instance,
            surface,
            _adapter: adapter,
            device,
            queue,
            config,
            quad_pipeline,
            quad_vertex_buffer,
            quad_index_buffer,
            quad_index_count,
            sprite_instance_buffer,
            sprite_instance_capacity,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        if self.config.width == width && self.config.height == height {
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(
        &mut self,
        clear_color: [u8; 4],
        scene: RenderScene<'_>,
    ) -> Result<(), RenderError> {
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,

            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(());
            }

            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }

            wgpu::CurrentSurfaceTexture::Validation => {
                return Err(RenderError::SurfaceValidation);
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Xenon Clear Encoder"),
            });

        {
            let sprite_instances: Vec<_> = scene
                .sprites
                .iter()
                .copied()
                .map(|sprite| {
                    SpriteInstance::from_sprite(
                        sprite,
                        scene.camera,
                        self.config.width,
                        self.config.height,
                    )
                })
                .collect();

            if sprite_instances.len() > self.sprite_instance_capacity {
                return Err(RenderError::TooManySprites {
                    count: sprite_instances.len(),
                    capacity: self.sprite_instance_capacity,
                });
            }

            if !sprite_instances.is_empty() {
                self.queue.write_buffer(
                    &self.sprite_instance_buffer,
                    0,
                    bytemuck::cast_slice(&sprite_instances),
                )
            }

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Xenon Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color_from_rgba8(clear_color)),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            if !sprite_instances.is_empty() {
                render_pass.set_pipeline(&self.quad_pipeline);
                render_pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, self.sprite_instance_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.quad_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(
                    0..self.quad_index_count,
                    0,
                    0..sprite_instances.len() as u32,
                );
            }
        }

        self.queue.submit([encoder.finish()]);
        frame.present();

        Ok(())
    }
}

fn color_from_rgba8([r, g, b, a]: [u8; 4]) -> wgpu::Color {
    wgpu::Color {
        r: r as f64 / 255.0,
        g: g as f64 / 255.0,
        b: b as f64 / 255.0,
        a: a as f64 / 255.0,
    }
}
