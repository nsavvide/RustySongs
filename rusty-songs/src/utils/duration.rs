use std::path::PathBuf;

use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;

pub fn get_mp3_duration(path: &PathBuf) -> Option<f64> {
    let file = std::fs::File::open(path).ok()?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // Use Symphonia to probe the file format and extract duration
    let mut probed = get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .ok()?;
    let format = &mut probed.format; // Mutable borrow for later

    // Extract track and codec information before the loop
    let track = format.default_track()?; // Immutable borrow here
    let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &Default::default())
        .ok()?; // Handle codec creation result correctly

    let mut total_duration = 0.0;

    // Use the mutable borrow of format for reading packets
    while let Ok(packet) = format.next_packet() {
        if let Ok(decoded) = decoder.decode(&packet) {
            total_duration += decoded.frames() as f64 / sample_rate as f64;
        }
    }

    Some(total_duration)
}
