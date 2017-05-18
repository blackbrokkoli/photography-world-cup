use image::{DynamicImage, GenericImage};
use minifb::{Window, WindowOptions, Scale, Error, Key};
use image_to_window::image_to_framebuffer;

pub fn display_image(image: DynamicImage) -> Result<(), Error> {
    let (width, height) = image.dimensions();

    let window_options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X1
    };

    let frame_buffer = image_to_framebuffer(image);

    let mut window = Window::new("Image Viewer", width as usize, height as usize, window_options)?;
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&frame_buffer);
    }

    Ok(())
}
