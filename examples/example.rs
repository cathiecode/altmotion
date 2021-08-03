use altmotion::core::*;
use altmotion::renderer::*;
use altmotion::wgpu_renderer::*;
use tiny_skia::Pixmap;

use async_std::task::block_on;
use renderdoc::{RenderDoc, V100, V110};

fn main() {
    env_logger::init();
    let mut rd: RenderDoc<V110> = RenderDoc::new().expect("Unable to connect");
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
                    shape: vec![
                        Shape::Triangle([Vertex(-1.0, -1.0, 0.0, 0.0, 0.0), Vertex(1.0, -1.0, 0.0, 512.0, 0.0), Vertex(-1.0, 1.0, 0.0, 0.0, 512.0)]),
                        Shape::Triangle([Vertex(1.0, -1.0, 0.0, 512.0, 0.0), Vertex(-1.0, 1.0, 0.0, 0.0, 512.0), Vertex(1.0, 1.0, 0.0, 512.0, 512.0)])
                    ],
                    image: &image
                }
            ]
        }]
    };

    println!("render");
    rd.start_frame_capture(std::ptr::null(), std::ptr::null());
    renderer.render(scene, &canvas);
    rd.end_frame_capture(std::ptr::null(), std::ptr::null());

    println!("into bitmap");
    block_on(renderer.into_bitmap(&canvas, &mut bit_canvas));

    println!("save to png");
    bit_canvas.save_png("test.png").unwrap();

    println!("done");
}
