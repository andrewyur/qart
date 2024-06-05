use qart::qr;

fn main() {
    let start = std::time::Instant::now();

    // takes the version number (1-40), the target string that the qr code will point to,
    // the width/height of each of the squares(modules) of the qr code in pixels,
    // the file path to your image, and the brightness threshold(0-255) which decides
    // what color pixels of a given brightness will be in the final qr code.
    let code = qr::build(
        38,
        String::from("https://rat.com"),
        5,
        String::from("target.png"),
        100,
    );
    // takes version number, path to image, and brightness threshold
    // let code = qr::preview(40, String::from("target.jpg"), 100);
    match code {
        Ok(img) => img.save("mascot2.png").unwrap(),
        Err(s) => println!("{}", s),
    }

    println!("Time Elapsed: {:?}", start.elapsed());
}

// times for v40 qr code
// non threaded: 14.73s
// threaded: 6.52s
// threaded and remove unnecessary calls: 6.72s (?)
