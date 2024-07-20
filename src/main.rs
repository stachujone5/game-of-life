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

    fn get_num_of_alive_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if let Some(nx) = x.checked_add_signed(dx) {
                    if let Some(ny) = y.checked_add_signed(dy) {
                        if self.get_cell(nx, ny) == Some(true) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
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

    fn get_row(&self, index: usize) -> Option<&[bool]> {
        self.cells
            .get(index * self.size..index * self.size + self.size)
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        let row = self.get_row(y);

        if let Some(row) = row {
            return row.get(x).copied();
        }

        None
    }

    fn print(&self) {
        for row_index in 0..self.size {
            let row = self.get_row(row_index).expect("Invalid row index");

            let row: Vec<&str> = row.iter().map(|v| if *v { " # " } else { " . " }).collect();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_new() {
        let size = 5;
        let board = Board::new(size);
        assert_eq!(board.size, size);
        assert_eq!(board.cells.len(), size * size);
    }

    #[test]
    fn test_get_cell() {
        let size = 3;
        let board = Board {
            size,
            cells: vec![true, false, true, false, true, false, true, false, true],
        };

        assert_eq!(board.get_cell(0, 0), Some(true));
        assert_eq!(board.get_cell(1, 1), Some(true));
        assert_eq!(board.get_cell(2, 2), Some(true));
        assert_eq!(board.get_cell(2, 1), Some(false));
        assert_eq!(board.get_cell(3, 3), None);
    }

    #[test]
    fn test_get_row() {
        let size = 3;
        let board = Board {
            size,
            cells: vec![true, false, true, false, true, false, true, false, true],
        };

        assert_eq!(board.get_row(0), Some(&[true, false, true][..]));
        assert_eq!(board.get_row(1), Some(&[false, true, false][..]));
        assert_eq!(board.get_row(2), Some(&[true, false, true][..]));
        assert_eq!(board.get_row(3), None);
    }

    #[test]
    fn test_get_num_of_alive_neighbours() {
        let size = 3;
        let board = Board {
            size,
            cells: vec![true, false, true, false, true, false, true, false, true],
        };

        assert_eq!(board.get_num_of_alive_neighbours(0, 0), 1);
        assert_eq!(board.get_num_of_alive_neighbours(1, 1), 4);
        assert_eq!(board.get_num_of_alive_neighbours(2, 2), 1);
    }

    #[test]
    fn test_create_next_generation() {
        let size = 3;
        let mut board = Board {
            size,
            cells: vec![false, true, false, false, true, false, false, true, false],
        };

        board.create_next_generation();

        let expected_next_gen = vec![false, false, false, true, true, true, false, false, false];

        assert_eq!(board.cells, expected_next_gen);
    }
}
