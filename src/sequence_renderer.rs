use std::{cell::RefCell, rc::Rc};

use crate::{clip::{ClipRegistory, ClipRenderContext, ClipRenderer}, core::{Layer, Scene}, project::{Clip, Sequence}, renderer::Renderer};

pub struct SequenceRenderer<'a, T> where T: Renderer + 'a {
    clip_renderer_registory: &'a mut ClipRegistory<T>,
    sequence: &'a Sequence,

    current_frame: u32,
}

impl<'a, T> SequenceRenderer<'a, T> where T: Renderer {
    pub fn new<V>(clip_renderer_registory: &'a mut ClipRegistory<V>, sequence: &'a Sequence) -> SequenceRenderer<'a, V> where V: Renderer {
        SequenceRenderer {
            clip_renderer_registory,
            sequence,
            current_frame: 0
        }
    }

    pub fn jump(&mut self, frame: u32) {
        // TODO: クリップレンダラのクリア
        self.current_frame = frame;
    }

    pub fn next(&mut self, renderer: &mut T, target: &T::Image) {
        let mut scene = Scene::<T::Image> {
            layers: Vec::new(),
            width: self.sequence.width,
            height: self.sequence.height,
        };

        for layer in self.sequence.layers.iter() {
            let current_clip_or_none = layer.clips.get(self.current_frame);
            if let Some(current_clip) = current_clip_or_none {
                let renderer_factory_or_none = self.clip_renderer_registory.get_mut(current_clip.renderer_id);
                if let Some(renderer_factory) = renderer_factory_or_none {
                    let mut clip_renderer = renderer_factory.new(renderer);
                    let objects = clip_renderer.render(&mut ClipRenderContext {
                        renderer
                    });
                    scene.layers.push(Layer {
                        objects
                    });
                }
            }
        }

        renderer.render(&scene, target);

        self.current_frame += 1;
    }

    pub fn current_frame(&self) -> u32 {
        self.current_frame
    }
}
