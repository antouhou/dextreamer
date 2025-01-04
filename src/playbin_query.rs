use gst::prelude::*;
use gstreamer as gst;

use crate::{AudioTrack, SubtitleTrack};

use gstreamer::Element;

/// Retrieves the video info from the playbin pipeline.
pub(crate) fn subtitle_tracks(playbin_pipeline: &Element) -> Vec<SubtitleTrack> {
    let subtitles = playbin_pipeline.property::<i32>("n-text");

    let mut subtitle_tracks = Vec::new();
    for i in 0..subtitles {
        let tags = playbin_pipeline.emit_by_name::<Option<gst::TagList>>("get-text-tags", &[&i]);

        if let Some(tags) = tags {
            let mut subtitle_track = SubtitleTrack {
                id: i as usize,
                title: "".to_string(),
                language: "".to_string(),
            };

            if let Some(title) = tags.get::<gst::tags::Title>() {
                subtitle_track.title = title.get().to_string();
            }

            // TODO: this is not always working, maybe use language code instead
            if let Some(language) = tags.get::<gst::tags::LanguageName>() {
                subtitle_track.language = language.get().to_string();
            }

            subtitle_tracks.push(subtitle_track);
        }
    }

    subtitle_tracks
}

/// Retrieves audio tracks from the video stream.
pub(crate) fn audio_tracks(playbin_pipeline: &Element) -> Vec<AudioTrack> {
    let audio = playbin_pipeline.property::<i32>("n-audio");

    // Get audio tracks
    let mut audio_tracks = Vec::new();
    for i in 0..audio {
        let tags = playbin_pipeline.emit_by_name::<Option<gst::TagList>>("get-audio-tags", &[&i]);

        if let Some(tags) = tags {
            let mut audio_track = AudioTrack {
                id: i as usize,
                title: "".to_string(),
                language: "".to_string(),
            };

            if let Some(title) = tags.get::<gst::tags::Title>() {
                audio_track.title = title.get().to_string();
            }

            // TODO: this is not always working, maybe use language code instead
            //  language code also not always working, figure out how to handle it
            if let Some(language) = tags.get::<gst::tags::LanguageName>() {
                audio_track.language = language.get().to_string();
            }

            audio_tracks.push(audio_track);
        }
    }

    audio_tracks
}

/// Retrieves the duration of the video stream.
pub(crate) fn video_duration(playbin_pipeline: &Element) -> f64 {
    // Query the duration
    let mut duration_query = gst::query::Duration::new(gst::Format::Time);

    if playbin_pipeline.query(&mut duration_query) {
        let duration = match duration_query.result() {
            gstreamer::GenericFormattedValue::Time(Some(duration)) => duration.nseconds(),
            _ => 0,
        };
        // By default, the duration is in nanoseconds
        duration as f64 / 1_000_000_000.0
    } else {
        0.0
    }
}
