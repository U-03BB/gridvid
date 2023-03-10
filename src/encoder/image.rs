use crate::{Gridlines, Rgb};

/// Converts data type, transposes and flattens grid to conform with openh264::formats::rgb2yuv

// Wrapped into a single function to avoid unnecessary 2D Vec allocations
pub(crate) fn format<T, F>(
    grid: &[Vec<T>],
    scale: u16,
    convert: F,
    gridlines: &Gridlines,
) -> Vec<u8>
where
    F: Fn(&T) -> Rgb,
{
    let grid_width = grid.len();
    let grid_height = grid[0].len();

    let (grid_padding_width, grid_padding_height): (usize, usize) =
        if let Gridlines::Show(_) = &gridlines {
            let w = (grid_width - 1) * 2;
            let h = (grid_height - 1) * 2;
            (w, h)
        } else {
            (0, 0)
        };

    let frame_width = grid_width * scale as usize + grid_padding_width;
    let frame_height = grid_height * scale as usize + grid_padding_height;

    let mut output: Vec<u8> = Vec::with_capacity(frame_width * frame_height * 3);
    (0..grid_height)
        .rev() // For OpenH264, (0,0) is upper-left corner
        .for_each(|y| {
            let row_step_iter = (0..grid_width).flat_map(|x| -> Vec<Rgb> {
                let rgb_value: Rgb = convert(&grid[x][y]);
                let cell_width: Vec<Rgb> = vec![rgb_value; scale as usize];
                if let Gridlines::Show(color) = gridlines {
                    if x != grid_width - 1 {
                        let grid_divider = color;
                        // width must remain a multiple of 2
                        return [cell_width, vec![*grid_divider, *grid_divider]].concat();
                    }
                }
                cell_width
            });

            for _ in 0..scale {
                for rgb in row_step_iter.clone() {
                    output.push(rgb.0);
                    output.push(rgb.1);
                    output.push(rgb.2);
                }
            }
            if let Gridlines::Show(color) = gridlines {
                if y != 0 {
                    // height must remain a multiple of 2
                    for _ in 0..(frame_width * 2) {
                        output.push(color.0);
                        output.push(color.1);
                        output.push(color.2);
                    }
                }
            }
        });

    output
}
