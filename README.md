gridvid
=======

`gridvid` is a Rust wrapper library for rendering MP4 videos from 2D vectors using a minimal interface.

The outer vector translates to the X-axis and the inner vectors translate to the Y-axis.

## Usage

```rust
    // Create a 2D Vec of any element type, bool in this example
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 10]; 10];

    // fn to map grid element type to RGB tuple `(u8, u8, u8)`
    let convert = |&b: &bool| if b { (0, 0, 255) } else { (0, 0, 0) };

    // Initialize video encoder
    let mut video = gridvid::Encoder::new("/tmp/output.mp4", Box::new(convert)).build()?;

    // Update the grid as desired, adding a new frame for each grid state
    for i in 0..grid.len() {
        grid[i][i] = true;
        video.add_frame(&grid)?;
    }

    // Write encoded video to output file
    video.close()?;
```

This sample code renders and exports the following:

<video controls style="display: block; max-width: 360px" src="https://user-images.githubusercontent.com/65624699/224349598-32c3c34c-fde2-4194-a398-fe7cde6b3335.mp4"></video>

## Additional Options

```rust
use gridvid::Gridlines;

let mut video = Encoder::new(filename, Box::new(convert))
    .fps(20)    // Change video frame rate to 20 fps
    .scale(16)  // Scale each grid element to a 16x16 pixel square
    .gridlines(Gridlines::Show((255,255,255)))  // Set gridline color to white
    .gridlines(Gridlines::Hide)                 // Hide gridlines
    .build()?;
```

Defaults:
 - Video frame rate is 4 fps
 - Black `Gridlines` are inserted in between elements
 - Video is scaled to fit in a 720 pixel square

## Documentation

https://docs.rs/gridvid/

## Gallery

`examples/game_of_life.rs`

<video controls style="display: block; max-width: 360px" src="https://user-images.githubusercontent.com/65624699/224367600-e5718cec-0a77-4313-98ad-6043abbc9b94.mp4"></video>

## License

Currently licensed under AGPL-3.0, but looking into options that would allow for a more permissive license.
