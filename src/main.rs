use std::error::Error;
use image::{Rgb32FImage, Rgb};
use nalgebra::{vector, point, Unit};
use rand::prelude::*;

mod object;
use object::*;

//Notes:: f32 is 0.0 -> 1.0
//        only seems to work with the openexr format (which gimp can load)
fn main() -> Result<(), Box<dyn Error>> {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const FOV: f64 = 90.0;
        
    //background
    let mut image = Rgb32FImage::new(HEIGHT, WIDTH);
    for row in 0..WIDTH {
        for col in 0..HEIGHT {
            image.put_pixel(col, row, Rgb([0.0, 0.0, 0.0]));
        }
    }

    //Note:: the objects need to be ordered by how close they are to the viewport so our early return works    
    let mut objects = Vec::new();
    let distance_to_viewport = WIDTH as f64 / 2.0 * (90.0 - FOV / 2.0).tan();
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        objects.push(Sphere {
            location: point![
                (rng.gen::<f64>() - 0.5) * WIDTH as f64,
                (rng.gen::<f64>() - 0.5) * HEIGHT as f64,
                distance_to_viewport * (rng.gen::<f64>() + 2.0)
            ],
            radius: rng.gen::<f64>() * 100.0,
            color: [rng.gen(), rng.gen(), rng.gen()],
        });
    }
    objects.sort_unstable_by(|o1, o2| o1.location.coords.magnitude().partial_cmp(&o2.location.coords.magnitude()).unwrap());
    
    for row in 0..WIDTH {
        for col in 0..HEIGHT {
            let ray = vector![row as f64 - WIDTH as f64 / 2.0, col as f64 - HEIGHT as f64 / 2.0, distance_to_viewport];
            let ray_unit = Unit::new_normalize(ray);
            for ele in objects.iter() {
                if ele.hit_test(ray_unit) {
                    image.put_pixel(col, row, Rgb(ele.color));
                    break;
                }
            }
        }
    }
    
    image.save("result.exr")?;
    Ok(())
}
