use clap::{Parser, Subcommand};
use qart::qr;

#[derive(Parser)]
#[command(name = "qart")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a functional QR code that looks like the provided image
    Build {
        /// Version number (size) of the QR code: 1-40
        version: u32,
        /// URL that the QR code will point to. Should not contain URL fragments or query strings.
        url: String,
        /// Relative path of the target image that the QR code will look like
        image_path: String,
        /// Path that the produced QR code will be saved to. Default is "code.png"
        #[arg(long, default_value = "code.png")]
        save_path: String,
        /// The side length of each of the modules of the QR code in pixels. Default is 5
        #[arg(long, default_value_t = 5)]
        module_size: u32,
        /// The brightness value at which brighter pixels will be white, and darker pixels will be black. Default is 128
        #[arg(long, default_value_t = 128)]
        threshold: u8,
        /// Display the time taken to generate the QR code
        #[arg(long)]
        benchmark: bool,
        /// Distribute uncontrollable pixels randomly instead of based off of contrast
        #[arg(long)]
        random: bool,
    },
    /// Generate a preview of a QR code that will quickly show what the image will look like as part of the QR code
    Preview {
        /// Version number (size) of the QR code: 1-40
        version: u32,
        /// Relative path of the target image that the QR code will look like
        image_path: String,
        /// Path that the produced QR code will be saved to. Default is "preview.png"
        #[arg(long, default_value = "preview.png")]
        save_path: String,
        /// The brightness value at which brighter pixels will be white, and darker pixels will be black. Default is 128
        #[arg(long, default_value_t = 128)]
        threshold: u8,
        /// Distribute uncontrollable pixels randomly instead of based off of contrast
        #[arg(long)]
        random: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            version,
            url,
            image_path,
            save_path,
            module_size,
            threshold,
            benchmark,
            random,
        } => {
            let start = std::time::Instant::now();
            let code = qr::build(version, url, module_size, image_path, threshold, random);
            match code {
                Ok(img) => {
                    img.save(save_path).unwrap();
                    if benchmark {
                        println!("Time Elapsed: {:?}", start.elapsed());
                    }
                }
                // TODO: make this print to stderr
                Err(s) => println!("{}", s),
            }
        }
        Commands::Preview {
            version,
            image_path,
            save_path,
            threshold,
            random,
        } => {
            let code = qr::preview(version, image_path, threshold, random);
            match code {
                Ok(img) => {
                    img.save(save_path).unwrap();
                }
                Err(s) => println!("{}", s),
            }
        }
    }
}

// times for v40 qr code
// non threaded: 14.73s
// threaded: 6.52s
// threaded and remove unnecessary calls: 6.72s (?)
// compiled: 284.87ms
