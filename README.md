# Binairo Resolver

Binairo Resolver is a program designed to solve Binairo puzzles programmatically. Binairo, also known as Takuzu, is a binary puzzle game that involves filling a grid with 0s and 1s while adhering to specific rules.

## What is Binairo?

Binairo is a logic puzzle game played on a rectangular grid. The objective is to fill the grid with 0s and 1s while following these rules:

1. Each row and each column must contain an equal number of 0s and 1s.
2. No more than two of the same number can be adjacent to each other, either horizontally or vertically.
3. No two rows and no two columns can be the same.

## How to Play Binairo

1. Start with an empty grid or a grid with some cells pre-filled with 0s and 1s.
2. Fill the remaining cells with 0s and 1s while ensuring that:
   - Each row and each column contains an equal number of 0s and 1s.
   - No more than two of the same number are adjacent to each other.
   - No two rows and no two columns are identical.
3. The puzzle is solved when all cells are filled correctly according to the rules.

## How to Use the Binairo Resolver

I play Binairo on my tablet with the `Zuzu` app. I take a screenshot of the puzzle and use the `binairo_resolver` to solve it. The program reads the image, extracts the grid, and solves the puzzle.

### Prerequisites

- Python 3.x
- OpenCV for Python
- Rust and Cargo

### Usage

#### Using an Image

1. Prepare an image of the Binairo puzzle from the `Zuzu` app.

2. Run the program with the path to the image:
   ```sh
   cargo run --release -- <image_path> [--verbose]
   ```

   - `<image_path>`: Path to the image of the Binairo puzzle.
   - `--verbose`: Optional flag for verbose output.

#### Using a Text File

1. Create a text file representing the Binairo puzzle. Each line should contain the initial values for a row, with cells separated by commas. Use `-1` for empty cells. For example:
   ```
   0,1,-1,0
   -1,-1,1,0
   1,0,-1,-1
   -1,0,1,1
   ```

2. Run the program with the path to the text file:
   ```sh
   cargo run --release -- <file_path> [--verbose]
   ```

   - `<file_path>`: Path to the text file of the Binairo puzzle.
   - `--verbose`: Optional flag for verbose output.

#### Programmatically Setting Initial Values

You can also set the initial values programmatically using the `set_initial_values` method and then solve the puzzle by calling the `solve` method.

### Example

#### Using an Image

```sh
cargo run --release -- assets/binairo_puzzle.png --verbose
```

#### Using a Text File

```sh
cargo run --release -- assets/binairo_puzzle.txt --verbose
```

### Output

The program will display the initial grid and the solved grid in the terminal. If the puzzle is solved successfully, it will also display the time taken to solve the puzzle.

### Project Structure

- `src/`: Contains the Rust source code for the Binairo solver.
  - `binario.rs`: Contains the main logic for solving the Binairo puzzle.
  - `main.rs`: Entry point for the CLI application.
- `scripts/`: Contains the Python script for processing the image.
  - `main.py`: Uses OpenCV to detect the grid and the colors of the cells.
- `assets/`: Directory for storing example images and text files of Binairo puzzles.
- `Cargo.toml`: Configuration file for the Rust project.
- `README.md`: This file.

### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Acknowledgments

- Thanks to my girlfriend for introducing me to the Binairo puzzle game.
