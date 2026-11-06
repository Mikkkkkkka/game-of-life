use core::fmt;

#[derive(Debug, Clone)]
pub enum Cell {
    Dead,
    Alive,
}

#[derive(Debug)]
pub struct GameFrame {
    cells: Vec<Vec<Cell>>,
}

impl GameFrame {
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![Cell::Dead; height]; width],
        }
    }

    pub fn next(&self) -> GameFrame {
        let width = self.get_width();
        let height = self.get_height();
        let mut next_frame = GameFrame::empty(width, height);
        for i in 0..width {
            for j in 0..height {
                let new_state = self.observe_cell(i, j);
                next_frame.set_cell(new_state, i, j);
            }
        }
        next_frame
    }

    pub fn set_cell(&mut self, cell: Cell, x: usize, y: usize) {
        self.cells[x][y] = cell;
    }

    fn get_width(&self) -> usize {
        self.cells.len()
    }

    fn get_height(&self) -> usize {
        self.cells.first().map_or(0, Vec::len)
    }

    fn observe_cell(&self, x: usize, y: usize) -> Cell {
        let current = &self.cells[x][y];
        let mut count = 0;
        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }
                let maybe_row = x
                    .checked_add(i)
                    .and_then(|coordinate| coordinate.checked_sub(1))
                    .and_then(|coordinate| self.cells.get(coordinate));
                match maybe_row {
                    None => continue,
                    Some(row) => {
                        let maybe_cell = y
                            .checked_add(j)
                            .and_then(|coordinate| coordinate.checked_sub(1))
                            .and_then(|coordinate| row.get(coordinate));
                        match maybe_cell {
                            None => continue,
                            Some(cell) => match cell {
                                Cell::Dead => continue,
                                Cell::Alive => count += 1,
                            },
                        }
                    }
                }
            }
        }
        match (current, count) {
            (Cell::Alive, ..2) => Cell::Dead,
            (Cell::Alive, 2..4) => Cell::Alive,
            (Cell::Alive, 4..) => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            _ => Cell::Dead,
        }
    }
}

impl fmt::Display for GameFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height = self.cells.first().map_or(0, Vec::len);

        // `field` is indexed as `field[x][y]`. Rendering y from low to high
        // makes `field[0][height - 1]` the bottom-left character.
        for (display_row, y) in (0..height).enumerate() {
            for column in &self.cells {
                let cell = &column[y];
                let symbol = match cell {
                    Cell::Alive => 'O',
                    Cell::Dead => '.',
                };
                write!(f, "{symbol}")?;
            }

            if display_row + 1 < height {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
