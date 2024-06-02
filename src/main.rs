use qart::qr::Code;

fn main() {
    let code = Code::new(String::from("https://testing.com"), 40);
    let img_res = code.build(5);
    match img_res {
        Ok(img) => img.save("code.png").unwrap(),
        Err(s) => println!("{}", s),
    }

    // let target = image::open("target.png").unwrap();
}
