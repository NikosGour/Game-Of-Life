mod game_of_life;
use game_of_life::{clear_screen, Grid};
use inline_colorization::*;
use std::{thread, time};
fn main() -> color_anyhow::anyhow::Result<()> {
    color_anyhow::install()?;

    let mut grid: Grid = Grid::new();
    grid.place_glider(0, 0)?;
    let mut first: bool = true;

    while grid.running {
        if !first {
            clear_screen();
        }
        grid.display();
        grid.play_next()?;
        thread::sleep(time::Duration::from_millis(100));
        first = false;
    }
    println!("{color_bright_green}Halted!{style_reset}");
    Ok(())
}
