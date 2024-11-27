use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;

#[derive(Clone)]
pub struct Binairo {
    grid: Vec<Vec<(usize, usize, i32)>>,
    size: i32,
}

impl Binairo {
    pub fn new(size: i32) -> Self {
        let mut grid: Vec<Vec<(usize, usize, i32)>> = Vec::with_capacity(size as usize);

        for i in 0..size {
            let mut row = Vec::with_capacity(size as usize);
            for j in 0..size {
                row.push((i as usize, j as usize, -1i32));
            }
            grid.push(row);
        }

        Self { grid, size }
    }

    #[allow(dead_code)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        // Read all lines from the file
        let file = File::open(path)?;
        let lines: Vec<String> = io::BufReader::new(file)
            .lines()
            .collect::<io::Result<_>>()?;

        // Get the size from the first line
        let size = lines[0].split(',').count() as i32;

        // Create a new Binairo instance
        let mut binairo = Binairo::new(size);

        // Parse each line and set the initial values
        for (row, line) in lines.iter().enumerate() {
            let values: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse::<i32>().unwrap_or(-1))
                .collect();

            for (col, &value) in values.iter().enumerate() {
                if value != -1 {
                    binairo.grid[row][col].2 = value;
                }
            }
        }

        Ok(binairo)
    }

    pub fn from_image(image_path: &str) -> std::io::Result<Self> {
        // Run the Python script to process the image
        let output = Command::new("python3")
            .arg("scripts/main.py")
            .arg(image_path)
            .output()?;

        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Python script failed to execute",
            ));
        }

        // Read the generated result file
        let content = fs::read_to_string("result.txt")?;
        let lines: Vec<&str> = content.lines().collect();

        // Create a new Binairo with the appropriate size
        let size = lines[0].split(',').count() as i32;
        let mut binairo = Binairo::new(size);

        // Parse the values and set them in the grid
        for (row, line) in lines.iter().enumerate() {
            let values: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse::<i32>().unwrap_or(-1))
                .collect();

            for (col, &value) in values.iter().enumerate() {
                binairo.grid[row][col].2 = value;
            }
        }

        // Clean up the temporary file
        fs::remove_file("result.txt")?;

        Ok(binairo)
    }

    pub fn print_grid(&self) {
        for row in &self.grid {
            for &cell in row {
                print!(
                    "{}",
                    match cell {
                        (_, _, -1) => '⬜',
                        (_, _, 0) => '🟦',
                        (_, _, 1) => '🟥',
                        (_, _, _) => '?',
                    }
                );
            }
            println!();
        }
    }

    // Method to set initial grid values
    #[allow(dead_code)]
    pub fn set_initial_values(&mut self, initial_values: &[(usize, usize, i32)]) {
        for &(row, col, value) in initial_values {
            if row < self.size as usize && col < self.size as usize {
                self.grid[row][col].2 = value;
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&x| x.2 != -1)) && self.is_valid()
    }

    pub fn is_valid(&self) -> bool {
        // Check for three consecutive same numbers horizontally and vertically
        for row in 0..self.size as usize {
            for col in 0..self.size as usize {
                // Skip unfilled cells
                if self.grid[row][col].2 == -1 {
                    continue;
                }

                // Check vertically (going down)
                if row + 2 < self.size as usize {
                    if self.grid[row][col].2 != -1
                        && self.grid[row + 1][col].2 == self.grid[row][col].2
                        && self.grid[row + 2][col].2 == self.grid[row][col].2
                    {
                        return false;
                    }
                }

                // Check horizontally (going right)
                if col + 2 < self.size as usize {
                    if self.grid[row][col].2 != -1
                        && self.grid[row][col + 1].2 == self.grid[row][col].2
                        && self.grid[row][col + 2].2 == self.grid[row][col].2
                    {
                        return false;
                    }
                }
            }
        }

        // Check for equal number of 0s and 1s in each row and column
        for index in 0..self.size as usize {
            let row_count_0 = self.grid[index].iter().filter(|&&x| x.2 == 0).count();
            let row_count_1 = self.grid[index].iter().filter(|&&x| x.2 == 1).count();
            let col_count_0 = (0..self.size as usize)
                .filter(|&i| self.grid[i][index].2 == 0)
                .count();
            let col_count_1 = (0..self.size as usize)
                .filter(|&i| self.grid[i][index].2 == 1)
                .count();

            // For completed rows/columns, check exact equality
            if self.grid[index].iter().all(|&x| x.2 != -1) && row_count_0 != row_count_1 {
                return false;
            }

            let col_complete = (0..self.size as usize).all(|i| self.grid[i][index].2 != -1);
            if col_complete && col_count_0 != col_count_1 {
                return false;
            }

            // For incomplete rows/columns, check if we haven't exceeded half
            if row_count_0 > self.size as usize / 2
                || row_count_1 > self.size as usize / 2
                || col_count_0 > self.size as usize / 2
                || col_count_1 > self.size as usize / 2
            {
                return false;
            }
        }

        // Check for duplicate rows and columns
        for i in 0..self.size as usize {
            for j in 0..i {
                // Check for duplicate rows
                if self.grid[i].iter().all(|&x| x.2 != -1)
                    && self.grid[j].iter().all(|&x| x.2 != -1)
                {
                    if self.grid[i]
                        .iter()
                        .zip(self.grid[j].iter())
                        .all(|(a, b)| a.2 == b.2)
                    {
                        return false;
                    }
                }

                // Check for duplicate columns
                let col_i: Vec<i32> = (0..self.size as usize).map(|x| self.grid[x][i].2).collect();
                let col_j: Vec<i32> = (0..self.size as usize).map(|x| self.grid[x][j].2).collect();

                if col_i.iter().all(|&x| x != -1) && col_j.iter().all(|&x| x != -1) {
                    if col_i == col_j {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn solve(&mut self) -> bool {
        if !self.is_valid() {
            return false;
        }

        if self.is_solved() {
            return true;
        }

        // Find cell with minimum possibilities
        let mut min_possibilities = 3;
        let mut best_cell = None;

        for row in 0..self.size as usize {
            for col in 0..self.size as usize {
                if self.grid[row][col].2 == -1 {
                    let mut possibilities = 0;
                    for value in [0, 1].iter() {
                        self.grid[row][col].2 = *value;
                        if self.is_valid() {
                            possibilities += 1;
                        }
                        self.grid[row][col].2 = -1;
                    }
                    if possibilities > 0 && possibilities < min_possibilities {
                        min_possibilities = possibilities;
                        best_cell = Some((row, col));
                    }
                }
            }
        }

        match best_cell {
            None => false,
            Some((row, col)) => {
                for value in [0, 1].iter() {
                    self.grid[row][col].2 = *value;
                    if self.is_valid() && self.solve() {
                        return true;
                    }
                    self.grid[row][col].2 = -1;
                }
                false
            }
        }
    }
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

    #[test]
    fn test_validity() {
        let mut game = Binairo::new(4);
        game.set_initial_values(&[
            (0, 0, 0),
            (0, 1, 1),
            (0, 2, 0),
            (0, 3, 1),
            (1, 0, 0),
            (1, 1, 1),
            (1, 2, 1),
            (1, 3, 0),
            (2, 0, 1),
            (2, 1, 0),
            (2, 2, 1),
            (2, 3, 0),
            (3, 0, 1),
            (3, 1, 0),
            (3, 2, 0),
            (3, 3, 1),
        ]);

        assert!(game.is_valid());
    }

    #[test]
    fn test_is_solved() {
        let mut game = Binairo::new(4);
        game.set_initial_values(&[
            (0, 0, 0),
            (0, 1, 1),
            (0, 2, 0),
            (0, 3, 1),
            (1, 0, 0),
            (1, 1, 1),
            (1, 2, 1),
            (1, 3, 0),
            (2, 0, 1),
            (2, 1, 0),
            (2, 2, 1),
            (2, 3, 0),
            (3, 0, 1),
            (3, 1, 0),
            (3, 2, 0),
            (3, 3, 1),
        ]);

        assert!(game.is_solved());
    }
}
