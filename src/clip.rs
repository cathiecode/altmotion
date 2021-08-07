use std::{cell::RefCell, collections::HashMap};

pub type Id = &'static str;

pub type ClipRegistory<T> = HashMap<Id, Box<dyn ClipRendererFactory<T>>>;

pub struct ClipRenderContext {
    
}

pub trait ClipRendererFactory<T> {
    fn new(&mut self) -> Box<dyn ClipRenderer<T>>;
    fn id(&self) -> Id;
}

pub trait ClipRenderer<T> {
    fn render(&mut self, context: &ClipRenderContext) -> Vec<crate::core::Object<T>>;
}
