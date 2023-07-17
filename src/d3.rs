use super::{window::Window, resolution::Resolution};

#[derive(Debug)]
// Point in resolution space
pub struct Point{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    // Commit the point to cgmath Vector3 via the resolution x window aspect ratio conversion
    pub fn scale_aspect_ratio(&self, resolution: &Resolution, window: &Window) -> cgmath::Vector3<f32> {
        let ar_scale = resolution.window_aspect_ratio_scale(window);

        cgmath::Vector3 {
            x: self.x * ar_scale.0,
            y: self.y * ar_scale.1,
            z: self.z,
        }
    }
}