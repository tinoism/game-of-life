mod utils;
use std::vec::Vec;
use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Alive = 1,
    Dead = 0,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let cells = (0..width*height).map(|index| {
            if index % 2 == 0 || index % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();
        Universe {
            width,
            height, 
            cells
        }
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize{
        (row * self.width + col) as usize
    }

    pub fn get_live_neighbours(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for dx in [self.width - 1, 0, 1].iter().cloned() {
            for dy in [self.height - 1, 0, 1].iter().cloned() {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let neighbour_x = (dx + row) % self.width;
                let neighbour_y = (dy + col) % self.height;
                let neighbour_index = self.get_index(neighbour_x, neighbour_y);
                if self.cells[neighbour_index] == Cell::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    // ref mut since we're modifying self.cells
    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();
        for index in 0..self.width * self.height {
            let row = index / self.height;
            let col = index % self.height;
            let live_neighbours = self.get_live_neighbours(row, col);
            let next_cell = match (live_neighbours, &self.cells[index as usize]) {
                (x, Cell::Alive) if x < 2 => Cell::Dead,
                (x, Cell::Alive) if x > 3 => Cell::Dead,
                (3, Cell::Dead) => Cell::Alive,
                (_, y) => y.clone(),
            };
            next_cells[index as usize] = next_cell;
        }
        self.cells = next_cells;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result<> {
        for index in 0..self.height*self.width {
            let symbol = if self.cells[index as usize] == Cell::Alive {'◼'} else {'◻'};
            write!(f, "{}", symbol)?;
            if (index + 1) % self.width == 0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}