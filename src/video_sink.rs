use crate::streamer::InternalMessage;
use crate::{FrameData, VideoStreamEvent};
use gst::element_error;
use gstreamer as gst;
use gstreamer_app as gst_app;
use gstreamer_app::AppSink;
use gstreamer_video as gst_video;
use std::sync::mpsc::Sender;
use crate::frame_handler::FrameHandler;

pub(crate) fn memory_video_sink(
    internal_sender: Sender<InternalMessage>,
    external_sender: Sender<VideoStreamEvent>,
    frame_data_handler: impl FrameHandler + Send + 'static,
) -> AppSink {
    let video_format = gst_video::VideoCapsBuilder::new()
        .format(gst_video::VideoFormat::Rgba)
        .build();

    let appsink = gst_app::AppSink::builder().caps(&video_format).build();

    let sink_callback = gst_app::AppSinkCallbacks::builder()
        // Add a handler to the "new-sample" signal.
        .new_sample(move |appsink| {
            // Pull the sample in question out of the appsink's buffer.
            let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
            let buffer = sample.buffer().ok_or_else(|| {
                element_error!(
                    appsink,
                    gst::ResourceError::Failed,
                    ("Failed to get buffer from appsink")
                );

                gst::FlowError::Error
            })?;

            // At this point, buffer is only a reference to an existing memory region somewhere.
            // When we want to access its content, we have to map it while requesting the required
            // mode of access (read, read/write).
            // This type of abstraction is necessary, because the buffer in question might not be
            // on the machine's main memory itself, but rather in the GPU's memory.
            // So mapping the buffer makes the underlying memory region accessible to us.
            // See: https://gstreamer.freedesktop.org/documentation/plugin-development/advanced/allocation.html
            let map = buffer.map_readable().map_err(|_| {
                element_error!(
                    appsink,
                    gst::ResourceError::Failed,
                    ("Failed to map buffer readable")
                );

                gst::FlowError::Error
            })?;

            let caps = sample.caps().expect("Expect caps to exist");
            let info = gst_video::VideoInfo::from_caps(caps).expect("Failed to parse caps");

            internal_sender
                .send(InternalMessage::RequestPositionUpdate)
                .unwrap();

            frame_data_handler.handle_new_frame(map.as_slice(), (info.width(), info.height()));

            external_sender
                .send(VideoStreamEvent::NewFrame)
                .unwrap();

            Ok(gst::FlowSuccess::Ok)
        })
        .build();

    appsink.set_callbacks(sink_callback);

    appsink
}
