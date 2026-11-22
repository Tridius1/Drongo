#![allow(unused)]

mod formats;

use crate::formats::{
    MediaFormat,
    catalog::{format_types, audio_codecs, video_codecs},
};



fn main() {
    let format = format_types::MP4;
    let codec = video_codecs::H264;

    println!("{}", format.display_name());
    println!("{}", codec.display_name());
}
