use crate::{clip::{ClipRegistory, ClipRenderer}, project::Sequence, renderer::Renderer};

pub struct SequenceRenderer<'a, T> where T: Renderer {
    renderer: &'a T,
    clip_registory: &'a ClipRegistory<T>,
    sequence: &'a Sequence,

    current_frame: u32,
    clip_renderers: Vec<Box<dyn ClipRenderer<T>>>
}

impl<'a, T> SequenceRenderer<'a, T> where T: Renderer {
    pub fn new<V>(renderer: &'a V, clip_registory: &'a ClipRegistory<V>, sequence: &'a Sequence) -> SequenceRenderer<'a, V> where V: Renderer {
        SequenceRenderer {
            renderer,
            clip_registory,
            sequence,
            current_frame: 0,
            clip_renderers: Vec::new()
        }
    }

    pub fn jump(&mut self, frame: u32) {
        // TODO: クリップレンダラのクリア
        self.current_frame = frame;
    }

    pub fn next(&mut self, target: &T::Image) {
        // TODO: クリップレンダラのセットアップとレンダリング
        for layer in &self.sequence.layers {
            layer.clips.get(self.current_frame);
        }
        self.current_frame += 1;
    }

    pub fn current_frame(&self) -> u32 {
        self.current_frame
    }
}
