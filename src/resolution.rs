use super::window::{Border as WindowBorder, Point as WindowPoint, Window};

// The expected resolution
// Exists in (0.5w, 0.5h) space
// IE: 3840 x 2160
//
// Aspect Ratio is based on height over width for more exact numbers
//
// Note: for 3D, resolution can be used to perform aspect ratio calculations
pub struct Resolution {
    width: f32,
    height: f32,
    aspect_ratio: f32,
}

// Point in resolution space
#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Resolution {
    // Generate new resolution based on width and height
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            aspect_ratio: height / width,
        }
    }

    // Generate a new point in the resolutions space
    pub fn new_point(&self, x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }

    // Provide scaling to window
    fn window_scale(&self, window: &Window) -> (f32, f32) {
        (window.width / self.width, window.height / self.height)
    }

    // Get the window scale aspect ratio based on the resolution's aspect ratio
    pub fn window_aspect_ratio_scale(&self, window: &Window) -> (f32, f32) {
        // Get the w and h values from window as expected by the resolution's aspect ratio
        let (width_expected, height_expected) = if window.aspect_ratio < self.aspect_ratio {
            (window.height / self.aspect_ratio, window.height)
        } else {
            (window.width, window.width * self.aspect_ratio)
        };

        // The scale is expected_value divided by the current_value
        // Will be <= 1.0
        // IE: expected == current -> 1.0
        let width_scale = width_expected / window.width;
        let height_scale = height_expected / window.height;

        (width_scale, height_scale)
    }

    // Get the border values for the window based on the resolution's aspect ratio
    pub fn window_border(&self, window: &Window) -> WindowBorder {
        let window_aspect_ratio_scale = self.window_aspect_ratio_scale(window);

        // current value - (current value * aspect ratio value)
        let x_cut_space = window.width - (window.width * window_aspect_ratio_scale.0);
        let y_cut_space = window.height - (window.height * window_aspect_ratio_scale.1);

        // Pad both sides (top and bottom)
        let x_cut_space_half = x_cut_space / 2.0;
        let y_cut_space_half = y_cut_space / 2.0;

        // Returning to (1.0w, 1.0h) space
        WindowBorder {
            top: window.height - y_cut_space_half,
            bottom: y_cut_space_half,
            left: x_cut_space_half,
            right: window.width - x_cut_space_half,
        }
    }
}

impl Point {
    // Scale the point to the current window dimensions
    pub fn scale_to_window(&self, resolution: &Resolution, window: &Window) -> WindowPoint {
        let window_scale = resolution.window_scale(window);

        // Returning to (1.0w, 1.0h) space
        // resolution_x * window_scale.x + (window_w / 2.0)
        WindowPoint {
            x: self.x * window_scale.0 + window.width / 2.0,
            y: self.y * window_scale.1 + window.height / 2.0,
            z: self.z,
        }
    }
}
