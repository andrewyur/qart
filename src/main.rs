use qart::qr::Code;

fn main() {
    let code = Code::new(String::from("https://github.com/andrewyur/qart"), 40);
    let img_res = code.build(5, 60, String::from("target.jpg"));
    match img_res {
        Ok(img) => img.save("code.png").unwrap(),
        Err(s) => println!("{}", s),
    }
}
