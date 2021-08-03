#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod core {
    use bytemuck::{Pod, Zeroable};

    #[repr(C)]
    #[derive(Clone, Copy, Pod, Zeroable)]
    pub struct Vertex(pub f32, pub f32, pub f32, pub f32, pub f32);

    impl Into<[f32; 5]> for Vertex {
        fn into(self) -> [f32; 5] {
            [self.0, self.1, self.2, self.3, self.4]
        }
    }

    //pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

    pub type Triangle = [Vertex; 3];

    // データ構造


    pub enum Shape {
        Triangle(Triangle)
    }

    pub struct Object<'a, T> {
        pub image: &'a T,
        pub shape: Vec<Shape>,
    }

    pub struct Layer<'a, T> {
        pub objects: Vec<Object<'a, T>>
    }

    pub struct Scene<'a, T> {
        pub layers: Vec<Layer<'a, T>>,
        pub width: u32,
        pub height: u32
    }
}

pub mod renderer {
    use crate::core;
    use tiny_skia::Pixmap;
    use async_trait::async_trait;

    #[async_trait]
    pub trait Renderer {
        type Image;
        fn render(&mut self, scene: core::Scene<Self::Image>, dest: &Self::Image); // TODO: TargetをImageで受けとるようにする
        fn into_image(&mut self, bitmap: Pixmap) -> Self::Image;
        async fn into_bitmap(&mut self, image: &Self::Image, dest: &mut Pixmap);
    }
}

pub mod wgpu_renderer {
    use std::ops::Deref;
    use std::usize;

    use crate::renderer;
    use crate::core;
    use tiny_skia::Pixmap;
    use async_trait::async_trait;
    use wgpu::*;
    use wgpu::util::DeviceExt;

    pub struct WGpuTexture {
        pub texture: wgpu::Texture,
        pub width: usize,
        pub height: usize,
        pub buffer: wgpu::Buffer,
        pub buffer_dimensions: BufferDimensions
    }

    pub struct WGpuRenderer {
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue
    }

    impl WGpuRenderer {
        pub async fn new() -> Self {
            let adapter = wgpu::Instance::new(wgpu::BackendBit::PRIMARY)
                .request_adapter(&wgpu::RequestAdapterOptions::default())
                .await
                .unwrap();

            let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("WGpuRenderer device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(), // TODO: 本当に？
                },
                None
            )
                .await
                .unwrap();


            println!("Greeting from WGpu Renderer! ({:?})", adapter.get_info());

            WGpuRenderer {
                adapter,
                device,
                queue
            }
        }
    }

    #[async_trait]
    impl renderer::Renderer for WGpuRenderer {
        type Image = WGpuTexture;
        fn render(&mut self, scene: core::Scene<Self::Image>, dest: &Self::Image) {
            let bind_group_layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            sample_type: wgpu::TextureSampleType::Float{filterable: true},
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                ],
            });
            let pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
            let sc_desc = wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                format: TextureFormat::Rgba8UnormSrgb,
                width: scene.width,
                height: scene.height,
                present_mode: wgpu::PresentMode::Mailbox,
            };
            for layer in &scene.layers {
                for object in &layer.objects {
                    let image = object.image;
                    let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::TextureView(&image.texture.create_view(&wgpu::TextureViewDescriptor::default())),
                            },
                        ],
                        label: None,
                    });

                    let shader = self.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                        label: None,
                        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
                        flags: ShaderFlags::empty(),
                    });

                    let vertex_size = std::mem::size_of::<core::Vertex>();
                    let vertex_buffers = [wgpu::VertexBufferLayout {
                        array_stride: vertex_size as wgpu::BufferAddress,
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x3,
                                offset: 0,
                                shader_location: 0,
                            },
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x2,
                                offset: 4 * 3,
                                shader_location: 1,
                            },
                        ],
                    }];


                    let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: None,
                        layout: Some(&pipeline_layout),
                        vertex: wgpu::VertexState {
                            module: &shader,
                            entry_point: "vs_main",
                            buffers: &vertex_buffers, // TODO
                        },
                        fragment: Some(wgpu::FragmentState {
                            module: &shader,
                            entry_point: "fs_main",
                            targets: &[sc_desc.format.into()],
                        }),
                        primitive: wgpu::PrimitiveState {
                            cull_mode: None, // TODO: あやしい
                            ..wgpu::PrimitiveState::default()
                        },
                        depth_stencil: None,
                        multisample: wgpu::MultisampleState::default(),
                    });

                    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    
                    let texture_view = dest.texture.create_view(&wgpu::TextureViewDescriptor::default());

                    let mut vertex_buffers = Vec::<(&core::Shape, Buffer)>::new();

                    {
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[wgpu::RenderPassColorAttachment {
                                view: &texture_view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: true,
                                },
                            }],
                            depth_stencil_attachment: None,
                        });

                        rpass.set_pipeline(&pipeline);
                        
                        //rpass.set_index_buffer(index_buf.slice(..), wgpu::IndexFormat::Uint16);
                        
                        //rpass.draw_indexed(0..index_count as u32, 0, 0..1);

                        /**/

                        for shape in &object.shape {
                            let vertex_data: &[core::Vertex] = match(shape) {
                                core::Shape::Triangle(tri) => tri
                            };

                            vertex_buffers.push((
                                &shape,
                                self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                    label: Some("Vertex Buffer"),
                                    contents: bytemuck::cast_slice(&vertex_data),
                                    usage: wgpu::BufferUsage::VERTEX,
                                })
                            ));
                        }

                        for vertex_buffer in &vertex_buffers {
                            rpass.set_bind_group(0, &bind_group, &[]);
                            rpass.set_vertex_buffer(0, vertex_buffer.1.slice(..));
                            rpass.draw(0..3, 0..1); // TODO: Triangle以外だと3ではいけない
                        }
                    }
                    self.queue.submit(Some(encoder.finish()));
                }
            }
        }

        fn into_image(&mut self, image: tiny_skia::Pixmap) -> Self::Image {
            let texture_extent = wgpu::Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            };
            let texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: texture_extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::all() & !(wgpu::TextureUsage::STORAGE), // OPTIMIZE: Performance problem
            });
            let buffer_dimensions = BufferDimensions::new(image.width() as usize, image.height() as usize);
            let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: (buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height) as u64,
                usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            });
            println!("Width: {}", image.width());
            self.queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO
                },
                image.data(),
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(std::num::NonZeroU32::new(image.width() * 4).unwrap()), // NOTE: 4 for RGBA(u8, u8, u8, u8)
                    rows_per_image: Some(std::num::NonZeroU32::new(image.height()).unwrap())
                },
                texture_extent,
            );

            self.queue.submit(None);

            return WGpuTexture {
                texture,
                buffer,
                buffer_dimensions,
                width: image.width() as usize,
                height: image.height() as usize,
            };
        }

        async fn into_bitmap(&mut self, image: &Self::Image, dest: &mut Pixmap) {
            let output_buffer = &image.buffer;
            let buffer_dimensions = &image.buffer_dimensions;

            let command_buffer = {
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {label: Some("into_bitmap")});
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("into_bitmap_copy_pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &image.texture.create_view(&wgpu::TextureViewDescriptor::default()),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true
                        }
                    }],
                    depth_stencil_attachment: None,
                });
                encoder.copy_texture_to_buffer(
                    wgpu::ImageCopyTexture {
                        texture: &image.texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO
                    },
                    wgpu::ImageCopyBuffer {
                        buffer: &output_buffer,
                        layout: wgpu::ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(
                                std::num::NonZeroU32::new(buffer_dimensions.padded_bytes_per_row as u32)
                                    .unwrap(),
                            ),
                            rows_per_image: None,
                        },
                    },
                    wgpu::Extent3d {
                        width: image.width as u32,
                        height: image.height as u32,
                        depth_or_array_layers: 1,
                    },
                );
                encoder.finish()
            };

            self.queue.submit(Some(command_buffer));

            let buffer_slice = output_buffer.slice(..);
            let buffer_future = buffer_slice.map_async(wgpu::MapMode::Read);

            self.device.poll(wgpu::Maintain::Wait);

            if let Ok(()) = buffer_future.await {
                let padded_buffer = buffer_slice.get_mapped_range();

                let mut offset = 0;
                let data = dest.data_mut();
                for chunk in padded_buffer.chunks(buffer_dimensions.padded_bytes_per_row) {
                    data[offset..offset + buffer_dimensions.padded_bytes_per_row].copy_from_slice(chunk);

                    offset += buffer_dimensions.unpadded_bytes_per_row; // FIXME: 多分壊れてる
                }
                drop(padded_buffer);
        
                output_buffer.unmap();
            }
        }
    }

    pub struct BufferDimensions {
        width: usize,
        height: usize,
        unpadded_bytes_per_row: usize,
        padded_bytes_per_row: usize,
    }
    
    impl BufferDimensions {
        fn new(width: usize, height: usize) -> Self {
            let bytes_per_pixel = std::mem::size_of::<u32>();
            let unpadded_bytes_per_row = width * bytes_per_pixel;
            let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
            let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
            let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
            Self {
                width,
                height,
                unpadded_bytes_per_row,
                padded_bytes_per_row,
            }
        }
    }
}
