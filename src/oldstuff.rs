
// width = MARGIN * 3 + pic_1.width + pic_2.width
// height = MARGIN * 2 + taller_picture * 2

// pub struct Picture {
//     pub mut data: RgbaImage,
//     pub width: u32,
//     pub height: u32,
//     pub mut pos_x: u32,
//     pub mut pos_y: u32,
// }

// impl Picture {
//     pub fn new(img: DynamicImage) -> Picture {
//         Picture {
//             data: img.to_rgba(),
//             width: 0,
//             height: 0,
//             pos_x: 0,
//             pos_y: 0,
//         }
//     }

//     // takes two images as input and calculates the coordinates of them in the buffer
//     pub fn calc_position(img1: &Picture, img2: &Picture) {
//         //let height_difference = (img1.height - img2.height).abs();
//         let height_difference = 0;
//         let max_height = cmp::max(img1.height, img2.height);

//         // the x position of the pictures is always the same
//         img1.pos_x = MARGIN;
//         img2.pos_x = MARGIN * 2 + img1.width;

//         // the y position depens on which picture is taller
//         if max_height == img1.height {
//             img1.pos_y = MARGIN;
//             img2.pos_y = MARGIN + (height_difference / 2);
//         } else {
//             img1.pos_y = MARGIN + (height_difference / 2);
//             img2.pos_y = MARGIN;
//         }
//     }
// }

// taller picture:
// y = MARGIN
// x = MARGIN * 2 + other_picture.width

// wider picture:
// x = MARGIn
// y = MARGIN + (height_difference / 2)


// pub fn display_image(image_left: DynamicImage, image_right: DynamicImage) -> Result<(), Error> {

//     // General options for the window itself
//     let window_options = WindowOptions {
//         borderless: false,
//         title: true,
//         resize: false,
//         scale: Scale::X1
//     };

//     // create the window
//     let mut window = Window::new(
//         "Image Viewer", 
//         500 as usize, 
//         500 as usize, 
//         window_options
//     )?;

//     let mut buffer: Vec<u32> = vec![0; 500 as usize * 500 as usize];

//     // STUFF GOES HERER



//     while window.is_open() && !window.is_key_down(Key::Escape) {
//         window.update_with_buffer(&buffer);
//     }

//     Ok(())let rgba_image_left = image_left.to_rgba();
//     let rgba_image_right = image_right.to_rgba();
// }
// pub fn display_image(image_left: DynamicImage, image_right: DynamicImage) -> Result<(), Error> {

//     // General options for the window itself
//     let window_options = WindowOptions {
//         borderless: false,
//         title: true,
//         resize: false,
//         scale: Scale::FitScreen
//     };
    
//     // define the two images
//     let rgba_image_left = Picture::new(i
//             let current_pixel = image_to_use.get_pixel(image_pixel_column, row);
//             let rgba = current_pixel.to_rgba();
//             buffer[index as usize] = pixel_to_buffer_pixel(rgba[0], rgba[1], rgba[2], rgba[3]);  
// mage_left);
//     let rgba_image_rightfor row in 0..window_height {
//         for column in 0..window_width {mensions, calculated
//             let current_pixel = image_to_use.get_pixel(image_pixel_column, row);
//             let rgba = current_pixel.to_rgba();
//             buffer[index as usize] = pixel_to_buffer_pixel(rgba[0], rgba[1], rgba[2], rgba[3]);  
// ba_image_right.height);

//     // create the window
//     let mut window = Window::new(
//         "Image Viewer", 
//         window_width as usize, 
//         window_height as usize, 
//         window_options
//     )?;

//     // create an image buffer
//     let mut buffer: Vec<u32> = vec![0; window_width as usize * window_height as usize];

//     // paint the images (or rather, the whole UI)
//     // we are not looping the buffer, but rather each row and each column
//     for row in 0..window_height {
//         for column in 0..window_width {
//             let index = row * window_width + column;

//             // x range for img1
//             let (img1_start, img1_end) = (
//                 rgba_image_left.pos_x, 
//                 rgba_image_left.pos_x + rgba_image_left.width
//             )

//             // x range for img2
//             let (img2_start, img2_end) = (
//                 rgba_image_right.pos_x, 
//                 rgba_image_right.pos_x + rgba_image_right.width
//             )

//             let color: u32 = pixel_to_buffer_pixel(0, 255, 255, 255);
//             match index {
//                 img1_start..img1_end => {
//                     let buffer_offset_x = column - MARGIN;
//                     let buffer_offset_y = row - rgba_image_left.pos_y;
//                 }
//                 img2_start..img2_end => {

//                 }
//             }
//             buffer[index as usize] = color;

//             // set which column from the image we want to paint in this given buffer pixel
//             // the default (=image starting in the top left corner w/o margin) is just the column counter itself
//             let mut image_pixel_column = column;
//             let mut image_to_use = &rgba_image_left;
            
//             // for the right half
//             if column >= rgba_image_left.dimensions().0 {
//                 image_to_use = &rgba_image_right;
//                 image_pixel_column = column - image_to_use.dimensions().0;
//             }

//             let current_pixel = image_to_use.get_pixel(image_pixel_column, row);
//             let rgba = current_pixel.to_rgba();
//             buffer[index as usize] = pixel_to_buffer_pixel(rgba[0], rgba[1], rgba[2], rgba[3]);  
//         }
//     }

//     while window.is_open() && !window.is_key_down(Key::Escape) {
//         window.update_with_buffer(&buffer);
//     }

//     Ok(())
// }