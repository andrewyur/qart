use qart::qr::Code;

fn main() {
    let code = Code::new(String::from("HI!!!"), 40);
    let img_res = code.build(5);
    match img_res {
        Ok(img) => img.save("code.png").unwrap(),
        Err(s) => println!("{}", s),
    }

    // let img_width = 300;
    // let img_height = 300;

    // let mut img_buf = image::ImageBuffer::new(img_width, img_height);

    // for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
    //     *pixel = image::Rgb([x as u8, y as u8, 0])
    // }

    // img_buf.save("image.png").unwrap();
}
