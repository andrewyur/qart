use qart::qr::Code;

fn main() {
    let code = Code::new(String::from("https://testing.com"), 20);
    let img_res = code.build(5, 60);
    match img_res {
        Ok(img) => img.save("code.png").unwrap(),
        Err(s) => println!("{}", s),
    }

    // let target = image::open("target.png").unwrap();
}
