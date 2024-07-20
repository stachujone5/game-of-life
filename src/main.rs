use rand::random;

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

    fn print(&self) {
        for row in 0..self.size {
            let row_slice = &self.cells[row * self.size..row * self.size + self.size];

            let row_slice: Vec<&str> = row_slice
                .iter()
                .map(|v| if v.to_owned() { " # " } else { " . " })
                .collect();

            println!("{}", row_slice.join(""))
        }
    }
}

fn main() {
    let board = Board::new(30);

    board.print()
}
