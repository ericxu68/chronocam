// preludes
use chrono::prelude::*;
use opencv::prelude::*;

// opencv imports
use opencv::core;
use opencv::core::Point;
use opencv::core::Rect;
use opencv::core::Scalar;
use opencv::imgcodecs::imwrite;
use opencv::imgproc::put_text;
use opencv::imgproc::rectangle;
use opencv::imgproc::FONT_HERSHEY_PLAIN;
use opencv::imgproc::LINE_AA;
use opencv::types::VectorOfi32;
use opencv::videoio::VideoCapture;
use opencv::videoio::CAP_ANY;
use opencv::videoio::CAP_PROP_FRAME_WIDTH;

// clap imports
use clap::{App, Arg};

// standard imports
use std::env;
use std::fs::create_dir_all;
use std::thread;
use std::time::Duration;

fn capture(interval: Duration, output_dir: &str) -> opencv::Result<()> {
    let mut cam = VideoCapture::new(0, CAP_ANY)?;
    let mut first_frame: bool = true;

    loop {
        // read frame data
        let mut frame = core::Mat::default()?;
        cam.read(&mut frame)?;

        // skip the first frame
        if first_frame {
            first_frame = false;

            // give the camera some time to figure out its life
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        // ensure the frame actually contains data
        if frame.size()?.width == 0 {
            continue;
        }

        // set up the output file name
        let now: DateTime<Local> = Local::now();
        let filename = format!("{}/{}.jpg", output_dir, now.timestamp_nanos());
        let timestamp = now.to_rfc2822();
        let width: i32 = cam.get(CAP_PROP_FRAME_WIDTH).unwrap() as i32;

        if frame.size()?.width > 0 {
            // add timestamp text
            // TODO: adapt text color to time of day (so that black text doesn't get lost in background)
            rectangle(
                &mut frame,
                Rect::new(0, 0, width, 45),
                Scalar::all(0.0),
                -1,
                LINE_AA,
                0,
            )?;
            put_text(
                &mut frame,
                timestamp.as_str(),
                Point::new(10, 32),
                FONT_HERSHEY_PLAIN,
                2.0,
                Scalar::all(255.0),
                2,
                LINE_AA,
                false,
            )?;
            println!("printing {}", filename);
            let params = VectorOfi32::new();
            imwrite(filename.as_str(), &frame, &params)?;
        }
        thread::sleep(interval);
    }
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    let output_dir_default = format!("{}/chronocam-output", current_dir.display());
    let matches = App::new("Chronocam")
        .version("0.1.0")
        .author("Nick Wood <nickwood@hey.com>")
        .about("A simple security timelapse camera")
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .value_name("DELAY_SECONDS")
                .help("Specifies delay (in seconds) between photos")
                .default_value("3")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_DIRNAME")
                .help("The name of the output directory")
                .default_value(output_dir_default.as_str())
                .takes_value(true),
        )
        .get_matches();

    let interval = matches.value_of("interval").unwrap();
    let output = matches.value_of("output").unwrap();
    let interval: u64 = interval.parse::<u64>().unwrap();

    // make space for output
    create_dir_all(output).unwrap();

    // start capturing
    capture(Duration::from_secs(interval), output).unwrap();
}
