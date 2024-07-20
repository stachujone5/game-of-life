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

    fn get_cell(&self, x: usize, y: usize) -> bool {
        let row = self.get_row(y);

        row[x]
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
    }
}

fn main() {
    let board = Board::new(30);

    board.print();

    let coord = board.get_cell(0, 0);

    println!("{}", coord)
}
