use std::{fs::File, io::Write};

fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    let mut data = format!("P3\n{} {} \n255\n", image_width, image_height);

    let mut file = File::create("img.ppm").expect("Could not create file");

    for j in 0..image_height {
        for i in 0..image_width {
            log::debug!("\rScanlines left: {}", image_height - j);
            let r: f32 = i as f32 / (image_width as f32 - 1.00);
            let g: f32 = j as f32 / (image_height as f32 - 1.00);
            let b: f32 = 0.00;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            let string = format!("{} {} {}\n", ir, ig, ib);

            data.push_str(&string);
        }
    }

    let _ = file.write_all(data.as_bytes());
}
