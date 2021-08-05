#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod core;
pub mod clip;
pub mod clips;
pub mod project;
pub mod renderer;
pub mod sequence_renderer;
pub mod wgpu_renderer;
