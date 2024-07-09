use inline_colorization::*;
const GRID_SIZE: usize = 20;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Standard(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Standard(msg) => write!(f, "Grid Error: {msg}"),
        }
    }
}
impl std::error::Error for Error {}

#[derive(Default)]
pub struct Grid {
    cells: [[Cell; GRID_SIZE]; GRID_SIZE],
    pub running: bool,
}

#[derive(Clone, Copy, PartialEq, Default)]
enum Cell {
    #[default]
    Dead,
    Alive,
}

impl Grid {
    pub fn new() -> Self {
        return Grid {
            cells: [[Cell::Dead; GRID_SIZE]; GRID_SIZE],
            running: true,
        };
    }

    pub fn display(&self) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if self.cells[y][x] == Cell::Alive {
                    print!("{bg_white}{color_white}■ {style_reset}")
                } else {
                    print!("{bg_black}{color_black}■ {style_reset}")
                }
            }
            println!()
        }
    }

    pub fn place_glider(&mut self, x: usize, y: usize) -> Result<()> {
        if x >= GRID_SIZE - 3 || y >= GRID_SIZE - 3 {
            return Err(Error::Standard(format!(
                "You asked to place a glider outside of the possible locations for a glider.
You gave position ({x},{y}), which is not possible to put a glider there as the glider needs
3 spaces vertically and horizontally (GRID_SIZE = {GRID_SIZE}). Change the location",
            )));
        }
        for i in y..y + 3 {
            for j in x..x + 3 {
                self.cells[j][i] = Cell::Dead;
            }
        }

        self.cells[y + 1][x] = Cell::Alive;
        self.cells[y + 2][x + 1] = Cell::Alive;
        self.cells[y + 0][x + 2] = Cell::Alive;
        self.cells[y + 1][x + 2] = Cell::Alive;
        self.cells[y + 2][x + 2] = Cell::Alive;
        Ok(())
    }

    pub fn play_next(&mut self) -> Result<()> {
        let mut new_cells = [[Cell::Dead; GRID_SIZE]; GRID_SIZE];
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let neighbour_alive: u8 = self.get_neighbour_alive_count(y, x);
                match (self.cells[y][x], neighbour_alive) {
                    (Cell::Alive, num) if num < 2 => new_cells[y][x] = Cell::Dead,

                    (Cell::Alive, num) if num <= 3 => new_cells[y][x] = Cell::Alive,
                    (Cell::Alive, _) => new_cells[y][x] = Cell::Dead,
                    (Cell::Dead, num) if num == 3 => new_cells[y][x] = Cell::Alive,
                    _ => (),
                }
            }
        }
        let are_grids_same = Grid::are_grids_same(
            &self,
            &Grid {
                cells: new_cells,
                ..Default::default()
            },
        )?;

        if are_grids_same {
            self.running = false;
        }

        self.cells = new_cells;
        Ok(())
    }

    fn get_neighbour_alive_count(&self, y: usize, x: usize) -> u8 {
        let mut neighbour_alive: u8 = 0;
        for i in -1_i8..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let new_y: i8 = (y as i8) + i;
                let new_x: i8 = (x as i8) + j;

                if new_y < 0 || new_y >= GRID_SIZE as i8 {
                    continue;
                }

                if new_x < 0 || new_x >= GRID_SIZE as i8 {
                    continue;
                }

                let y_index: usize = new_y as usize;
                let x_index: usize = new_x as usize;
                if self.cells[y_index][x_index] == Cell::Alive {
                    neighbour_alive += 1;
                }
            }
        }
        return neighbour_alive;
    }

    pub fn are_grids_same(g1: &Grid, g2: &Grid) -> Result<bool> {
        if g1.cells.len() != g2.cells.len() {
            return Err(Error::Standard("The grids' sizes don't match".to_owned()));
        }

        for i in 0..g1.cells.len() {
            for j in 0..g1.cells.len() {
                if g1.cells[i][j] != g2.cells[i][j] {
                    return Ok(false);
                }
            }
        }

        return Ok(true);
    }
}

pub fn clear_screen() {
    print!("{esc}[{n}D{esc}[{n}A", n = GRID_SIZE, esc = 27 as char);
}
