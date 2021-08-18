use std::{cell::RefCell, collections::HashMap, rc::Rc};

use tiny_skia::Pixmap;

use crate::{clip::{ClipRegistory, ClipRenderContext, ClipRenderer, ClipRendererFactory, Id}, core::{Object, Shape, Vertex}, renderer::Renderer};

pub struct NullClipRenderer {

}

impl<T> ClipRenderer<T> for NullClipRenderer where T: Renderer {
    fn render(&mut self, context: &mut ClipRenderContext<T>) -> Vec<crate::core::Object<T::Image>> {
        Vec::new()
    }
}

pub struct NullClip;

impl<T> ClipRendererFactory<T> for NullClip where T: Renderer {
    fn new(&mut self, renderer: &mut T) -> Box<dyn ClipRenderer<T>> {
        Box::new(NullClipRenderer {})
    }

    fn id(&self) -> Id {
        "altmotion.builtin.null"
    }
}

pub struct TestClipRenderer<T> where T: Renderer {
    texture: T::Image
}

impl<T> ClipRenderer<T> for TestClipRenderer<T> where T: Renderer {
    fn render(&mut self, context: &mut ClipRenderContext<T>) -> Vec<crate::core::Object<T::Image>> {
        println!("load image");
        let mut initial_image = Pixmap::load_png("texture.png").unwrap();
    
        println!("create image");
    
        let mut image = context.renderer.create_image(initial_image.width() as usize, initial_image.height() as usize);
        context.renderer.into_image(initial_image, &image);
    
        let mut boxed_image = Rc::new(RefCell::new(image));
    
        let mut objects: Vec<Object<T::Image>> = Vec::new();
        for _ in 0..100 {
            let mut shapes: Vec<Shape> = Vec::new();
    
            for i in 0..100 {
                let rad = (i as f32)/ 100.0 * std::f32::consts::TAU;
                let rad2 = ((i + 1) as f32) / 100.0 * std::f32::consts::TAU;
                shapes.push(Shape::Triangle([Vertex(0.0, 0.0, 0.0, 256.0, 256.0), Vertex(rad.cos(), rad.sin(), 0.0, 256.0 + rad.cos() * 256.0, 256.0 + rad.sin() * 256.0), Vertex(rad2.cos(), rad2.sin(), 0.0, 256.0 + rad2.cos() * 256.0, 256.0 + rad2.sin() * 256.0)]));
            } 
            objects.push(Object {
                shape: shapes,
                image: boxed_image.clone()
            })
        }
        objects
    }
}

pub struct TestClip;

impl<T> ClipRendererFactory<T> for TestClip where T: Renderer + 'static {
    fn new(&mut self, renderer: &mut T) -> Box<dyn ClipRenderer<T>> {
        Box::new(TestClipRenderer::<T> {
            texture: renderer.create_image(100, 100),
        })
    }

    fn id(&self) -> Id {
        "altmotion.builtin.null"
    }
}

pub fn builtin_clip_registory<T>() -> ClipRegistory<T> where T: Renderer + 'static {
    let mut reg: ClipRegistory<T> = HashMap::new();
    reg.insert("altmotion.builtin.null", Box::new(NullClip));
    reg.insert("altmotion.bulitin.test_clip", Box::new(TestClip));
    reg
}
