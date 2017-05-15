use image::{DynamicImage, GenericImage, Pixel};
use minifb::{Window, WindowOptions, Scale, Error, Key};

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

fn image_to_framebuffer(image: DynamicImage) -> Vec<u32> {
    let rgba_image = image.to_rgba();
    let framebuffer: Vec<u32> = rgba_image.pixels().map(|rgba| rgba.channels4())
        .map(|(r, g, b, a)| {
            let red: u8 = r.into();
            let green: u8 = g.into();
            let blue: u8 = b.into();
            let alpha: u8 = a.into();
            let mut argb: u32 = 0;
            argb |= (alpha as u32) << 24;
            argb |= (red as u32) << 16;
            argb |= (green as u32) << 8;
            argb |= blue as u32;

            argb
        }).collect();
    
    framebuffer
}
