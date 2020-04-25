use std::error::Error;
use std::time::Instant;
use std::env;
use getopts::Options;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    
    opts.optopt("d", "device", "Select device index", "NUMBER");
    opts.optopt("w", "width", "Capture width", "NUMBER");
    opts.optopt("h", "height", "Capture height", "NUMBER");
    opts.optopt("f", "fps", "Desired capture FPS", "NUMBER");
    opts.optflag("", "help", "Print this help menu");
    
    let matches = opts.parse(&args[1..])?;
    
    if matches.opt_present("help") {
        print_usage(&program, opts);
        return Ok(());
    }
    
    let device = matches.opt_get("d")?.unwrap_or(0);
    let width = matches.opt_get("w")?.unwrap_or(1920);
    let height = matches.opt_get("h")?.unwrap_or(960);
    let fps = matches.opt_get("f")?.unwrap_or(60);
    
    let device = escapi::init(device, width, height, fps)?;
    
    println!("Camera {}: {}x{}", device.name(), device.capture_width(), device.capture_height());
    
    let mut last_capture = Instant::now();
    
    loop {
        let frame = match device.capture() {
            Ok(frame) => frame,
            Err(escapi::Error::CaptureTimeout) => continue,
            Err(err) => panic!("Failed to capture frame {}", err),
        };
        println!("{} FPS\t{}", 1.0 / last_capture.elapsed().as_secs_f32(), frame.len());
        last_capture = Instant::now();
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
