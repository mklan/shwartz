use clap::Parser;
use minifb::{Key, Window, WindowOptions};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Width of the screen
    #[arg(short, long, default_value_t = 1920)]
    width: usize,

    /// Height of the screen
    #[arg(short, long, default_value_t = 1080)]
    height: usize,

    /// How often a pixel should be blacked out
    #[arg(short, long, default_value_t = 2)]
    every: usize,

    /// Overwrite color
    #[arg(short, long, default_value_t = 0xFF000000)]
    color: u32,
}

fn main() {
    let args = Args::parse();

    let mut buffer: Vec<u32> = vec![0; args.width * args.height];

    let mut window = Window::new(
        "shwartz",
        args.width,
        args.height,
        WindowOptions {
            resize: false,
            topmost: true,
            borderless: true,
            transparency: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max 1 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_millis(1000)));

    while window.is_open() {
        for y in 0..args.height {
            for x in 0..args.width {
                let index = y * args.width + x;
                if (x + y) % args.every == 0 {
                    buffer[index] = args.color;
                } else {
                    buffer[index] = 0x00000000;
                }
            }
        }

        window
            .update_with_buffer(&buffer, args.width, args.height)
            .unwrap();
    }
}