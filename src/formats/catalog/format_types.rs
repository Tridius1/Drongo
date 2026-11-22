// Media format definitions

use crate::formats::{
    AudioFormat, ImageFormat, MediaFormat, VideoFormat,
};

use super::audio_codecs::*;
use super::video_codecs::*;

pub const MP4: MediaFormat = MediaFormat::Video(VideoFormat {
    display_name: "MP4",
    extension: "mp4",
    video_codecs: &[H264, H265],
    audio_codecs: &[AAC, MP3],
});

pub const MP3_FORMAT: MediaFormat = MediaFormat::Audio(AudioFormat {
    display_name: "MP3",
    extension: "mp3",
    codecs: &[MP3],
});

pub const JPEG: MediaFormat = MediaFormat::Image(ImageFormat {
    display_name: "JPEG",
    extension: "jpg",
});
