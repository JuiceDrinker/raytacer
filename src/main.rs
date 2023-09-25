mod ray;
mod vec3;

use std::{fs::File, io::Write};

use crate::vec3::Vec3;

fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    let mut data = format!("P3\n{} {} \n255\n", image_width, image_height);

    let mut file = File::create("img.ppm").expect("Could not create file");

    for j in 0..image_height {
        for i in 0..image_width {
            log::debug!("\rScanlines lwefaeft: {}", image_height - j);
            let pixel_color = Vec3::new(
                i as f32 / (image_width as f32 - 1.00),
                j as f32 / (image_height as f32 - 1.00),
                0.00,
            );

            data.push_str(&pixel_color.to_pixel_row());
        }
    }

    let _ = file.write_all(data.as_bytes());
    log::debug!("\rDone!       \n");
}
