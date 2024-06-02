// a function to handle opening up the target image and extracting brightness/contrast data from it

// formula to get percieved brightness: ((299 * r + 587 * g + 114 * b) + 500) / 1000
// brightness > 125 -> white, else black

use image::{self, DynamicImage, GenericImageView};

pub fn get_target_scale(path: String, side_len: usize) -> Result<Vec<Vec<(u32, u8)>>, String> {
    let target = match image::open(path) {
        Ok(img) => img,
        Err(_) => return Err(String::from("Could not open target image")),
    };

    let scaled = target.resize_exact(
        side_len as u32,
        side_len as u32,
        image::imageops::FilterType::Gaussian,
    );

    let brightness = make_brightness_array(scaled);

    let mut result = Vec::with_capacity(side_len);

    for y in 0..side_len {
        let mut row = Vec::with_capacity(side_len);
        for x in 0..side_len {
            row.push((
                get_contrast(x as u32, y as u32, &brightness),
                brightness[y][x],
            ))
        }
        result.push(row)
    }

    Ok(result)
}

// pub fn get_target_intersect(path: String) -> Vec<Vec<u8>> {}

fn make_brightness_array(image: DynamicImage) -> Vec<Vec<u8>> {
    let mut brightness_array = Vec::with_capacity(image.height() as usize);
    for y in 0..image.height() {
        let mut row = Vec::with_capacity(image.width() as usize);
        for x in 0..image.width() {
            let p = image.get_pixel(x as u32, y as u32).0;
            row.push(
                (((299 * (p[0] as u32) + 587 * (p[1] as u32) + 114 * (p[2] as u32)) + 500) / 1000)
                    as u8,
            )
        }
        brightness_array.push(row)
    }
    brightness_array
}

fn get_contrast(target_x: u32, target_y: u32, brightness: &Vec<Vec<u8>>) -> u32 {
    let range = 5;

    let mut n = 0;
    let mut sum: usize = 0;
    let mut sum_sequence: usize = 0;

    for offset_y in 0..(range * 2) {
        for offset_x in 0..(range * 2) {
            let pixel_y = (target_y as i32 - range) + offset_y;
            let pixel_x = (target_x as i32 - range) + offset_x;
            if pixel_y >= 0
                && (pixel_y as usize) < brightness.len()
                && pixel_x >= 0
                && (pixel_x as usize) < brightness[0].len()
            {
                let v = brightness[pixel_y as usize][pixel_x as usize];

                sum += v as usize;
                sum_sequence += v as usize * v as usize;

                n += 1;
            }
        }
    }

    let avg = sum / n;
    let contrast = sum_sequence / n - avg * avg;

    contrast as u32
}
