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

    let mut image = renderer.create_image(initial_image.width() as usize, initial_image.height() as usize);
    renderer.into_image(initial_image, &image);

    println!("create canvas");
    let canvas = renderer.create_image(1920, 1080);
    //renderer.into_image(Pixmap::new(1920, 1080).unwrap(), &canvas);

    println!("create bitmap output");
    let mut bit_canvas = Pixmap::new(1920, 1080).unwrap();

    println!("build scene");
    let mut objects: Vec<Object<<WGpuRenderer<'_> as Renderer>::Image>> = Vec::new();
    for _ in 0..100 {
        let mut shapes: Vec<Shape> = Vec::new();

        for i in 0..100 {
            let rad = (i as f32) / 100.0 * std::f32::consts::TAU;
            let rad2 = ((i + 1) as f32) / 100.0 * std::f32::consts::TAU;
            shapes.push(Shape::Triangle([Vertex(0.0, 0.0, 0.0, 256.0, 256.0), Vertex(rad.cos(), rad.sin(), 0.0, 256.0 + rad.cos() * 256.0, 256.0 + rad.sin() * 256.0), Vertex(rad2.cos(), rad2.sin(), 0.0, 256.0 + rad2.cos() * 256.0, 256.0 + rad2.sin() * 256.0)]));
        }
        objects.push(Object {
            shape: shapes,
            image: &image
        })
    }

    let scene = Scene {
        width: 1920,
        height: 1080,
        layers: vec![Layer {
            objects
        }]
    };

    let mut fps = fps_counter::FPSCounter::new();

    /* // ベンチマークをしたい時はこの行をコメントアウト
    loop {
        renderer.render(&scene, &canvas);
        block_on(renderer.into_bitmap(&canvas, &mut bit_canvas));
        println!("{:?}", fps.tick());
    }
    // */
    
    println!("render");
    rd.start_frame_capture(std::ptr::null(), std::ptr::null());
    renderer.render(&scene, &canvas);
    rd.end_frame_capture(std::ptr::null(), std::ptr::null());

    println!("into bitmap");
    block_on(renderer.into_bitmap(&canvas, &mut bit_canvas));


    println!("save to png");
    bit_canvas.save_png("test.png").unwrap();

    println!("done");

    let renderer = block_on(WGpuRenderer::new());
    let clip_registory = altmotion::clips::builtin_clip_registory();
    let sequence = altmotion::project::Sequence::default();
    let mut seq_renderer = altmotion::sequence_renderer::SequenceRenderer::<WGpuRenderer>::new(&renderer, &clip_registory, &sequence);
    loop {
        seq_renderer.next(&canvas);
    }
}
