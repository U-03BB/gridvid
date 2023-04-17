gridvid
=======

Gridvid is a Rust wrapper library for rendering MP4 videos from 2D vectors using a minimal interface.

The outer vector translates to the X-axis and the inner vectors translate to the Y-axis.

## Basic Usage

```rust
    use gridvid::Encoder;

    // Create a 2D Vec of any element type, bool in this example
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 10]; 10];

    // fn to map grid element reference to RGB tuple `(u8, u8, u8)`
    let convert = |&b: &bool| if b { (0, 0, 255) } else { (0, 0, 0) };

    // Initialize video encoder
    let mut video = Encoder::new("/tmp/output.mp4", Box::new(convert)).build()?;

    // Update the grid as desired, adding a new frame for each grid state
    for i in 0..grid.len() {
        grid[i][i] = true;
        video.add_frame(&grid)?;
    }

    // Write encoded video to output file
    video.close()?;
```

This example renders and exports the following:

[Output](https://user-images.githubusercontent.com/65624699/224349598-32c3c34c-fde2-4194-a398-fe7cde6b3335.mp4)
<video controls style="display: block; max-width: 360px" src="https://user-images.githubusercontent.com/65624699/224349598-32c3c34c-fde2-4194-a398-fe7cde6b3335.mp4"></video>

## Options Summary

```rust
use gridvid::{Encoder, Gridlines, Scaling};

let mut video = Encoder::new(filename, Box::new(convert))
    .fps(20)    // Set video frame rate to 20 fps

    // Video Frame Scaling options
    .scale(Scaling::Uniform(16))        // Upscale by a factor of 16
    .scale(Scaling::MaxSize(720, 480))  // Scale to 720x480, keeping aspect ratio
    .scale(Scaling::Stretch(720, 480))  // Stretch to 720x480, ignoring aspect ratio

    // Gridline options
    .gridlines(Gridlines::Show((255,255,255)))  // Set gridline color to white
    .gridlines(Gridlines::Hide)                 // Hide gridlines
    .build()?;
```

Encoder Defaults:
 - Video frame rate is 4 fps
 - Black gridlines: `Gridlines(0,0,0)`
 - Video is scaled to 720x720: `MaxSize(720, 720)`

## Documentation

https://docs.rs/gridvid/

## Gallery

[Sample output from `examples/game_of_life.rs`](https://user-images.githubusercontent.com/65624699/224367600-e5718cec-0a77-4313-98ad-6043abbc9b94.mp4)
<video controls style="display: block; max-width: 360px" src="https://user-images.githubusercontent.com/65624699/224367600-e5718cec-0a77-4313-98ad-6043abbc9b94.mp4"></video>

## License

Licensed under the MIT license as of version 0.2.0.
