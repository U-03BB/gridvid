use std::path::{Path, PathBuf};

pub fn env_logger_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[allow(unused)]
#[derive(Clone, Copy, PartialEq)]
pub enum GridItem {
    On,
    Off,
}

pub fn griditem_to_rgb(item: &GridItem) -> gridvid::Rgb {
    match item {
        GridItem::On => (128, 0, 255),
        GridItem::Off => (0, 0, 0),
    }
}

#[derive(Debug)]
pub struct TempPath(pub PathBuf);
impl TempPath {
    pub fn new<P: AsRef<Path>>(filename: &P) -> Self {
        let path = std::env::temp_dir().join(filename);
        Self(path)
    }
}

impl Drop for TempPath {
    fn drop(&mut self) {
        std::fs::remove_file(&self.0).ok();
    }
}

impl std::convert::AsRef<Path> for TempPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}
