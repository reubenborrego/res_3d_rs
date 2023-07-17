use super::d3::Point as D3Point;

// Point in window space
#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


// Border for window space
#[derive(Debug)]
pub struct Border {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

// The actual window
// Exists in (1.0w, 1.0h) space
// IE: 1920 x 1080
//
// Aspect Ratio is based on height over width for more exact numbers
pub struct Window {
    pub width: f32,
    pub height: f32,
    pub aspect_ratio: f32,
}

impl Window {
    // Generate a new window
    pub fn new(width: f32, height: f32) -> Self {
        Window {
            width,
            height,
            aspect_ratio: height / width,
        }
    }
}

impl Point{
    // Scale the window point to 3d rendering (-1.0 to 1.0)
    // Returns to (0.5w, 0.5h) space
    pub fn scale_to_3d(&self, window: &Window) -> D3Point {
        
        // scale = value * 2.0 / total
        let x_conv = 2.0 / (window.width);
        let y_conv = 2.0 / (window.height);

        D3Point {
            x: (self.x - window.width / 2.0) * x_conv,
            y: (self.y - window.height / 2.0) * y_conv,
            z: 0.0,
        }
    }
}