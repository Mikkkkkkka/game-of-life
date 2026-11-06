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

    pub fn next(&self) -> Self {
        Self {
            cells: self
                .cells
                .iter()
                .enumerate()
                .map(|(x, column)| {
                    column
                        .iter()
                        .enumerate()
                        .map(|(y, _)| {
                            self.observe_cell(x as i32, y as i32)
                                .expect("coordinates came from self")
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn get_cell_at(&self, x: i32, y: i32) -> Option<&Cell> {
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        self.cells.get(x)?.get(y)
    }

    pub fn set_cell_at(&mut self, x: i32, y: i32, cell: Cell) -> Option<()> {
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        if x >= self.get_width() || y >= self.get_height() {
            None
        } else {
            self.cells[x][y] = cell;
            Some(())
        }
    }

    fn get_width(&self) -> usize {
        self.cells.len()
    }

    fn get_height(&self) -> usize {
        self.cells.first().map_or(0, Vec::len)
    }

    fn observe_cell(&self, x: i32, y: i32) -> Option<Cell> {
        use Cell::{Alive, Dead};
        let current = self.get_cell_at(x, y)?;
        let count = (0..3)
            .flat_map(|i| {
                (0..3)
                    .map(move |j| {
                        self.get_cell_at(x - 1 + i, y - 1 + j)
                            .filter(|_| i != 1 || j != 1)
                    })
                    .filter(|cell| match cell {
                        Some(Alive) => true,
                        _ => false,
                    })
            })
            .count();
        match (current, count) {
            (Alive, 2..=3) | (Dead, 3) => Some(Alive),
            _ => Some(Dead),
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
