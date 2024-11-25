#[derive(Clone)]
struct Binairo {
    grid: Vec<Vec<i32>>,
    size: i32,
}

impl Binairo {
    pub fn new(size: i32) -> Self {
        let grid = vec![vec![-1; size as usize]; size as usize];
        Self { grid, size }
    }

    pub fn print_grid(&self) {
        for row in &self.grid {
            for &cell in row {
                print!(
                    "{}  ",
                    match cell {
                        -1 => '.', // Unset cell
                        0 => '0',
                        1 => '1',
                        _ => '?',
                    }
                );
            }
            println!();
        }
    }

    // Method to set initial grid values
    pub fn set_initial_values(&mut self, initial_values: &[(usize, usize, i32)]) {
        for &(row, col, value) in initial_values {
            if row < self.size as usize && col < self.size as usize {
                self.grid[row][col] = value;
            }
        }
    }
}

fn main() {
    let mut game_4x4 = Binairo::new(4);

    // Example: Add some initial values
    game_4x4.set_initial_values(&[(0, 0, 0), (1, 0, 0), (1, 3, 0), (2, 2, 1)]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_grid() {
        let game = Binairo::new(4);

        assert_eq!(game.grid.len(), 4);
        assert_eq!(game.grid[0].len(), 4);
    }
}
