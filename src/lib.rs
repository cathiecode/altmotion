#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod core {
    pub struct Vertex(pub f32, pub f32, pub f32, pub f32, pub f32);
    pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

    // データ構造


    pub enum Shape {
        Triangle(Triangle),
        Poly(Vec<Triangle>)
    }

    pub struct Object<T> {
        pub image: T,
        pub shape: Vec<Shape>,
    }

    pub struct Layer<T> {
        pub objects: Vec<Object<T>>
    }

    pub struct Scene<T> {
        pub layer: Vec<Layer<T>>,
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
        async fn into_bitmap(&mut self, image: Self::Image) -> Pixmap;
    }
}

pub mod tinyskia_renderer {
    use crate::renderer;
    use crate::core;
    use async_trait::async_trait;
    use tiny_skia::Pixmap;

    pub struct TSkiaRenderer {
        
    }

    impl TSkiaRenderer {
        pub fn new() -> Self {
            TSkiaRenderer {}
        }
    }

    #[async_trait]
    impl renderer::Renderer for TSkiaRenderer {
        type Image = Pixmap;
        fn render(&mut self, scene: core::Scene<Self::Image>, dest: &Self::Image) {
            todo!();
        }

        fn into_image(&mut self, image: tiny_skia::Pixmap) -> Self::Image {
            image
        }

        async fn into_bitmap(&mut self, image: Self::Image) -> tiny_skia::Pixmap {
            image
        }
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

    pub struct WGpuTexture {
        pub texture: wgpu::Texture,
        pub width: usize,
        pub height: usize
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
            todo!();
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
                format: wgpu::TextureFormat::Rgba8Uint,
                usage: wgpu::TextureUsage::all(), // OPTIMIZE: Performance problem
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
                width: image.width() as usize,
                height: image.height() as usize
            };
        }

        async fn into_bitmap(&mut self, image: Self::Image) -> tiny_skia::Pixmap {
            let buffer_dimensions = BufferDimensions::new(image.width, image.height);
            let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: (buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height) as u64,
                usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            });

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
            
            let mut output = Pixmap::new(buffer_dimensions.width as u32, buffer_dimensions.height as u32).unwrap();

            if let Ok(()) = buffer_future.await {
                let padded_buffer = buffer_slice.get_mapped_range();

                /*
                png_encoder.set_depth(png::BitDepth::Eight);
                png_encoder.set_color(png::ColorType::RGBA);
                */
                /*let mut png_writer = png_encoder
                    .write_header()
                    .unwrap()
                    .into_stream_writer_with_size(buffer_dimensions.unpadded_bytes_per_row);*/
        
                // from the padded_buffer we write just the unpadded bytes into the image
                let mut offset = 0;
                let data = output.data_mut();
                for chunk in padded_buffer.chunks(buffer_dimensions.padded_bytes_per_row) {
                    for (i, &byte) in chunk.iter().enumerate() {
                        data[offset + i] = byte;
                    }

                    offset += buffer_dimensions.unpadded_bytes_per_row;
                    /*png_writer
                        .write_all(&chunk[..buffer_dimensions.unpadded_bytes_per_row])
                        .unwrap();*/
                }
                /*png_writer.finish().unwrap();*/
        
                // With the current interface, we have to make sure all mapped views are
                // dropped before we unmap the buffer.
                drop(padded_buffer);
        
                output_buffer.unmap();
            }

            output
        }
    }

    struct BufferDimensions {
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
