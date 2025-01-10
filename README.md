# dextreamer

dextreamer is a sleek and simple wrapper around gstreamer for handling video streams in Rust. While it provides a more straightforward interface, please note that it may not offer the complete functionality of gstreamer.

Developed as a part of the larger project, the Daiko UI framework, dextreamer stands alone and does not depend on Deko. It is a separate library that can be utilized independently in any Rust application that requires video streaming capabilities.

## Installation

### Prerequisite: Gstreamer

Before using dextreamer, you must install gstreamer on your system. For detailed installation instructions, please refer to the original gstreamer bindings [README](https://crates.io/crates/gstreamer).

### Installing dextreamer

To add dextreamer to your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
dextreamer = "0.1.0"
```

Then run `cargo build` to build your project.

## Usage

Here's a simple example of how you might use dextreamer in your project:

```rust
// Add the dextreamer crate
use dextreamer;

fn main() {
    // Open a video stream
    let (sender, receiver) = dextreamer::open_video("path_to_your_video_file");

    // Handle video stream events
    while let Ok(event) = receiver.recv() {
        match event {
            dextreamer::VideoStreamEvent::VideoLoaded(video_info) => println!("Video loaded: {:?}", video_info),
            _ => (),
        }
    }
}
```

For more examples and detailed usage instructions, please see the [dextreamer documentation](https://docs.rs/dextreamer).

## Authors

- [Anton Suprunchuk](https://github.com/antouhou) - [Website](https://antouhou.com)

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
