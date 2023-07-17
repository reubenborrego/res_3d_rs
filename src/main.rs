use space_lib::*;

fn main() {
    let window = window::Window::new(2000.0, 1080.0);
    let resolution = resolution::Resolution::new(3840.0, 2160.0);

    let resolution_point = resolution.new_point(-3840.0 / 2.0, -2160.0 / 2.0, 0.0); //(0.0, 0.0); //(3840.0 / 2.0, 2160.0 / 2.0);

    println!("Resolution {:?}", resolution_point);
    println!(
        "Window {:?}",
        resolution_point.scale_to_window(&resolution, &window)
    );

    let window_border = resolution.window_border(&window);
    println!("Window {:?}", window_border);

    println!(
        "3d {:?}",
        resolution_point
            .scale_to_window(&resolution, &window)
            .scale_to_3d(&window)
    );

    println!(
        "3d aspect ratio applied {:?}",
        resolution_point
            .scale_to_window(&resolution, &window)
            .scale_to_3d(&window)
            .scale_aspect_ratio(&resolution, &window)
    );
}
