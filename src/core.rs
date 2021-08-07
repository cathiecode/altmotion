use std::{cell::RefCell, rc::Rc};

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex(pub f32, pub f32, pub f32, pub f32, pub f32);

impl Into<[f32; 5]> for Vertex {
    fn into(self) -> [f32; 5] {
        [self.0, self.1, self.2, self.3, self.4]
    }
}

//pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

pub type Triangle = [Vertex; 3];

// データ構造

pub enum Shape {
    Triangle(Triangle),
}

pub struct Object<T> {
    pub image: Rc<RefCell<T>>,
    pub shape: Vec<Shape>,
}

pub struct Layer<T> {
    pub objects: Vec<Object<T>>,
}

pub struct Scene<T> {
    pub layers: Vec<Layer<T>>,
    pub width: u32,
    pub height: u32,
}
