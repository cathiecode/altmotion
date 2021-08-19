use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::renderer::Renderer;

pub type Id = &'static str;

pub type ClipRegistory<T> = HashMap<Id, Box<dyn ClipRendererFactory<T>>>;

pub struct ClipRenderContext<'a, T> {
    pub renderer: &'a mut T
}

pub trait ClipRendererFactory<T> where T: Renderer {
    fn new(&mut self, renderer: &mut T) -> Box<dyn ClipRenderer<T>>;
    fn id(&self) -> Id;
}

pub trait ClipRenderer<T> where T: Renderer + 'static {
    fn render(&mut self, context: &mut ClipRenderContext<T>) -> Vec<crate::core::Object<T::Image>>;
}
