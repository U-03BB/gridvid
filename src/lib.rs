#![warn(missing_docs)]
//! `gridvid` is a Rust wrapper library for rendering MP4 videos from 2D vectors using a minimal interface.
//!
//! The outer vector translates to the X-axis and the inner vectors translate to the Y-axis.
//!
//! ## Usage
//!
//! ```
//! # fn main() -> gridvid::Result<()> {
//!     // Create a 2D Vec of any element type, bool in this example
//!     let mut grid: Vec<Vec<bool>> = vec![vec![false; 10]; 10];
//!     let filename = std::env::temp_dir().join("gridvid_example.mp4");
//!
//!     // fn to map grid element type to RGB tuple `(u8, u8, u8)`
//!     let convert = |&b: &bool| if b { (0, 0, 255) } else { (0, 0, 0) };
//!
//!     // Initialize video encoder
//!     let mut video = gridvid::Encoder::new(&filename, Box::new(convert)).build()?;
//!
//!     // Update the grid as desired, adding a new frame for each grid state
//!     for i in 0..grid.len() {
//!         grid[i][i] = true;
//!         video.add_frame(&grid)?;
//!     }
//!
//!     // Write encoded video to output file
//!     video.close()?;
//!     println!("Output written to: {}", &filename.display());
//! #
//! #    // Remove file so doctest can be repeated
//! #    std::fs::remove_file(&filename)?;
//! #    Ok(())
//! # }
//! ```
//! This sample code renders and exports the following:
//! 
//! <video controls style="display: block; max-width: 360px" src="https://user-images.githubusercontent.com/65624699/224349598-32c3c34c-fde2-4194-a398-fe7cde6b3335.mp4"></video>
//!
//! ## Additional Options
//! ```
//! # fn main() -> gridvid::Result<()> {
//! #
//!     use gridvid::{Encoder, Gridlines};
//!
//! #    let convert = |&b: &bool| if b { (0, 0, 255) } else { (0, 0, 0) };
//! #    let filename = std::env::temp_dir().join("gridvid_demo.mp4");
//! #    
//!     let mut video = Encoder::new(&filename, Box::new(convert))
//!         .fps(20)    // Change video frame rate to 20 fps
//!         .scale(16)  // Upscale each grid element to a 16x16 square
//!         .gridlines(Gridlines::Show((255,255,255)))  // Set gridline color to white
//!         .gridlines(Gridlines::Hide)                 // Hide gridlines
//!         .build()?;
//! #
//! #    // Remove file so doctest can be repeated
//! #    std::fs::remove_file(&filename)?;
//! #
//! #    Ok(())
//! # }
//! ```
//!
//! # Defaults
//!
//! - Video frame rate is 4 fps
//! - Black [Gridlines] are inserted in between elements
//! - Video is scaled to fit in a 720 pixel square
//!

mod encoder;
mod error;

#[doc(inline)]
pub use encoder::{Converter, Encoder, EncoderBuilder, Gridlines, Result, Rgb};
#[doc(inline)]
pub use error::{Error, OPENH264_MAX_SIZE};
