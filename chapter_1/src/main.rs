use image::{ImageBuffer, Rgba};

fn main() {
    let imgx = 200;
    let imgy = 100;

    // Define the image buffer with explicit type

    // Or alternatively, define the type directly with generics
    let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = ((x) as f32 / (imgx) as f32 * 255.0) as u8;
        let g = ((imgy - y) as f32 / (imgy) as f32 * 255.0) as u8;
        let b = (0.2 * 255.0) as u8;
        let a = 255;
        *pixel = Rgba([r, g, b, a]);
    }

    imgbuf.save("red_image.png").unwrap();
}
