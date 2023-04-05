use crate::Error;
use openh264::encoder::{Encoder as OpenH264Encoder, EncoderConfig};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

mod image;
mod muxer;

const DEFAULT_FPS: u16 = 4;
const DEFAULT_SCALE_MAX_SIZE: u16 = 720;

/// A tuple containing Red, Green and Blue color intensities.
pub type Rgb = (u8, u8, u8);
/// A specialized gridvid Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// A function to map grid element type to [Rgb].
pub type Converter<T> = dyn Fn(&T) -> Rgb;

/// A video encoder wrapper. Converts grid to encoded video frames and writes output to a file.
///
/// Defaults:
/// - Video frame rate is 4 fps
/// - Black [Gridlines] are inserted in between elements
/// - Video is scaled to fit in a 720 pixel square
pub struct Encoder<T> {
    filepath: PathBuf,
    width: Option<usize>,
    height: Option<usize>,
    scale: Option<u16>,
    encoder: Option<OpenH264Encoder>,
    buffer: Vec<u8>,
    fps: u32,
    frame_count: usize,
    gridlines: Gridlines,
    converter: Box<Converter<T>>,
}

/// Options for showing or hiding gridlines.
pub enum Gridlines {
    /// Insert gridlines with the wrapped `(u8, u8, u8)` color in between elements.
    Show(Rgb),
    /// Hide gridlines.
    Hide,
}

/// EncoderBuilder allows for flexible customization of the video [Encoder].
pub struct EncoderBuilder<T> {
    filepath: PathBuf,
    converter: Box<Converter<T>>,
    scale: Option<u16>,
    fps: Option<u16>,
    gridlines: Option<Gridlines>,
}

impl<T> EncoderBuilder<T> {
    /// Scales the video. The size of every element in the grid is multiplied by `scale` so each element appears as an `scale×scale` pixel square.
    ///
    /// If unset, video is scaled to fit in a 720 pixel square
    pub fn scale(mut self, scale: u16) -> Self {
        self.scale = if scale > 0 { Some(scale) } else { None };
        self
    }
    /// Sets video frame rate.
    ///
    /// If unset, defaults to 4 fps.
    pub fn fps(mut self, fps: u16) -> Self {
        self.fps = if fps > 0 { Some(fps) } else { None };
        self
    }
    /// Indicates whether or not to show [Gridlines] and if so pick a color.
    ///
    /// If unset, defaults to `Show(0,0,0)` which shows black gridlines.
    pub fn gridlines(mut self, gridlines: Gridlines) -> Self {
        self.gridlines = Some(gridlines);
        self
    }

    /// Returns a configured video [Encoder].
    pub fn build(self) -> Result<Encoder<T>> {
        if Path::try_exists(&self.filepath)? {
            return Err(Error::IoError(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("output file already exists: {}", &self.filepath.display()),
            )));
        }

        fs::File::create(&self.filepath)?;
        log::debug!("video output file created: {}", &self.filepath.display());

        Ok(Encoder {
            filepath: self.filepath,
            fps: self.fps.unwrap_or(DEFAULT_FPS) as u32,
            scale: self.scale,
            gridlines: self.gridlines.unwrap_or(Gridlines::Show((0, 0, 0))),
            converter: self.converter,
            buffer: Vec::new(),
            frame_count: 0,
            width: None,
            height: None,
            encoder: None,
        })
    }
}

impl<T> Encoder<T> {
    /// Returns a new [EncoderBuilder].
    ///
    /// # Arguments
    ///
    /// - `filepath` - The destination file path. Warns if it does not end with the extension `.mp4`.
    /// - `converter` - A boxed function that maps grid type to a tuple, `&T -> (u8, u8, u8)` containing Red, Green and Blue values.
    ///
    #[allow(clippy::new_ret_no_self)]
    pub fn new<F: AsRef<Path>>(filepath: F, converter: Box<Converter<T>>) -> EncoderBuilder<T> {
        let filepath = filepath.as_ref().to_owned();

        if filepath.extension().unwrap() != "mp4" {
            log::warn!("video filename extension is not `.mp4`");
        }

        EncoderBuilder {
            filepath,
            converter,
            fps: None,
            scale: None,
            gridlines: None,
        }
    }

    /// Adds a grid as a frame to the video. Returns a `Result` with the current frame count or an Error.
    pub fn add_frame(&mut self, grid: &[Vec<T>]) -> Result<usize> {
        let grid_width = grid.len();
        let grid_height = grid.get(0).map_or(0, |x| x.len());

        // Grid shape sanity checks
        if grid_width == 0 || grid_height == 0 {
            return Err(Error::InvalidFrameDimensions(
                self.scale.unwrap_or(0),
                (grid_width, grid_height),
            ));
        }
        if grid.iter().skip(1).any(|y| y.len() != grid_height) {
            return Err(Error::InconsistentGridHeight(self.frame_count));
        }

        let (grid_padding_width, grid_padding_height): (usize, usize) =
            if let Gridlines::Show(_) = &self.gridlines {
                let w = (grid_width - 1) * 2;
                let h = (grid_height - 1) * 2;
                (w, h)
            } else {
                (0, 0)
            };

        if self.encoder.is_none() {
            // then this is the first frame

            if self.scale.is_none() {
                // Limit final image size to DEFAULT_SCALE_MAX_SIZE
                self.scale = if grid_width >= grid_height {
                    Some((DEFAULT_SCALE_MAX_SIZE - grid_padding_width as u16) / grid_width as u16)
                } else {
                    Some((DEFAULT_SCALE_MAX_SIZE - grid_padding_height as u16) / grid_height as u16)
                };
            };
            let scale = self.scale.unwrap();

            let video_width = grid_width * scale as usize + grid_padding_width;
            let video_height = grid_height * scale as usize + grid_padding_height;

            // Validate OpenH264 frame requirements
            if video_width * video_height > crate::error::OPENH264_MAX_SIZE {
                return Err(Error::OversizedFrame((video_width, video_height)));
            };
            if scale % 2 == 1 && (video_width % 2 == 1 || video_height == 1) {
                return Err(Error::InvalidFrameDimensions(
                    scale,
                    (grid_width, grid_height),
                ));
            }

            let config = EncoderConfig::new(video_width as u32, video_height as u32);
            let encoder = OpenH264Encoder::with_config(config)?;
            self.width = Some(video_width);
            self.height = Some(video_height);
            self.encoder = Some(encoder);
        }

        let video_width = self.width.unwrap();
        let video_height = self.height.unwrap();
        let scale = self.scale.unwrap();

        let frame_width = grid_width * scale as usize + grid_padding_width;
        let frame_height = grid_height * scale as usize + grid_padding_height;

        if frame_width != video_width || frame_height != video_height {
            return Err(Error::FrameSizeMismatch(
                self.frame_count,
                (frame_width, frame_height),
                (video_width, video_height),
            ));
        }

        let rgb_stream: Vec<u8> = image::format(grid, scale, &self.converter, &self.gridlines);
        let yuv = openh264::formats::YUVBuffer::with_rgb(video_width, video_height, &rgb_stream);

        // Encode YUV into H.264.
        let encoder = self.encoder.as_mut().unwrap();
        let bitstream = encoder.encode(&yuv)?;
        bitstream.write_vec(&mut self.buffer);

        self.frame_count += 1;
        log::debug!(
            "video frame added to {}. total: {}",
            &self.filepath.display(),
            &self.frame_count
        );

        Ok(self.frame_count)
    }

    /// Writes encoded video to output file. Returns number of bytes written.
    pub fn close(self) -> Result<usize> {
        if *self.frame_count() == 0 {
            return Err(Error::NoFrames);
        };

        muxer::mux(&self);

        log::debug!("video output written: {}", &self.filepath.display());
        Ok(0) // TODO: Get size of file?
    }

    /// Returns the current number of frames
    pub fn frame_count(&self) -> &usize {
        &self.frame_count
    }
}
