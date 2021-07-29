use altmotion::core::*;
use altmotion::renderer::*;
use altmotion::wgpu_renderer::*;
use tiny_skia::Pixmap;

use async_std::task::block_on;

fn main() {
    println!("create renderer");
    let mut renderer = block_on(WGpuRenderer::new());

    println!("load image");
    let mut initial_image = Pixmap::load_png("texture.png").unwrap();

    println!("create image");
    let mut image = renderer.into_image(initial_image);

    println!("create canvas");
    let canvas = renderer.into_image(Pixmap::new(1920, 1080).unwrap());

    println!("create bitmap output");
    let mut bit_canvas = Pixmap::new(1920, 1080).unwrap();

    println!("build scene");
    let scene = Scene {
        width: 1920,
        height: 1080,
        layers: vec![Layer {
            objects: vec![
                Object {
                    shape: vec![Shape::Triangle([Vertex(250.0, 0.0, 0.0, 0.0, 0.0), Vertex(500.0, 500.0, 0.0, 0.0, 0.0), Vertex(0.0, 500.0, 0.0, 0.0, 0.0)])],
                    image: &image
                }
            ]
        }]
    };

    println!("render");
    renderer.render(scene, &canvas);

    println!("into bitmap");
    block_on(renderer.into_bitmap(&canvas, &mut bit_canvas));

    println!("save to png");
    bit_canvas.save_png("test.png").unwrap();

    println!("done");
}
