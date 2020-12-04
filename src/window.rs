use image::{DynamicImage, GenericImage, Pixel};

use minifb::{Window, WindowOptions, Scale, Error, Key};
use image_to_window::image_to_framebuffer;
use std::cmp;


pub fn display_image(image_left: DynamicImage, image_right: DynamicImage) -> Result<(), Error> {

    // General options for the window itself
    let window_options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::FitScreen
    };
    
    // define the two images
    let rgba_image_left = image_left.to_rgba();
    let rgba_image_right = image_right.to_rgba();

    // set the window dimensions, calculated via the image dimensions
    let window_width = rgba_image_left.dimensions().0 + rgba_image_right.dimensions().0;
    let window_height = cmp::max(rgba_image_left.dimensions().1, rgba_image_right.dimensions().1);

    // create the window
    let mut window = Window::new(
        "Image Viewer", 
        window_width as usize, 
        window_height as usize, 
        window_options
    )?;

    // create an image buffer
    let mut buffer: Vec<u32> = vec![0; window_width as usize * window_height as usize];

    // paint the images (or rather, the whole UI)
    // we are not looping the buffer, but rather each row and each column
    for row in 0..window_height {
        for column in 0..window_width {
            // calculate the index in the onedimensional buffer given the two dimensional coords
            let index = row * window_width + column;
            // set which column from the image we want to paint in this given buffer pixel
            // the default (=image starting in the top left corner w/o margin) is just the column counter itself
            let mut image_pixel_column = column;
            let mut image_to_use = &rgba_image_left;
            
            // for the right half
            if column >= rgba_image_left.dimensions().0 {
                image_to_use = &rgba_image_right;
                image_pixel_column = column - image_to_use.dimensions().0;
            }

            let current_pixel = image_to_use.get_pixel(image_pixel_column, row);
            let rgba = current_pixel.to_rgba();
            buffer[index as usize] = pixel_to_buffer_pixel(rgba[0], rgba[1], rgba[2], rgba[3]);  
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer);
    }

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