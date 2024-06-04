use qart::qr;

fn main() {
    let start = std::time::Instant::now();

    let code = qr::build(
        40,
        String::from("https://github.com/andrewyur/qart"),
        5,
        String::from("target.jpg"),
        100,
    );
    // let code = qr::preview(40, String::from("target.jpg"), 100);
    match code {
        Ok(img) => img.save("code.png").unwrap(),
        Err(s) => println!("{}", s),
    }

    println!("Time Elapsed: {:?}", start.elapsed());
}

// for v40 qr code
// non threaded: 14.73s
// threaded: 6.52s
// threaded and remove unnecessary calls:
