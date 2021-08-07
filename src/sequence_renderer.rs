use std::{cell::RefCell, rc::Rc};

use crate::{clip::{ClipRegistory, ClipRenderContext, ClipRenderer}, core::{Layer, Scene}, project::{Clip, Sequence}, renderer::Renderer};

pub struct SequenceRenderer<'a, T> where T: Renderer {
    renderer: &'a T,
    clip_renderer_registory: &'a mut ClipRegistory<T>,
    sequence: &'a Sequence,

    current_frame: u32,
    prev_clips: Vec<Option<(&'a Clip, Rc<RefCell<Box<dyn ClipRenderer<T>>>>)>>
}

impl<'a, T> SequenceRenderer<'a, T> where T: Renderer {
    pub fn new<V>(renderer: &'a V, clip_renderer_registory: &'a mut ClipRegistory<V>, sequence: &'a Sequence) -> SequenceRenderer<'a, V> where V: Renderer {
        SequenceRenderer {
            renderer,
            clip_renderer_registory,
            sequence,
            current_frame: 0,
            prev_clips: std::iter::repeat_with(|| None).take(sequence.layers.len()).collect()
        }
    }

    pub fn jump(&mut self, frame: u32) {
        // TODO: クリップレンダラのクリア
        self.current_frame = frame;
    }

    pub fn next(&mut self, target: &T::Image) {
        let mut scene = Scene::<T> {
            layers: Vec::new(),
            width: self.sequence.width,
            height: self.sequence.height,
        };
        for (i, layer) in self.sequence.layers.iter().enumerate() {
            let current_clip_or_none = layer.clips.get(self.current_frame);
            if let Some(current_clip) = current_clip_or_none {
                let clip_renderer_ref = self.get_current_clip_renderer(current_clip, i);
                let objects = clip_renderer_ref.borrow_mut().render(&ClipRenderContext {});
                scene.layers.push(Layer {
                    objects
                });
            } else {
                continue;
            }
        }
        self.current_frame += 1;
    }

    pub fn current_frame(&self) -> u32 {
        self.current_frame
    }

    fn get_current_clip_renderer(&mut self, current_clip: &'a Clip, layer_number: usize) -> Rc<RefCell<Box<dyn ClipRenderer<T>>>> {
        {
            let prev_clip_or_none = self.prev_clips.get_mut(layer_number).unwrap();
            if let Some(prev_clip) = prev_clip_or_none {
                if current_clip.renderer_id == prev_clip.0.renderer_id {
                    return prev_clip.1.clone();
                }
            }
        }
        let renderer = self.clip_renderer_registory.get_mut(current_clip.renderer_id).unwrap().new();
        self.prev_clips[layer_number] = Some((current_clip, Rc::new(RefCell::<Box<dyn ClipRenderer<T>>>::new(renderer))));
        return self.prev_clips[layer_number].as_mut().unwrap().1.clone();
    }
    
}
