//! # dextreamer: A Sleek Gstreamer Wrapper
//!
//! `dextreamer` is a sleek and simple wrapper around gstreamer that allows you to handle video streams easily in Rust.
//! It was originally developed as part of the Deko UI framework, but it's completely independent and can be used on its own.
//!
//! Note that while `dextreamer` provides a more user-friendly interface than the base gstreamer library, it might not offer the full functionality of gstreamer.
//!
//! ## Installation
//! Before you can use `dextreamer`, you need to have gstreamer installed on your system.
//! Follow the gstreamer installation instructions in the [gstreamer bindings README](https://crates.io/crates/gstreamer) for guidance.
//!
//! ## Usage
//! Add `dextreamer` to your `Cargo.toml` dependencies and run `cargo build`.
//! Here is an example of how to open a video:
//!
//! ```no_run
//! use std::sync::mpsc;
//! use dextreamer;
//!
//! // open a video
//! let (sender, receiver) = dextreamer::open_video("file:///home/user/my_video.mkv");
//!
//! // Send a play action to the video thread
//! sender.send(dextreamer::VideoStreamAction::SetPlay).unwrap();
//!
//! // Receive events from the video thread
//! match receiver.recv().unwrap() {
//!     dextreamer::VideoStreamEvent::VideoLoaded(info) => println!("Video loaded: {:?}", info),
//!     dextreamer::VideoStreamEvent::NewFrame(frame) => println!("New frame: {:?}", frame),
//!     _ => (),
//! }
//! ```
//!
//! Make sure to replace `"my_video.mkv"` with the actual path to your video file.
//!
//! See the [dextreamer documentation](https://docs.rs/dextreamer) for more detailed usage examples.
//!
//! ## License
//! This library is distributed under the terms of the MIT license.
//! See [LICENSE](LICENSE) for details.

mod event_debounce;
mod playbin_query;
mod streamer;
mod video_info;
mod video_sink;

pub use streamer::*;
pub use video_info::*;
