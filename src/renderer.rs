use crate::core;
use async_trait::async_trait;
use tiny_skia::Pixmap;

#[async_trait]
pub trait Renderer {
    type Image;
    fn render(&mut self, scene: &core::Scene<Self::Image>, dest: &Self::Image); // TODO: TargetをImageで受けとるようにする
    fn create_image(&mut self, width: usize, height: usize) -> Self::Image;
    fn into_image(&mut self, bitmap: Pixmap, image: &Self::Image);
    async fn into_bitmap(&mut self, image: &Self::Image, dest: &mut Pixmap);
}
