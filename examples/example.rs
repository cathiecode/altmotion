use altmotion::core::*;
use altmotion::renderer::*;
use altmotion::wgpu_renderer::*;
use tiny_skia::Pixmap;

use async_std::task::block_on;

fn main() {
    let mut renderer = block_on(WGpuRenderer::new());

    let mut initial_image = Pixmap::load_png("texture.png").unwrap();

    let mut image = renderer.into_image(initial_image);

    let canvas = renderer.into_image(Pixmap::new(1920, 1080).unwrap());

    loop {
        let scene = Scene {
            width: 1920,
            height: 1080,
            layer: vec![Layer {
                objects: vec![
                    Object {
                        shape: vec![Shape::Triangle(Triangle(Vertex(250.0, 0.0, 0.0, 0.0, 0.0), Vertex(500.0, 500.0, 0.0, 0.0, 0.0), Vertex(0.0, 500.0, 0.0, 0.0, 0.0)))],
                        image: &image
                    }
                ]
            }]
        };

        block_on(renderer.into_bitmap(&canvas));
        println!("done");
    }
    
    //.save_png("test.png").unwrap();
}
