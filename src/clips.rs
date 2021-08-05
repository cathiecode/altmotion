use std::collections::HashMap;

use crate::clip::{Clip, ClipRegistory, ClipRenderContext, ClipRenderer};

pub struct NullClipRenderer {

}

impl<T> ClipRenderer<T> for NullClip {
    fn render(&mut self, context: &ClipRenderContext) -> Vec<crate::core::Object<T>> {
        Vec::new()
    }
}

pub struct NullClip;

impl<T> Clip<T> for NullClip {
    fn new(&mut self) -> Box<dyn ClipRenderer<T>> {
        Box::new(NullClip {})
    }
}

pub fn builtin_clip_registory<T>() -> ClipRegistory<T> {
    let mut reg: ClipRegistory<T> = HashMap::new();
    reg.insert("altmotion.builtin.null", Box::new(NullClip));
    reg
}
