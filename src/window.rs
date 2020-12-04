use image::{DynamicImage, GenericImage, Pixel};

use minifb::{Window, WindowOptions, Scale, Error, Key};
use image_to_window::image_to_framebuffer;
use std::cmp;



pub fn display_image(image: DynamicImage) -> Result<(), Error> {

    let window_options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::FitScreen
    };
    
    // define images
    // type RgbaImage = ImageBuffer<Rgba<u8>, Vec<u8>>;
    // Sendable Rgb + alpha channel image buffer
    let rgba_image_left = image.to_rgba();
    let rgba_image_right = image.to_rgba();

    let window_width = rgba_image_left.dimensions().0 + rgba_image_right.dimensions().0;
    let window_height = cmp::max(rgba_image_left.dimensions().1, rgba_image_right.dimensions().1);

    let mut window = Window::new(
        "Image Viewer", 
        window_width as usize, 
        window_height as usize, 
        window_options
    )?;

    // let frame_buffer = image_to_framebuffer(image);
    let mut buffer: Vec<u32> = vec![0; window_width as usize * window_height as usize];
    
    // for i in buffer.iter_mut() {
    //     //println!("uwu what dis {}", i);
    // }
    
    // Loop rows

    // Image A is 3 wide
    // Image B is 5 wide
    // 0-2 should be A
    // 3-7 should be B

    // In Column 4 we want to get B pixel 2
    // Getting B pixel in general:
    // Column + 1 - A.width 
    // 4 + 1 - 3 = 2


    for row in 0..window_height {
        for column in 0..window_width {
            let index = row * window_width + column;
            if column < rgba_image_left.dimensions().0 {
                let relevant_image_pixel = rgba_image_left.get_pixel(column, row);
                let rgba = relevant_image_pixel.to_rgba();
                let buffer_pixel = pixel_to_buffer_pixel(rgba[0], rgba[1], rgba[2], rgba[3]);
                buffer[index as usize] = buffer_pixel;
                // buffer[index as usize] = pixel_to_buffer_pixel(0, 125, 45, 57); 
            } else {
                let image_pixel_column = column- rgba_image_right.dimensions().0;
                let relevant_image_pixel = rgba_image_right.get_pixel(image_pixel_column, row);
                let rgba = relevant_image_pixel.to_rgba();
                let buffer_pixel = pixel_to_buffer_pixel(rgba[0], rgba[1], rgba[2], rgba[3]);
                buffer[index as usize] = buffer_pixel;
                // buffer[index as usize] = pixel_to_buffer_pixel(0, 52, 107, 235); 
            }
        }
    }
        // Loop cols
            // buffer(cols * rowlength)(cols)

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer);
    }


    
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     window.update_with_buffer(&frame_buffer);
    // }

    Ok(())
}

pub fn pixel_to_buffer_pixel(red: u8, green: u8, blue: u8, alpha: u8) -> u32 {
    let mut argb: u32 = 0;
    argb |= (alpha as u32) << 24;
    argb |= (red as u32) << 16;
    argb |= (green as u32) << 8;
    argb |= blue as u32;
    argb
}
 
// pub fn image_to_framebuffer(image: DynamicImage) -> Vec<u32> {
//     let rgba_image = image.to_rgba();
//     let framebuffer: Vec<u32> = rgba_image.pixels().map(|rgba| rgba.channels4())
//         .map(|(r, g, b, a)| {
//             let red: u8 = r.into();
//             let green: u8 = g.into();
//             let blue: u8 = b.into();
//             let alpha: u8 = a.into();
//             let mut argb: u32 = 0;
//             argb |= (alpha as u32) << 24;
//             argb |= (red as u32) << 16;
//             argb |= (green as u32) << 8;
//             argb |= blue as u32;

//             argb
//         }).collect();
    
//     framebuffer
// }