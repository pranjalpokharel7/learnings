mod utils;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_cell(idx: usize, set: bool);
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    // we represent the universe as a flat array so we need to handle indexing properly
    #[inline(always)]
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                // modulo operator handles the "wrapping" around of cell neighbors when it exceeds the boundaris
                let nrow = (row + delta_row) % self.height;
                let ncol = (column + delta_col) % self.width;
                let idx = self.get_index(nrow, ncol);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        // we need to make a copy beforehand because we can't mutate
        // in-place and still apply the rules
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];

                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (true, x) if x < 2 || x > 3 => false,
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };

                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }

    pub fn cells(&self) -> *const usize {
        self.cells.as_slice().as_ptr()
    }
}
