// Audio codec definitions

use crate::formats::{Codec, VideoCodec};

pub const H264: Codec = Codec::Video(VideoCodec {
    display_name: "H.264",
    encoder: "libx264",
});

pub const H265: Codec = Codec::Video(VideoCodec {
    display_name: "H.265",
    encoder: "libx265",
});

pub const VP9: Codec = Codec::Video(VideoCodec {
    display_name: "VP9",
    encoder: "libvpx-vp9",
});
