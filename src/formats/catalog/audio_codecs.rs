// Audio codec definitions

use crate::formats::{AudioCodec, Codec};

pub const AAC: Codec = Codec::Audio(AudioCodec {
    display_name: "AAC",
    encoder: "aac",
});

pub const MP3: Codec = Codec::Audio(AudioCodec {
    display_name: "MP3",
    encoder: "libmp3lame",
});

pub const FLAC: Codec = Codec::Audio(AudioCodec {
    display_name: "FLAC",
    encoder: "flac",
});
