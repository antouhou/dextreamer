/// `SubtitleTrack` represents a subtitle track in a video.
///
/// # Fields
///
/// * `id: u32`: The ID of the subtitle track.
/// * `language: Option<String>`: The language of the subtitle track.
#[derive(Default, Clone, Debug)]
pub struct SubtitleTrack {
    pub id: usize,
    pub title: String,
    pub language: String,
}

/// `AudioTrack` represents an audio track in a video.
///
/// # Fields
///
/// * `id: u32`: The ID of the audio track.
/// * `language: Option<String>`: The language of the audio track.
#[derive(Default, Clone, Debug)]
pub struct AudioTrack {
    pub id: usize,
    pub title: String,
    pub language: String,
}

/// `PlayingState` represents the current playing state of a video.
///
/// # Variants
///
/// * `Playing`: The video is currently playing.
/// * `Paused`: The video is currently paused.
/// * `Stopped`: The video is currently stopped. Currently unused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayingState {
    Playing,
    Paused,
    Stopped,
}

impl Default for PlayingState {
    fn default() -> Self {
        Self::Paused
    }
}

/// `VideoInfo` contains detailed information about a video.
///
/// # Fields
///
/// * `title: String`: The title of the video.
/// * `current_subtitle_track: Option<usize>`: The currently selected subtitle track, represented by its index in the `subtitle_tracks` vector. If `None`, no subtitle track is currently selected.
/// * `current_audio_track: usize`: The currently selected audio track, represented by its index in the `audio_tracks` vector.
/// * `volume: f32`: The current volume level of the video playback. The volume level is a value between 0.0 and 1.0, with 0.0 being silent and 1.0 being the maximum volume.
/// * `subtitle_tracks: Vec<SubtitleTrack>`: A vector of the available subtitle tracks.
/// * `audio_tracks: Vec<AudioTrack>`: A vector of the available audio tracks.
/// * `playing_state: PlayingState`: The current playback state of the video (e.g., playing, paused).
/// * `duration: f64`: The total duration of the video in seconds.
/// * `current_position: f64`: The current playback position in the video in seconds. This value should be between 0 and `duration`.
///
/// # Example
///
/// ```
/// use dextreamer::{PlayingState, VideoInfo};
///
/// let info = VideoInfo {
///     title: "Example Video".into(),
///     current_subtitle_track: Some(0),
///     current_audio_track: Some(0),
///     volume: 1.0,
///     subtitle_tracks: vec![],
///     audio_tracks: vec![],
///     playing_state: PlayingState::Paused,
///     duration: 600.0,
///     current_position: 0.0,
/// };
/// ```
#[derive(Clone, Debug)]
pub struct VideoInfo {
    pub title: String,
    pub current_subtitle_track: Option<usize>,
    pub current_audio_track: Option<usize>,
    pub volume: f32,
    pub subtitle_tracks: Vec<SubtitleTrack>,
    pub audio_tracks: Vec<AudioTrack>,
    pub playing_state: PlayingState,
    pub duration: f64,
    pub current_position: f64,
}

impl VideoInfo {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            current_subtitle_track: Some(0),
            subtitle_tracks: vec![],
            audio_tracks: vec![],
            volume: 1.0,
            current_audio_track: Some(0),
            playing_state: PlayingState::Stopped,
            duration: 0.0,
            current_position: 0.0,
        }
    }

    pub fn subtitle_tracks(&self) -> &[SubtitleTrack] {
        &self.subtitle_tracks
    }

    pub fn audio_tracks(&self) -> &[AudioTrack] {
        &self.audio_tracks
    }
}

impl Default for VideoInfo {
    fn default() -> Self {
        Self::new()
    }
}
