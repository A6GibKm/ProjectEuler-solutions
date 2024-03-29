// Su Doku (Japanese meaning number place) is the name given to a popular puzzle concept. Its origin is unclear, but credit must be attributed to Leonhard Euler who invented a similar, and much more difficult, puzzle idea called Latin Squares. The objective of Su Doku puzzles, however, is to replace the blanks (or zeros) in a 9 by 9 grid in such that each row, column, and 3 by 3 box contains each of the digits 1 to 9. Below is an example of a typical starting puzzle grid and its solution grid.

// A well constructed Su Doku puzzle has a unique solution and can be solved by logic, although it may be necessary to employ "guess and test" methods in order to eliminate options (there is much contested opinion over this). The complexity of the search determines the difficulty of the puzzle; the example above is considered easy because it can be solved by straight forward direct deduction.

// The 6K text file, sudoku.txt (right click and 'Save Link/Target As...'), contains fifty different Su Doku puzzles ranging in difficulty, but all with unique solutions (the first puzzle in the file is the example above).

// By solving all fifty puzzles find the sum of the 3-digit numbers found in the top left corner of each solution grid; for example, 483 is the 3-digit number found in the top left corner of the solution grid above.

// #[warn_dead_code]

fn sum_block(sudoku: [&str; 9], n: u8) -> u8 {
    assert!(n < 10);
    let mut digits: Vec<u8> = Vec::new();
    for i in 0..10 {
        digits.push(sudoku[n as usize][i..i + 1].parse().unwrap())
    }
    digits.iter().sum()
}

fn sum_row(sudoku: [&str; 9], n: u8) -> u8 {
    assert!(n < 10);
    let mut digits: Vec<u8> = Vec::new();
    for i in 0..10 {
        digits.push(sudoku[n as usize][i..i + 1].parse().unwrap())
    }
    digits.iter().sum()
}

fn sum_column(sudoku: [&str; 9], n: u8) -> u8 {
    assert!(n < 10);
    let mut digits: Vec<u8> = Vec::new();
    for i in 0..10 {
        digits.push(
            sudoku[i as usize][n as usize..(n + 1) as usize]
                .parse()
                .unwrap(),
        )
    }
    digits.iter().sum()
}

fn solve(sudoku: [&str; 9]) -> [&str; 9] {
    ["000000000"; 9]
}

fn sum_digits(sudoku: [&str; 9]) -> u64 {
    let digits: String = String::from(&sudoku[0][0..3]);
    digits[0..1].parse::<u64>().unwrap()
        + digits[1..2].parse::<u64>().unwrap()
        + digits[2..3].parse::<u64>().unwrap()
}

fn main() {
    let list = [];
    let solution: u64 = list.iter().map(|&i| sum_digits(solve(i))).sum();
    println!("{}", solution)
}

#[test]
fn test_sudoku() {
    let test_sudoku: [&str; 9] = [
        "003020600",
        "900305001",
        "001806400",
        "008102900",
        "700000000",
        "006708200",
        "002609500",
        "800203009",
        "005010300",
    ];

    let test_sudoku_sol: [&str; 9] = [
        "483921657",
        "967345821",
        "251876493",
        "548132976",
        "729564138",
        "136798245",
        "372689514",
        "814253769",
        "695417382",
    ];
    assert_eq!(sum_digits(test_sudoku_sol), 15);
    assert_eq!(solve(test_sudoku), test_sudoku_sol);
}
