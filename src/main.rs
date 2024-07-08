use std::{thread, time};

use inline_colorization::*;
const GRID_SIZE: usize = 20;
struct Grid {
    cells: [[Cell; GRID_SIZE]; GRID_SIZE],
    running: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive,
}
impl Grid {
    fn new() -> Self {
        return Grid {
            cells: [[Cell::Dead; GRID_SIZE]; GRID_SIZE],
            running: true,
        };
    }

    fn display(&self) {
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

    fn place_glider(&mut self, x: usize, y: usize) {
        if x >= GRID_SIZE - 3 || y >= GRID_SIZE - 3 {
            panic!(
                "You asked to place a glider outside of the possible locations for a glider.
You gave position ({x},{y}), which is not possible to put a glider there as the glider needs
3 spaces vertically and horizontally (GRID_SIZE = {GRID_SIZE}). Change the location"
            )
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
    }

    fn play_next(&mut self) {
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
        self.cells = new_cells;
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
}

fn clear_screen() {
    print!("{esc}[{n}D{esc}[{n}A", n = GRID_SIZE, esc = 27 as char);
}
fn main() {
    let mut grid: Grid = Grid::new();
    grid.place_glider(0, 0);
    while grid.running {
        grid.display();
        grid.play_next();
        thread::sleep(time::Duration::from_millis(100));
        clear_screen();
    }
}
