/// OpenH264 limits a frame's width×height to this value
pub const OPENH264_MAX_SIZE: usize = 9437184;

#[derive(Debug, thiserror::Error)]
/// The error type for Gridvid operations.
///
/// Several variants represent common encoding errors that can be caught early in the pipeline.
pub enum Error {
    /// Error resulting from an I/O operation.
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    /// Error resulting from an OpenH264 operation.
    #[error("{0}")]
    Openh264Error(#[from] openh264::Error),
    /// Frame width×height > [OPENH264_MAX_SIZE].
    ///
    /// Wraps: `(frame_width, frame_height)`
    #[error(
        "openh264 limits frame width * height <= {}. (w, h)={0:?}",
        OPENH264_MAX_SIZE
    )]
    OversizedFrame((usize, usize)),
    /// Frame dimensions are invalid.
    /// Open264 frame requires frame width and height to both be above 0 and a multiple of 2.
    ///
    /// Wraps: `(frame_width, frame_height)`
    #[error("openh264 requires frame width and height to both be above 0 and a multiple of 2. (w, h)={0:?}")]
    InvalidFrameDimensions((usize, usize)),
    /// Video has zero frames.
    #[error("video has zero frames")]
    NoFrames,
    /// Frame dimensions differ from video.
    ///
    /// Wraps: `(frame_number, (frame_width, frame_height), (video_width, video_height))`
    #[error("the size of frame {0} does not match previous frame(s). frame={1:?}, video={1:?} ")]
    FrameSizeMismatch(usize, (usize, usize), (usize, usize)),
    /// Grid columns must all be of the same height.
    ///
    /// Wraps: non-compliant frame number
    #[error("at least one column in frame {0} differs from other columns")]
    InconsistentGridHeight(usize),
}
