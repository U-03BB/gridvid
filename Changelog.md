# Changelog

## 0.3.0 (2023-04-21)
- Added: Options for scaling video
    - Previous scaling functionality provided through `Scaling::Uniform`
    - New options: `Scaling::MaxSize` and `Scaling::Stretch`
- Changed: Removed unnecessary return value in `close`
- Fixed: Underflow when grid size < target resolution

## 0.2.0 (2023-03-30)
- Changed: Now licensed under MIT
- Changed: Implemented Rust-minimp4 bindings

## 0.1.1 (2023-03-10)
- Added: Links to videos in `README.md`, as crates.io doesn't render video tags

## 0.1.0 (2023-03-10)
- Initial Release
