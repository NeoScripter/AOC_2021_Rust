use std::fmt;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<u8>>,
    flash_count: u32,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for num in row {
                write!(f, "{:>1}", num)?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}
impl Grid {
    fn step(&mut self) {
        let mut to_flash = Vec::new();
    
        // First Phase: Update all cells and mark cells to flash
        self.grid.iter_mut().enumerate().for_each(|(row_idx, row)| {
            row.iter_mut().enumerate().for_each(|(oc_idx, oct)| {
                *oct += 1;
                if *oct > 9 {
                    to_flash.push((oc_idx, row_idx));
                    *oct = 0; // Reset the flashed cell immediately
                }
            });
        });
    
        // Second Phase: Handle flashes
        for (x, y) in to_flash {
            if self.grid[y][x] == 0 { // Check if the cell hasn't already flashed
                self.flash(x, y);
            }
        }
    }    
    fn flash(&mut self, x: usize, y: usize) {
        self.flash_count += 1;

        // Define a closure for updating and flashing octopuses
        let mut update_and_flash = |x: isize, y: isize| {
            if x >= 0 && y >= 0 && x < self.grid.len() as isize && y < self.grid[0].len() as isize {
                let oct = &mut self.grid[y as usize][x as usize];
                if *oct > 0 {                
                    *oct += 1;
                    if *oct > 9 {
                        *oct = 0; // Reset the value after flash
                        self.flash(x as usize, y as usize);
                    }
                }
            }
        };

        // Call the closure for the current position and all eight surrounding positions
        for dx in -1..=1 {
            for dy in -1..=1 {
                update_and_flash(x as isize + dx, y as isize + dy);
            }
        }
    }
}

fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<u8>> = input.lines()
    .map(|l| l.chars().map(|c| c.to_digit(10).expect("Error parsing char") as u8).collect())
    .collect();
    let mut grid = Grid {
        grid: grid,
        flash_count: 0,
    };
    //println!("{}", grid);
    for n in 0.. {
        if grid.grid.iter().flatten().all_equal() {
            return n
        }
        grid.step();
    }
    //println!("{}", grid);
    grid.flash_count
}
fn main() {
    let input = include_str!("input11.txt");
    println!("{}", part1(input));
}