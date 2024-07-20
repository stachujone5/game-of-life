use rand::random;
use std::{thread, time};

struct Board {
    size: usize,
    cells: Vec<bool>,
}

impl Board {
    fn new(size: usize) -> Self {
        let mut randomized_cells: Vec<bool> = Vec::new();

        for _ in 0..size * size {
            randomized_cells.push(random())
        }

        Self {
            size,
            cells: randomized_cells,
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&bool> {
        let row = self.get_row(y);

        row.get(x)
    }

    fn get_num_of_alive_neighbours(&self, x: usize, y: usize) -> usize {
        let mut neighbours: Vec<&bool> = Vec::new();

        // left top diagonal
        if x != 0 && y != 0 {
            if let Some(state) = self.get_cell(x - 1, y - 1) {
                if state.to_owned() {
                    neighbours.push(state)
                }
            }
        }

        // top
        if y != 0 {
            if let Some(state) = self.get_cell(x, y - 1) {
                if state.to_owned() {
                    neighbours.push(state)
                }
            }
        }

        // right top diagonal
        if y != 0 && x != self.size - 1 {
            if let Some(state) = self.get_cell(x + 1, y - 1) {
                if state.to_owned() {
                    neighbours.push(state)
                }
            }
        }

        // left
        if x != 0 {
            if let Some(state) = self.get_cell(x - 1, y) {
                if state.to_owned() {
                    neighbours.push(state)
                }
            }
        }

        // right
        if x != self.size - 1 {
            if let Some(state) = self.get_cell(x + 1, y) {
                if state.to_owned() {
                    neighbours.push(state)
                }
            }
        }

        // left bottom diagonal
        if y != self.size - 1 && x != 0 {
            if let Some(state) = self.get_cell(x - 1, y + 1) {
                if state.to_owned() {
                    neighbours.push(state)
                }
            }
        }

        // bottom
        if y != self.size - 1 {
            if let Some(state) = self.get_cell(x, y + 1) {
                if state.to_owned() {
                    neighbours.push(state)
                }
            }
        }

        // right bottom diagonal
        if y != self.size - 1 && x != self.size - 1 {
            if let Some(state) = self.get_cell(x + 1, y + 1) {
                if state.to_owned() {
                    neighbours.push(state)
                }
            }
        }

        neighbours.len()
    }

    fn create_next_generation(&mut self) {
        let mut new_cells = self.cells.clone();

        for x in 0..self.size {
            for y in 0..self.size {
                let n = self.get_num_of_alive_neighbours(x, y);
                let index = y * self.size + x;

                new_cells[index] = match self.cells[index] {
                    true => n == 2 || n == 3,
                    false => n == 3,
                };
            }
        }

        self.cells = new_cells;
    }

    fn get_row(&self, index: usize) -> &[bool] {
        &self.cells[index * self.size..index * self.size + self.size]
    }

    fn print(&self) {
        for row_index in 0..self.size {
            let row = self.get_row(row_index);

            let row: Vec<&str> = row
                .iter()
                .map(|v| if v.to_owned() { " # " } else { " . " })
                .collect();

            println!("{}", row.join(""))
        }

        println!();
    }
}

fn main() {
    let mut board = Board::new(45);

    loop {
        board.print();
        board.create_next_generation();

        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
    }
}
