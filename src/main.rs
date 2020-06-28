extern crate sudoku;
use sudoku::puzzle::SudokuPuzzle;

use std::io;

fn main() -> io::Result<()> {
    println!("Please enter the puzzle as a sequence of 81 numbers.");
    println!("Use '0' to indicate an empty space.");
    println!("You may include line breaks, but no other whitespace.");
    println!("Press enter on an empty line when you are done.\n");

    let mut puzzle_source = String::new();
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;

        if buffer.trim_end().is_empty() {
            break;
        } else {
            puzzle_source.push_str(&buffer);
        }
    }

    let puzzle = SudokuPuzzle::from_string(&puzzle_source);
    println!("Input:\n{:?}\n\n", puzzle);

    match puzzle.solve() {
        Some(solution) => println!("Solution:\n{:?}", solution),
        None => println!("No solution could be found"),
    }

    Ok(())
}

// Example input:
//
// 410036000
// 007000850
// 600000000
// 090000200
// 006070008
// 000000091
// 002014000
// 000003000
// 740008509
