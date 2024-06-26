#[derive(Clone)]
pub struct Grid {
    pub cells: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            cells: vec![vec![false; height]; width],
            width,
            height,
        }
    }

    pub fn update_game_state(&mut self) {
        let mut new_cells = self.cells.clone();

        for (x, row) in self.cells.iter().enumerate() {
            for (y, &cell) in row.iter().enumerate() {
                let live_neighbors = self.live_neighbors(x, y);

                new_cells[x][y] = match (cell, live_neighbors) {
                    // Rule 1
                    (true, x) if x < 2 => false,
                    // Rule 2
                    (true, 2) | (true, 3) => true,
                    // Rule 3
                    (true, x) if x > 3 => false,
                    // Rule 4
                    (false, 3) => true,
                    // All other cases
                    (otherwise, _) => otherwise,
                }
            }
        }
        self.cells = new_cells;
    }

    pub fn live_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in [x.checked_sub(1), Some(x), x.checked_add(1)]
            .iter()
            .flatten()
        {
            for j in [y.checked_sub(1), Some(y), y.checked_add(1)]
                .iter()
                .flatten()
            {
                // make sure we're in bounds
                if *i < self.cells.len() && *j < self.cells[0].len() {
                    // don't count the cell itself
                    if !(*i == x && *j == y) {
                        // count the live cells
                        if self.cells[*i][*j] {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}
