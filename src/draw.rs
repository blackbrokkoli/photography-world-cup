use image::{DynamicImage, RgbaImage, GenericImage, Pixel};
use minifb::{Window, WindowOptions, Scale};

use minifb::ScaleMode::AspectRatioStretch;
use std::cmp;
use log::{debug};
use std::env;

const MARGIN: u32 = 15;
pub const BUFFER_HEIGHT: u32 = 800;
pub const BUFFER_WIDTH: u32 = 1000;

const IMG_WIDTH: u32 = (BUFFER_WIDTH - MARGIN * 3) / 2;

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
    fn write_buffer(mut self, layers: &[Vec<u32>]) -> Self {
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
                    g = Buffer::add_single_channel(g, rgba.1);
                    b = Buffer::add_single_channel(b, rgba.2);
                    a = Buffer::add_single_channel(a, rgba.3);
                }

                self.bytes[index as usize] = Buffer::rgba_to_buffer_pixel(r, g, b, a);
            }
        }
        self
    }

    pub fn buffer_pixel_to_rgba(rgba: u32) -> (u8, u8, u8, u8) {
        let r: u8 = (rgba >> 16) as u8;
        let g: u8 = (rgba >> 8) as u8;
        let b: u8 = rgba as u8;
        let a: u8 = (rgba >> 24) as u8;

        (r, g, b, a)
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

#[derive(Debug)]
struct Coord {
    column: u32,
    row: u32,
}

struct Layer {
    height: u32,
    width: u32,
    // values inclusive I guess?
    corner_top_left: Coord,
    // corner_top_right: Coord,
    corner_bottom_left: Coord,
    corner_bottom_right: Coord,
    image: RgbaImage,
}


impl Layer {
    // Expects that there is exactly one image per layer
    // paints the whole layer
    pub fn paint_layer(&self) -> Vec<u32> {
        let mut buffer: Vec<u32> = vec![0; (self.width * self.height) as usize];
        
        for row in 0..self.height {
            for column in 0..self.width {
                let index = column + row * self.width;

                // check whether we should draw the image
                if row >= self.corner_top_left.row && row < self.corner_bottom_left.row && column >= self.corner_top_left.column && column < self.corner_bottom_right.column
                {
                    // debug!("Trying to draw px (row, column): {} | {}",  row, column);
                    // Account for the space on top and to the left of the image
                    let image_pixel_row = row - self.corner_top_left.row;
                    let image_pixel_column = column - self.corner_top_left.column;

                    // Paint the actual buffer pixel :)
                    let mut rgba;
                    if self.image.dimensions().1 > image_pixel_row && self.image.dimensions().0 > image_pixel_column {
                        // debug!("Iterator: {} | {}, Image Dimensions: {} | {}", image_pixel_row, image_pixel_column, self.image.dimensions().1, self.image.dimensions().0);
                        let current_pixel = self.image.get_pixel(image_pixel_column, image_pixel_row);
                        rgba = current_pixel.to_rgba();
                        buffer[index as usize] = Buffer::rgba_to_buffer_pixel(rgba[0], rgba[1], rgba[2], rgba[3]);
                    } else {
                        // debug!("Fallback to black pixel");
                        buffer[index as usize] = 0;
                    }
                }
            }
        }
        buffer
    }
}

pub fn get_window() -> Window{

    // Draw in Window
    let window_options = WindowOptions {
        borderless: false,
        title: true,
        resize: true,
        scale_mode: AspectRatioStretch,
        scale: Scale::X1,
        topmost: true,
        transparency: false,
    };

    let window = Window::new(
        "UWU FUFU",
        BUFFER_WIDTH as usize,
        BUFFER_HEIGHT as usize,
        window_options
    );

    window.unwrap()
}

pub fn buffer_from_image(image: DynamicImage) -> Buffer {
    env_logger::init;

    debug!("Scaling");

    // Buffer 10, Image 5 => Factor 2
    let image_scaling_factor_by_width = (BUFFER_WIDTH - MARGIN * 2) as f64 / image.dimensions().0 as f64;
    let image_scaling_factor_by_height = (BUFFER_HEIGHT - MARGIN * 2) as f64 / image.dimensions().1 as f64;

    let image_scaling_factor = image_scaling_factor_by_height.min(image_scaling_factor_by_width);
    debug!("Image Original: {} | {}", image.dimensions().0, image.dimensions().1);
    debug!("Scaling: {} - Scaling by Height: {}", image_scaling_factor, image_scaling_factor_by_height);

    let mut image_scaled_width;
    let mut image_scaled_height;

    if image_scaling_factor > 1_f64 {
        debug!("scale up");
        image_scaled_width = image.dimensions().0  as f64;
        image_scaled_height = image.dimensions().1  as f64;
    } else {
        debug!("scale down");
        image_scaled_width = image_scaling_factor * image.dimensions().0  as f64;
        image_scaled_height = image_scaling_factor * image.dimensions().1  as f64;
    }

    debug!("Buffer: {} | {}, Image: {} | {}", BUFFER_WIDTH, BUFFER_HEIGHT, image_scaled_width, image_scaled_height);

    let image_resized = image.resize(
        image_scaled_width as u32,
        image_scaled_height as u32,
        image::imageops::Nearest
    );

    let rgba_image = image_resized.to_rgba();
    debug!("image to rgba done");

    let layer = Layer {
        height: BUFFER_HEIGHT,
        width: BUFFER_WIDTH,
        corner_top_left: Coord {column: MARGIN, row: MARGIN},
        corner_bottom_left: Coord {column: MARGIN, row: rgba_image.dimensions().1 + MARGIN},
        corner_bottom_right: Coord {column: BUFFER_WIDTH - MARGIN, row: rgba_image.dimensions().1 + MARGIN},
        image: rgba_image,
    };

    debug!("paint layer start");
    let painted_layer = layer.paint_layer();
    debug!("paint  layer done");
    let buffer = Buffer::new(BUFFER_WIDTH, BUFFER_HEIGHT);
    buffer.write_buffer(&[painted_layer])
}

pub fn buffer_from_two_images(image_left: DynamicImage, image_right: DynamicImage) -> Buffer {

    // left image rescaling
    let left_image_scaling_factor_by_width = (IMG_WIDTH) as f64 / image_left.dimensions().0 as f64;
    let left_image_scaled_width = left_image_scaling_factor_by_width * image_left.dimensions().0  as f64 ;
    let left_image_scaled_height = left_image_scaling_factor_by_width * image_left.dimensions().1  as f64;
   
    let image_left_resized = image_left.resize(
        left_image_scaled_width as u32, 
        left_image_scaled_height as u32, 
        image::imageops::Nearest
    );

    // right image rescaling
    let right_image_scaling_factor_by_width = (IMG_WIDTH) as f64 / image_right.dimensions().0 as f64;
    let right_image_scaled_width = right_image_scaling_factor_by_width * image_right.dimensions().0  as f64;
    let right_image_scaled_height = right_image_scaling_factor_by_width * image_right.dimensions().1  as f64;
    
    let image_right_resized = image_right.resize(
        right_image_scaled_width as u32, 
        right_image_scaled_height as u32, 
        image::imageops::Nearest
    );

    // convert to rgba
    let rgba_image_left = image_left_resized.to_rgba();
    let rgba_image_right = image_right_resized.to_rgba();

    let layer_image_left = Layer {
        height: BUFFER_HEIGHT,
        width: BUFFER_WIDTH,
        corner_top_left: Coord {column: MARGIN, row: MARGIN},
        // corner_top_right: Coord {column: IMG_WIDTH + MARGIN, row: MARGIN},
        corner_bottom_left: Coord {column: MARGIN, row: rgba_image_left.dimensions().1 + MARGIN},
        corner_bottom_right: Coord {column: IMG_WIDTH + MARGIN, row: rgba_image_left.dimensions().1 + MARGIN},
        image: rgba_image_left,
    };

    let layer_image_right = Layer {
        height: BUFFER_HEIGHT,
        width: BUFFER_WIDTH,
        corner_top_left: Coord {column: MARGIN * 2 + IMG_WIDTH, row: MARGIN},
        // corner_top_right: Coord {column: IMG_WIDTH * 2, row: MARGIN},
        corner_bottom_left: Coord {column: MARGIN * 2 + IMG_WIDTH, row: rgba_image_right.dimensions().1 + MARGIN},
        corner_bottom_right: Coord {column: MARGIN * 2 + IMG_WIDTH * 2, row: rgba_image_right.dimensions().1 + MARGIN},
        image: rgba_image_right,
    };

    // println!("Right Img Layer: {:?} | {:?} | {:?} | {:?}", layer_image_right.corner_top_left, layer_image_right.corner_top_right, layer_image_right.corner_bottom_left, layer_image_right.corner_bottom_right);
    // create and write the buffer(s)
    debug!("begin layer painting");
    let left_buffer = layer_image_left.paint_layer();
    let right_buffer = layer_image_right.paint_layer();
    debug!("both layers painted");

    let buffer = Buffer::new(BUFFER_WIDTH, BUFFER_HEIGHT);
    buffer.write_buffer(&[left_buffer, right_buffer])

}