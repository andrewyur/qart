use qart::qr::Code;
// use qart::target;

fn main() {
    let code = Code::new(String::from("https://github.com/andrewyur/qart"), 40);
    let img_res = code.build(5, 60, String::from("target.jpg"), 100);
    match img_res {
        Ok(img) => img.save("mascot.png").unwrap(),
        Err(s) => println!("{}", s),
    }

    // target::preview(String::from("target.png"), 40, 128)
}
