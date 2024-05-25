use qart::qr::Code;

fn main() {
    let code = Code::new(String::from("HI!!!"));
    let img_res = code.build();
    if let Ok(img) = img_res {
        img.save("code.png").unwrap()
    }

    // let img_width = 300;
    // let img_height = 300;

    // let mut img_buf = image::ImageBuffer::new(img_width, img_height);

    // for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
    //     *pixel = image::Rgb([x as u8, y as u8, 0])
    // }

    // img_buf.save("image.png").unwrap();
}
