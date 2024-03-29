mod common;
use common::*;

use gridvid::{Encoder, Error, Gridlines, Result, Scaling};

#[test]
fn file_overwrite_prevention() -> Result<()> {
    env_logger_init();

    let filename = TempPath::new(&"file_overwrite_check.mp4");
    std::fs::File::create(&filename)?;

    let res = Encoder::new(&filename, Box::new(griditem_to_rgb)).build();

    if let Err(Error::IoError(e)) = res {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            return Ok(());
        }
    }

    Err(Error::IoError(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        "output file possibly overwritten",
    )))
}

#[test]
// Ignored by default because it takes disproportionately long
#[ignore]
fn max_frame_size() -> Result<()> {
    env_logger_init();

    // Equals max size
    let grid = vec![vec![GridItem::Off; 3072]; 3072];
    let filename = TempPath::new(&"at_max_size.mp4");
    let mut video = Encoder::new(&filename, Box::new(griditem_to_rgb))
        .scale(Scaling::Uniform(1))
        .fps(2)
        .gridlines(Gridlines::Hide)
        .build()?;
    video.add_frame(&grid)?;

    // Above max size
    let grid = vec![vec![GridItem::Off; 3072]; 3073];
    let filename = TempPath::new(&"above_max_size.mp4");
    let mut video = Encoder::new(&filename, Box::new(griditem_to_rgb))
        .scale(Scaling::Uniform(1))
        .fps(2)
        .gridlines(Gridlines::Hide)
        .build()?;
    let res = video.add_frame(&grid);

    match res {
        Ok(_) => Err(Error::Openh264Error(openh264::Error::msg(
            "oversized frame passed constraints",
        ))),
        Err(Error::OversizedFrame(_)) => Ok(()),
        Err(e) => Err(e),
    }
}
