use image::{DynamicImage, GenericImage, Pixel};

pub fn image_to_framebuffer(image: DynamicImage) -> Vec<u32> {
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