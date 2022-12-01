use std::error::Error;
use image::{Rgb32FImage, Rgb};

//Notes:: f32 is 0.0 -> 1.0
//        only seems to work with the openexr format (which gimp can load)
fn main() -> Result<(), Box<dyn Error>> {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const COLOR_STEP: f32 = 1.0 / (WIDTH * HEIGHT) as f32;
    let mut image = Rgb32FImage::new(HEIGHT, WIDTH);
    for row in 0..WIDTH {
        for col in 0..HEIGHT {
            image.put_pixel(col, row, Rgb([
                (row * WIDTH + col) as f32 * COLOR_STEP,
                0.0,
                 1.0 - (row * WIDTH + col) as f32 * COLOR_STEP,
            ]));
        }
    }
    image.save("result.exr")?;
    Ok(())
}
