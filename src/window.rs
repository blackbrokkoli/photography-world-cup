use image::{DynamicImage, RgbaImage, GenericImage, Pixel, Rgba};

use minifb::{Window, WindowOptions, Scale, Error, Key};
use image_to_window::image_to_framebuffer;

use std::cmp;
use std::num;

const MARGIN: u32 = 15;

pub struct Buffer {
    pub bytes: Vec<u32>,
    pub height: u32,
    pub width: u32,
}

impl Buffer {
    fn new(width: u32, height: u32) -> Buffer {
        Buffer {
            bytes: vec![0; (width * height) as usize],
            width: width,
            height: height,
        }
    }

    // iterates over the buffer and adds the rgba channels of all passed layers
    // on a per pixel basis
    fn write_buffer(&mut self, layers: &[Vec<u32>]) {
        for y in 0..(self.height) {
            for x in 0..(self.width) {
                let index = x + y * self.width;
                let mut r: u8 = 0;
                let mut g: u8 = 0;
                let mut b: u8 = 0;
                let mut a: u8 = 0;

                for layer in layers {
                    let rgba = Buffer::buffer_pixel_to_rgba(layer[index as usize]);
                    r = Buffer::add_single_channel(r, rgba.0);
                    b = Buffer::add_single_channel(b, rgba.1);
                    g = Buffer::add_single_channel(g, rgba.2);
                    a = Buffer::add_single_channel(a, rgba.3);
                }

                self.bytes[index as usize] = Buffer::rgba_to_buffer_pixel(r, g, b, a);
            }
        }
    }


    pub fn buffer_pixel_to_rgba(rgba: u32) -> (u8, u8, u8, u8) {
        (255, 255, 255, 255)
    }

    // 
    pub fn rgba_to_buffer_pixel(red: u8, green: u8, blue: u8, alpha: u8) -> u32 {
        let mut argb: u32 = 0;
        argb |= (alpha as u32) << 24;
        argb |= (red as u32) << 16;
        argb |= (green as u32) << 8;
        argb |= blue as u32;
        argb
    }    

    pub fn add_single_channel(channel1: u8, channel2: u8) -> u8 {
        if (channel1 as u32 + channel2 as u32) > 255 {
            return 255;
        }

        channel1 + channel2
    }
}

struct Coord {
    column: u32,
    row: u32,
}

struct Layer {
    height: u32,
    width: u32,
    // values inclusive I guess?
    corner_top_left: Coord,
    corner_top_right: Coord,
    corner_bottom_left: Coord,
    corner_bottom_right: Coord,
    image: RgbaImage,
}


// TODO: There is no buffer to actually paint in yet
impl Layer {
    // Expects that there is exactly one image per layer
    // paints the whole layer
    pub fn paint_layer(&self) -> Vec<u32> {
        
        let mut buffer: Vec<u32> = vec![0; self.width as usize * self.height as usize];
        
        for row in 0..self.height {
            for column in 0..self.width {
                let index = column + row * self.width;

                // check whether we should draw the image
                if row >= self.corner_top_left.row && row <= self.corner_bottom_left.row && column >= self.corner_top_left.column && column <= self.corner_bottom_left.column 
                {
                    // Account for the space on top and to the left of the image
                    let image_pixel_row = row - self.corner_top_left.row;
                    let image_pixel_column = column - self.corner_top_left.column;

                    // Paint the actual buffer pixel :)
                    let current_pixel = self.image.get_pixel(image_pixel_column, image_pixel_row);
                    let rgba = current_pixel.to_rgba();
                    buffer[index as usize] = Buffer::rgba_to_buffer_pixel(rgba[0], rgba[1], rgba[2], rgba[3]);  
                }
            }
        }

        buffer
    }
}


// Define which elements and where;
// TODO: What to return?
pub fn draw_uwufufu(image_left: DynamicImage, image_right: DynamicImage) -> Result<(), Error>  {
    let rgba_image_left = image_left.to_rgba();
    let rgba_image_right = image_right.to_rgba();
    // TODO: scale images (not needed for functionality actually, will just cut off)
    
    let layer_image_left = Layer {
        height: 500,
        width: 500,
        corner_top_left: Coord {column: 10, row: 10},
        corner_top_right: Coord {column: 245, row: 10},
        corner_bottom_left: Coord {column: 10, row: 490},
        corner_bottom_right: Coord {column: 245, row: 490},
        image: rgba_image_left,
    };

    let layer_image_right = Layer {
        height: 500,
        width: 500,
        corner_top_left: Coord {column: 255, row: 10},
        corner_top_right: Coord {column: 495, row: 10},
        corner_bottom_left: Coord {column: 255, row: 490},
        corner_bottom_right: Coord {column: 495, row: 490},
        image: rgba_image_right,
    };

    // create and write the buffer(s)
    let left_buffer = layer_image_left.paint_layer();
    let right_buffer = layer_image_right.paint_layer();

    let mut buffer = Buffer::new(500, 500);
    buffer.write_buffer(&[left_buffer, right_buffer]);
  
    // Draw in Window
    let window_options = WindowOptions {
                borderless: false,
                title: true,
                resize: false,
                scale: Scale::X1
            };

    let mut window = Window::new(
        "Image Viewer", 
        500 as usize, 
        500 as usize, 
        window_options
    )?;
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer.bytes);
    }

    Ok(())
}
