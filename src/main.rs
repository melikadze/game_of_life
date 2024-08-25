use std::{thread, time::Duration};
use crossterm::{ExecutableCommand, terminal, cursor};
use std::io::{stdout, Write};

const GRID_SIZE: usize = 20;

fn main() {
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    grid[GRID_SIZE/2-1][GRID_SIZE/2-1] = 1;
    grid[GRID_SIZE/2-2][GRID_SIZE/2-1] = 1;
    grid[GRID_SIZE/2-3][GRID_SIZE/2-1] = 1;
    grid[GRID_SIZE/2-2][GRID_SIZE/2-2] = 1;
    grid[GRID_SIZE/2-1][GRID_SIZE/2] = 1;

    loop {
        clear_terminal();
        println!("Tick!");
        print_grid(&grid);

        let new_grid: [[u8; GRID_SIZE]; GRID_SIZE] = next_generation(&grid);
        grid = new_grid;

        thread::sleep(Duration::from_millis(100));
    }
}

fn next_generation(grid: &[[u8; GRID_SIZE]; GRID_SIZE]) -> [[u8; GRID_SIZE]; GRID_SIZE] {
    let mut new_grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let alive_neighbors = count_alive_neighbors(grid, row, col);
            new_grid[row][col] = match (grid[row][col], alive_neighbors) {
                (1, 2) | (_, 3) => 1,
                _ => 0,
            };
        }
    }
    
    new_grid
}

fn count_alive_neighbors(grid: &[[u8; GRID_SIZE]; GRID_SIZE], row: usize, col: usize) -> u8 {
    let mut count = 0;
    
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            
            if new_row >= 0 && new_row < GRID_SIZE as isize && new_col >= 0 && new_col < GRID_SIZE as isize {
                count += grid[new_row as usize][new_col as usize];
            }
        }
    }
    
    count
}

fn print_grid(grid: &[[u8; GRID_SIZE]; GRID_SIZE]) {
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{:2} ", if cell == 0 { ' ' } else { '*' });
        }
        println!();
    }
}

fn clear_terminal() {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).expect("Failed to clear terminal");
    stdout.execute(cursor::MoveTo(0, 0)).expect("Failed to move cursor to the start of line");
    stdout.flush().expect("Failed to flush stdout");
}
