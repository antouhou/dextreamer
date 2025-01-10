pub trait FrameHandler: Send {
    fn handle_new_frame(&self, frame_data: &[u8], frame_size: (u32, u32));
}