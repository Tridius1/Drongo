//

#[derive(Clone, Copy)]
pub enum Codec {
    Audio(AudioCodec),
    Video(VideoCodec),
}

#[derive(Clone, Copy)]
pub struct AudioCodec {
    pub display_name: &'static str,
    pub encoder: &'static str,
}

#[derive(Clone, Copy)]
pub struct VideoCodec {
    pub display_name: &'static str,
    pub encoder: &'static str,
}


impl Codec {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Audio(codec) => codec.display_name,
            Self::Video(codec) => codec.display_name,
        }
    }

    pub fn encoder(&self) -> &'static str {
        match self {
            Self::Audio(codec) => codec.encoder,
            Self::Video(codec) => codec.encoder,
        }
    }
}

