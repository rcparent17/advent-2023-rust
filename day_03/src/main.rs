use std::io::Read;

const SYMBOLS: &str = "-=+/*@#$%&";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("PASS INPUT FILE AS FIRST AND ONLY COMMAND LINE ARGUMENT");
        return;
    }

    let mut file: std::fs::File = std::fs::File::open(&args[1]).unwrap();
    let mut contents: String = String::new();
    let _ = file.read_to_string(&mut contents);
    let char_map: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    let part_1_result: usize = part_1(char_map.clone());
    println!("Part 1 result: {}", part_1_result);

    let part_2_result: usize = part_2(char_map.clone());
    println!("Part 2 result: {}", part_2_result);
}

/*********************************************************************************
* The engine schematic (your puzzle input) consists of a visual representation of*
* the engine. There are lots of numbers and symbols you don't really understand, *
* but apparently any number adjacent to a symbol (not .), even diagonally, is a  *
* "part number" and should be included in your sum. (Periods (.) do not count as *
* a symbol.)                                                                     *
*                                                                                *
* What is the sum of all of the part numbers in the engine schematic?            *
*********************************************************************************/

#[derive(Debug)]
struct SchematicNumber {
    row: usize,
    col: usize,
    len: usize,
    value: usize,
}

impl SchematicNumber {
    fn new(schematic: Vec<Vec<char>>, position: (usize, usize)) -> Self {
        let mut len: usize = 1;

        while schematic[position.0][position.1 + len].is_numeric() {
            if position.1 + len + 1 >= schematic[position.0].len() {
                break;
            }
            len += 1;
        }

        let value: usize = schematic[position.0][position.1..position.1 + len]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();

        SchematicNumber {
            row: position.0,
            col: position.1,
            len,
            value,
        }
    }
    fn is_part_number(&self, schematic: Vec<Vec<char>>) -> bool {
        let mut is_valid: bool = false;

        let row_bounds: (i32, i32) = (
            (self.row as i32 - 1).clamp(0, schematic.len() as i32),
            (self.row as i32 + 2).clamp(0, schematic.len() as i32),
        );
        let row_bounds: (usize, usize) = (row_bounds.0 as usize, row_bounds.1 as usize);
        let col_bounds: (i32, i32) = (
            (self.col as i32 - 1).clamp(0, schematic[self.row].len() as i32),
            ((self.col + self.len) as i32 + 1).clamp(0, schematic[self.row].len() as i32),
        );
        let col_bounds: (usize, usize) = (col_bounds.0 as usize, col_bounds.1 as usize);
        for row in &schematic[row_bounds.0..row_bounds.1] {
            for symbol in SYMBOLS.chars() {
                if row[col_bounds.0..col_bounds.1].contains(&symbol) {
                    is_valid = true;
                    break;
                }
            }
        }
        is_valid
    }
}

fn part_1(schematic: Vec<Vec<char>>) -> usize {
    let mut solution: usize = 0;
    let mut schematic_numbers: Vec<SchematicNumber> = Vec::new();
    for (r, row) in schematic.iter().enumerate() {
        let mut offset: usize = 0;
        for (c, _ch) in row.iter().enumerate() {
            if c + offset >= row.len() {
                break;
            }
            if row[c + offset].is_numeric() {
                let current_number: SchematicNumber =
                    SchematicNumber::new(schematic.clone(), (r, c + offset));
                offset += current_number.len;
                schematic_numbers.push(current_number);
            }
        }
    }
    for number in schematic_numbers {
        if number.is_part_number(schematic.clone()) {
            solution += number.value;
        }
    }
    solution
}

/*********************************************************************************
* The missing part wasn't the only issue - one of the gears in the engine is     *
* wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its*
* gear ratio is the result of multiplying those two numbers together.            *
*                                                                                *
* This time, you need to find the gear ratio of every gear and add them all up so*
* that the engineer can figure out which gear needs to be replaced.              *
*********************************************************************************/
fn part_2(_schematic: Vec<Vec<char>>) -> usize {
    let solution: usize = 0;
    solution
}
