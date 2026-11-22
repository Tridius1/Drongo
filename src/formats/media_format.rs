// Defining and implementing media format struct

use crate::formats::Codec;

#[derive(Clone, Copy)]
pub enum MediaFormat {
    Image(ImageFormat),
    Audio(AudioFormat),
    Video(VideoFormat),
}

#[derive(Clone, Copy)]
pub struct ImageFormat {
    pub display_name: &'static str,
    pub extension: &'static str,
}

#[derive(Clone, Copy)]
pub struct AudioFormat {
    pub display_name: &'static str,
    pub extension: &'static str,
    pub codecs: &'static [Codec],
}

#[derive(Clone, Copy)]
pub struct VideoFormat {
    pub display_name: &'static str,
    pub extension: &'static str,
    pub video_codecs: &'static [Codec],
    pub audio_codecs: &'static [Codec],
}


impl MediaFormat {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Image(format) => format.display_name,
            Self::Audio(format) => format.display_name,
            Self::Video(format) => format.display_name,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Self::Image(format) => format.extension,
            Self::Audio(format) => format.extension,
            Self::Video(format) => format.extension,
        }
    }
}