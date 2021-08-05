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

pub struct Object<'a, T> {
    pub image: &'a T,
    pub shape: Vec<Shape>,
}

pub struct Layer<'a, T> {
    pub objects: Vec<Object<'a, T>>,
}

pub struct Scene<'a, T> {
    pub layers: Vec<Layer<'a, T>>,
    pub width: u32,
    pub height: u32,
}
