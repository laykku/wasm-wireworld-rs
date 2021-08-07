use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    ElectronHead = 1,
    ElectronTail = 2,
    Conductor = 3,
}

#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: u32, height: u32) -> World {
        let cells: Vec<Cell> = (0..width * height).map(|_i| Cell::Empty).collect();
        World {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        match self.cells[idx] {
            Cell::Empty => {
                self.cells[idx] = Cell::Conductor;
            },
            Cell::Conductor => {
                self.cells[idx] = Cell::Empty;
            },
            _ => {}
        }
    }

    pub fn set_electronhead(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        if self.cells[idx] == Cell::Conductor {
            self.cells[idx] = Cell::ElectronHead;
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let electron_heads = self.electron_heads_count(row, col);

                let next_cell = match (cell, electron_heads) {
                    (Cell::Empty, _) => Cell::Empty,
                    (Cell::ElectronHead, _) => Cell::ElectronTail,
                    (Cell::ElectronTail, _) => Cell::Conductor,
                    (Cell::Conductor, electrons) => {
                        if electrons > 0 && electrons <= 2 {
                            Cell::ElectronHead
                        } else {
                            Cell::Conductor
                        }
                    }
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
}

impl World {
    fn electron_heads_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                if let Cell::ElectronHead = self.cells[idx] {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32, Cell)]) {
        for (row, col, cell) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = cell;
        }
    }
}
