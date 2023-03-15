use std::ffi::CString;

mod minimp4;

pub fn mux<T>(video: &crate::Encoder<T>) {
    let filename = CString::new(video.filepath.to_str().unwrap()).unwrap();

    // SAFETY: Inputs are validated earlier in the encoding process. This primarily wraps minimp4.h.
    unsafe {
        minimp4::mux_mp4(
            filename.as_ptr() as *mut i8,
            video.buffer.as_ptr() as *mut u8,
            video.buffer.len() as isize,
            video.width.unwrap() as i32,
            video.height.unwrap() as i32,
            video.fps as i32,
        );
    }
}
