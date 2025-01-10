use gst::prelude::*;
use gstreamer as gst;

use crate::video_sink::memory_video_sink;
use crate::{PlayingState, VideoInfo};

use gstreamer::Bus;
use std::sync::mpsc::{self, Receiver, Sender};

use crate::frame_handler::FrameHandler;
use crate::playbin_query::{audio_tracks, subtitle_tracks, video_duration};
use std::thread;

pub(crate) enum InternalMessage {
    VideoStreamAction(VideoStreamAction),
    RequestPositionUpdate,
}

/// `VideoStreamAction` represents the actions that can be sent to the video stream.
///
/// # Variants
///
/// * `SetCurrentSubtitleTrack(Option<u32>)`: Set the current subtitle track by its ID. If `None` is provided, the subtitles will be disabled.
/// * `SetCurrentAudioTrack(Option<u32>)`: Set the current audio track by its ID. If `None` is provided, the audio will be disabled.
/// * `SetVolume(f32)`: Set the volume. The volume should be between 0.0 and 1.0.
/// * `SetPlay`: Start or resume playback.
/// * `SetPause`: Pause playback.
/// * `SeekToSeconds(f64)`: Seek to a specific position in the video, provided in seconds.
#[derive(Debug, Clone, Copy)]
pub enum VideoStreamAction {
    /// Set the current subtitle track by its ID. If `None` is provided, the subtitles will be disabled.
    SetCurrentSubtitleTrack(Option<usize>),
    ///  Set the current audio track by its ID. If `None` is provided, the audio will be disabled.
    SetCurrentAudioTrack(Option<usize>),
    // TODO: volume is an f64 in the gstreamer
    /// Set the volume. The volume should be between 0.0 and 1.0.
    SetVolume(f32),
    /// Start or resume playback.
    SetPlay,
    /// Pause playback.
    SetPause,
    /// Seek to a specific position in the video, provided in seconds.
    SeekToSeconds(f64),
    /// Close the media stream.
    Close,
}

// /// `VideoStreamActionType` represents the type of the action. This is used to debounce the actions.
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub(crate) enum VideoStreamActionType {
//     SetCurrentSubtitleTrack,
//     SetCurrentAudioTrack,
//     SetVolume,
//     SetPlay,
//     SetPause,
//     SeekToSeconds,
//     Close,
// }

impl VideoStreamAction {
    // /// Returns the type of the action. This is used to debounce the actions.
    // pub(crate) fn message_type(&self) -> VideoStreamActionType {
    //     match self {
    //         VideoStreamAction::SetCurrentSubtitleTrack(_) => {
    //             VideoStreamActionType::SetCurrentSubtitleTrack
    //         }
    //         VideoStreamAction::SetCurrentAudioTrack(_) => {
    //             VideoStreamActionType::SetCurrentAudioTrack
    //         }
    //         VideoStreamAction::SetVolume(_) => VideoStreamActionType::SetVolume,
    //         VideoStreamAction::SetPlay => VideoStreamActionType::SetPlay,
    //         VideoStreamAction::SetPause => VideoStreamActionType::SetPause,
    //         VideoStreamAction::SeekToSeconds(_) => VideoStreamActionType::SeekToSeconds,
    //         VideoStreamAction::Close => VideoStreamActionType::Close,
    //     }
    // }
}

/// `VideoStreamEvent` represents the events that can be emitted from the video stream.
///
/// # Variants
///
/// * `VideoLoaded(VideoInfo)`: Emitted when a video is successfully loaded. Contains metadata about the video.
/// * `NewFrame(FrameData)`: Emitted for each new frame. Contains the raw data and size of the frame.
/// * `CurrentAudioTrackChanged(Option<u32>)`: Emitted when the current audio track changes. Contains the new audio track ID.
/// * `CurrentSubtitleTrackChanged(Option<u32>)`: Emitted when the current subtitle track changes. Contains the new subtitle track ID.
/// * `VolumeChanged(f32)`: Emitted when the volume changes. Contains the new volume.
/// * `PlayingStateChanged(PlayingState)`: Emitted when the playing state changes. Contains the new playing state.
/// * `PositionChanged(f64)`: Emitted when the playback position changes. Contains the new position in seconds.
#[derive(Debug, Clone)]
pub enum VideoStreamEvent {
    /// Emitted when a video is successfully loaded. Contains metadata about the video.
    VideoLoaded(VideoInfo),
    /// Emitted for each new frame. To get the actual frame data, use the `FrameHandler` trait.
    NewFrame,
    Error(String),
    /// Emitted when the current audio track changes. Contains the new audio track ID.
    CurrentAudioTrackChanged(usize),
    /// Emitted when the current subtitle track changes. Contains the new subtitle track ID.
    CurrentSubtitleTrackChanged(usize),
    /// Emitted when the volume changes. Contains the new volume.
    VolumeChanged(f32),
    /// Emitted when the playing state changes. Contains the new playing state.
    PlayingStateChanged(PlayingState),
    /// Emitted when the playback position changes. Contains the new position in seconds.
    PositionChanged(f64),
    /// Emitted when the video stream is closed.
    Closed,
}

/// `FrameData` contains the raw data and size of a video frame.
///
/// # Fields
///
/// * `data: Vec<u8>`: The raw data of the frame. Every 4 elements represent one pixel in the RGBA format.
/// * `size: [usize; 2]`: The size of the frame in pixels. The first element is the width and the second is the height.
#[derive(Default, Debug, Clone)]
pub struct FrameData {
    /// TThe raw data of the frame. Every 4 elements represent one pixel in the RGBA format.
    pub data: Vec<u8>,
    /// The size of the frame in pixels. The first element is the width and the second is the height.
    pub size: [usize; 2],
}

fn handle_action(
    video_action: VideoStreamAction,
    playbin_pipeline: &gst::Element,
    sender: &Sender<VideoStreamEvent>,
) -> bool {
    match video_action {
        VideoStreamAction::SetCurrentSubtitleTrack(track_id) => {
            // Setting a subtitle track
            if let Some(subtitle_track_id) = track_id {
                playbin_pipeline.set_property("current-text", subtitle_track_id as i32);
                sender
                    .send(VideoStreamEvent::CurrentSubtitleTrackChanged(
                        subtitle_track_id,
                    ))
                    .unwrap();
            } else {
                todo!("disable subtitles")
            }
        }
        VideoStreamAction::SetCurrentAudioTrack(audio_track_id) => {
            // Setting an audio track
            if let Some(audio_track_id) = audio_track_id {
                playbin_pipeline.set_property("current-audio", audio_track_id as i32);
                sender
                    .send(VideoStreamEvent::CurrentAudioTrackChanged(audio_track_id))
                    .unwrap();
            } else {
                todo!("disable audio")
            }
        }
        VideoStreamAction::SetVolume(volume) => {
            sender
                .send(VideoStreamEvent::VolumeChanged(volume))
                .unwrap();
            playbin_pipeline.set_property("volume", volume as f64);
        }
        VideoStreamAction::SetPlay => {
            playbin_pipeline.set_state(gst::State::Playing).unwrap();
            sender
                .send(VideoStreamEvent::PlayingStateChanged(PlayingState::Playing))
                .unwrap();
        }
        VideoStreamAction::SetPause => {
            playbin_pipeline.set_state(gst::State::Paused).unwrap();
            sender
                .send(VideoStreamEvent::PlayingStateChanged(PlayingState::Paused))
                .unwrap();
        }
        VideoStreamAction::SeekToSeconds(seconds) => {
            let position_ns = (seconds * 1_000_000_000.0) as u64;
            sender
                .send(VideoStreamEvent::PositionChanged(seconds))
                .unwrap();
            playbin_pipeline
                .seek(
                    1.0,
                    gst::SeekFlags::FLUSH | gst::SeekFlags::ACCURATE,
                    gst::SeekType::Set,
                    gst::ClockTime::from_nseconds(position_ns),
                    gst::SeekType::None,
                    gst::ClockTime::NONE,
                )
                .unwrap_or_else(|_| println!("Seek failed"));
        }
        VideoStreamAction::Close => {
            playbin_pipeline.set_state(gst::State::Null).unwrap();
            sender.send(VideoStreamEvent::Closed).unwrap();
            return true;
        }
    }

    false
}

fn handle_message(
    message: InternalMessage,
    playbin_pipeline: &gst::Element,
    sender: &Sender<VideoStreamEvent>,
) -> bool {
    match message {
        InternalMessage::VideoStreamAction(video_action) => {
            handle_action(video_action, playbin_pipeline, sender)
        }
        InternalMessage::RequestPositionUpdate => {
            let mut position_query = gst::query::Position::new(gst::Format::Time);
            if playbin_pipeline.query(&mut position_query) {
                let position_nanoseconds = match position_query.result() {
                    gstreamer::GenericFormattedValue::Time(Some(position)) => position.nseconds(),
                    _ => 0,
                };
                sender
                    .send(VideoStreamEvent::PositionChanged(
                        position_nanoseconds as f64 / 1_000_000_000.0,
                    ))
                    .unwrap();

                false
            } else {
                println!("Position query failed");

                false
            }
        }
    }
}

fn wait_for_video_to_load(playbin_message_bus: &Bus) {
    // Listen for messages on the pipeline's bus until `async-done` is received, which means
    //  that media has been loaded and playback can begin.
    while let Some(msg) = playbin_message_bus.timed_pop(gst::ClockTime::NONE) {
        if let gst::MessageView::AsyncDone(_) = msg.view() {
            // The async state change has completed
            break;
        }
    }
}

/// Opens a video stream and returns a sender and receiver to communicate with the video thread.
/// Sender is used to send actions to the video thread and receiver is used to receive events
/// from the video thread.
///
/// # Example
/// ```rust
/// struct VideoFrameLoader;
///
/// impl dextreamer::FrameHandler for VideoFrameLoader {
///    fn handle_new_frame(&self, frame_data: &[u8], frame_size: (u32, u32)) {
///       println!("New frame: {:?}", frame_size);
///   }
/// }
///
/// let (actions_sender, events_receiver) = dextreamer::open_video("file:///home/user/my_video.mkv", VideoFrameLoader);
/// // Now you can use `actions_sender` to send actions to the video thread and `events_receiver` to receive events from the video thread.
/// ```
pub fn open_video(
    uri: impl Into<String>,
    frame_data_handler: impl FrameHandler + 'static,
) -> (Sender<VideoStreamAction>, Receiver<VideoStreamEvent>) {
    // Sender to send messages to the video thread
    let (actions_sender, actions_receiver) = mpsc::channel();
    // Receiver to receive messages from the video thread
    let (event_sender, event_receiver) = mpsc::channel();

    let uri = uri.into();

    thread::spawn(move || {
        open_video_internal(&uri, actions_receiver, event_sender, frame_data_handler);
    });

    (actions_sender, event_receiver)
}

fn open_video_internal(
    uri: &str,
    receiver: Receiver<VideoStreamAction>,
    sender: Sender<VideoStreamEvent>,
    frame_data_handler: impl FrameHandler + 'static,
) {
    let (internal_sender, internal_receiver) = mpsc::channel::<InternalMessage>();

    gst::init().expect("to initialize gstreamer without errors");

    let memory_video_sink =
        memory_video_sink(internal_sender.clone(), sender.clone(), frame_data_handler);

    // Create a new playbin element, and tell it what uri to play back.
    let playbin_pipeline = gst::ElementFactory::make("playbin")
        .property("uri", uri)
        .build()
        .unwrap();

    playbin_pipeline.set_property("video-sink", memory_video_sink);

    let playbin_message_bus = playbin_pipeline.bus().unwrap();

    playbin_pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    wait_for_video_to_load(&playbin_message_bus);

    let bus_thread_handle = thread::spawn(move || {
        for msg in playbin_message_bus.iter_timed(gst::ClockTime::NONE) {
            use gst::MessageView;

            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                    break;
                }
                MessageView::StateChanged(_state_changed) =>
                // We are only interested in state-changed messages from playbin
                {
                    // println!("state changed");
                    // if state_changed
                    //     .src()
                    //     .map(|s| s == &playbin_pipeline)
                    //     .unwrap_or(false)
                    //     && state_changed.current() == gst::State::Playing
                    // {
                    //     // Generate a dot graph of the pipeline to GST_DEBUG_DUMP_DOT_DIR if defined
                    //     let bin_ref = playbin_pipeline.downcast_ref::<gst::Bin>().unwrap();
                    //     bin_ref.debug_to_dot_file(gst::DebugGraphDetails::all(), "PLAYING");
                    // }
                }

                _ => (),
            }
        }
    });

    let action_receiver_thread_handle = thread::spawn(move || {
        while let Some(message) = receiver.iter().next() {
            let needs_to_be_closed = matches!(&message, VideoStreamAction::Close);

            internal_sender
                .send(InternalMessage::VideoStreamAction(message))
                .unwrap();

            if needs_to_be_closed {
                break;
            }
        }
    });

    let video_state = VideoInfo {
        title: "Test title.mkv".to_string(),
        current_subtitle_track: Some(0),
        current_audio_track: Some(0),
        volume: 1.0,
        subtitle_tracks: subtitle_tracks(&playbin_pipeline),
        audio_tracks: audio_tracks(&playbin_pipeline),
        playing_state: PlayingState::Playing,
        duration: video_duration(&playbin_pipeline),
        current_position: 0.0,
    };

    // TODO: handle error
    sender
        .send(VideoStreamEvent::VideoLoaded(video_state))
        .expect("to notify the video has loaded");

    while let Some(message) = internal_receiver.iter().next() {
        let needs_to_close_stream = handle_message(message, &playbin_pipeline, &sender);

        if needs_to_close_stream {
            break;
        }
    }

    playbin_pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    action_receiver_thread_handle.join().unwrap();
    bus_thread_handle.join().unwrap();

    println!("All video rendering threads closed");
}
