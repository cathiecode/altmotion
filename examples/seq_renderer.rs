use std::collections::HashMap;

use altmotion::clips::NullClip;
use altmotion::core::*;
use altmotion::project::Clip;
use altmotion::renderer::*;
use altmotion::wgpu_renderer::*;
use timeliner::Timeline;
use tiny_skia::Pixmap;

use async_std::task::block_on;
use renderdoc::{RenderDoc, V100, V110};

fn main() {
    env_logger::init();

    let mut renderer = block_on(WGpuRenderer::new());

    println!("create canvas");
    let canvas = renderer.create_image(1920, 1080);

    println!("create bitmap output");
    let mut bit_canvas = Pixmap::new(1920, 1080).unwrap();

    let mut fps = fps_counter::FPSCounter::new();

    let mut clip_registory = altmotion::clips::builtin_clip_registory();

    let sequence = altmotion::project::Sequence {
        layers: vec![
            altmotion::project::Layer {
                name: "layer 1".to_owned(),
                clips: {
                    let mut timeline = Timeline::new();
                    timeline.insert(Clip {
                        name: "Test clip".to_owned(),
                        start: 0,
                        end: 10,
                        props: Vec::new(),
                        renderer_id: "altmotion.builtin.test_clip"
                    }).unwrap();

                    timeline
                }
            }
        ],
        clips: HashMap::new(),
        width: 1920,
        height: 1080,
    };

    let mut rd: RenderDoc<V110> = RenderDoc::new().expect("Unable to connecat");

    let mut seq_renderer = altmotion::sequence_renderer::SequenceRenderer::<WGpuRenderer>::new(&mut clip_registory, &sequence);
    rd.start_frame_capture(std::ptr::null(), std::ptr::null());
    seq_renderer.next(&mut renderer, &canvas);
    rd.end_frame_capture(std::ptr::null(), std::ptr::null());
    block_on(renderer.into_bitmap(&canvas, &mut bit_canvas));
    bit_canvas.save_png("test.png".to_string()).unwrap();
    println!("Sequence done");
}
