use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::clip::{ClipRegistory, ClipRenderContext, ClipRenderer, ClipRendererFactory, Id};

pub struct NullClipRenderer {

}

impl<T> ClipRenderer<T> for NullClip {
    fn render(&mut self, context: &ClipRenderContext) -> Vec<crate::core::Object<T>> {
        Vec::new()
    }
}

pub struct NullClip;

impl<T> ClipRendererFactory<T> for NullClip {
    fn new(&mut self) -> Box<dyn ClipRenderer<T>> {
        Box::new(NullClip {})
    }

    fn id(&self) -> Id {
        "altmotion.builtin.null"
    }
}

pub fn builtin_clip_registory<T>() -> ClipRegistory<T> {
    let mut reg: ClipRegistory<T> = HashMap::new();
    reg.insert("altmotion.builtin.null", Box::new(NullClip));
    reg
}
