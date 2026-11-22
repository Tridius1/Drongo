#![allow(unused)]

mod formats;

use crate::formats::{
    MediaFormat,
    catalog::{format_types, audio_codecs, video_codecs},
};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let format = format_types::MP4;
    let codec = video_codecs::H264;

    println!("{}", format.display_name());
    println!("{}", codec.display_name());

    // Launch GUI
    let main_window = MainWindow::new()?;
    Ok(main_window.run()?)
}
