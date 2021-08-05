use std::collections::HashMap;

type Id = &'static str;

pub type ClipRegistory<T> = HashMap<Id, Box<dyn Clip<T>>>;

pub struct ClipRenderContext {
    
}

pub trait Clip<T> {
    fn new(&mut self) -> Box<dyn ClipRenderer<T>>;
}

pub trait ClipRenderer<T> {
    fn render(&mut self, context: &ClipRenderContext) -> Vec<crate::core::Object<T>>;
}
