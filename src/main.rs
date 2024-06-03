use qart::qr;

fn main() {
    // let code = qr::build(
    //     40,
    //     String::from("https://github.com/andrewyur/qart"),
    //     5,
    //     String::from("target.jpg"),
    //     100,
    // );
    let preview = qr::preview(40, String::from("target.jpg"), 100);
    match preview {
        Ok(img) => img.save("code.png").unwrap(),
        Err(s) => println!("{}", s),
    }
}
