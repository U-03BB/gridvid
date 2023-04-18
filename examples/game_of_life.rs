use gridvid::Encoder;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

const STEPS: usize = 50;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::temp_dir().join("game_of_life.mp4");

    let mut grid = Grid::new();

    // Build encoder
    let mut video = Encoder::new(&filename, Box::new(convert)).fps(10).build()?;

    // Add initial state as a frame to the video
    video.add_frame(&grid.content)?;

    for _ in 1..STEPS {
        grid.update();
        video.add_frame(&grid.content)?;
    }

    video.close()?;
    println!("Output written to: {}", &filename.display());

    Ok(())
}

fn convert(cell: &Cell) -> gridvid::Rgb {
    match cell {
        Cell::Alive => (100, 0, 200), // Violet
        Cell::Dead => (0, 0, 0),      // Black
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

// Conway's Game of Life code adapted from the excellent Rust WASM book: https://rustwasm.github.io/docs/book/
struct Grid {
    content: Vec<Vec<Cell>>,
}

impl Grid {
    fn new() -> Self {
        let content: Vec<Vec<Cell>> = (0..u16::BITS)
            .map(|_| {
                // Poor man's PRNG. Avoids adding an extra dependency just for the example
                let partial_time_ns = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .subsec_nanos();

                let mut hasher = DefaultHasher::new();
                partial_time_ns.hash(&mut hasher);
                let hash = hasher.finish() as u16;

                let bit_array_iter = (0..u16::BITS).map(|i| hash >> i & 1 == 1);
                bit_array_iter
                    .map(|b: bool| if b { Cell::Alive } else { Cell::Dead })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();
        Self { content }
    }
    fn update(&mut self) {
        let grid = &self.content;
        let mut next = grid.clone();
        let width = grid.len();
        let height = grid[0].len();

        for x in 0..width {
            for y in 0..height {
                let cell = grid[x][y];
                let live_neighbors = self.live_neighbor_count(x, y);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[x][y] = next_cell;
            }
        }
        self.content = next;
    }
    fn live_neighbor_count(&self, x: usize, y: usize) -> u8 {
        let grid = &self.content;
        let width = grid.len();
        let height = grid[0].len();

        let mut count = 0;
        for delta_row in [width - 1, 0, 1].iter().cloned() {
            for delta_col in [height - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_x = (x + delta_row) % width;
                let neighbor_y = (y + delta_col) % height;
                count += grid[neighbor_x][neighbor_y] as u8;
            }
        }
        count
    }
}
