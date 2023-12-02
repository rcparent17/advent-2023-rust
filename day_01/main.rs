use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("PASS INPUT FILE AS FIRST AND ONLY COMMAND LINE ARGUMENT");
        return;
    }

    let filename: &String = &args[1];
    let file = std::fs::File::open(filename).unwrap();
    let input_lines: Vec<String> = std::io::BufReader::new(file).lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let part_1_result = part_1(input_lines.clone());
    println!("Part 1 result: {}", part_1_result);

    let part_2_result: i32 = part_2(input_lines.clone());
    println!("Part 2 result: {}", part_2_result);
}

// for each line, form a number by concatenating the first and last digits.
// if there is only one digit, use that digit as both the leftmost and rightmost. 
fn part_1(lines: Vec<String>) -> i32 {
    let mut result: i32 = 0;
    for line in lines {
        let mut leftmost: char = 'z';
        let mut rightmost: char = 'z';
        for c in line.chars() {
            if c.is_numeric() {
                if leftmost == 'z' {
                    leftmost = c;
                }
                rightmost = c;
            }
        }
        let leftmost_int: i32 = (leftmost.to_string()).parse::<i32>().unwrap();
        let rightmost_int: i32 = (rightmost.to_string()).parse::<i32>().unwrap();
        result += (leftmost_int * 10) + rightmost_int;
    }
    return result;
}

// same as part 1, but the numbers (1-9) can be spelled out
fn part_2(lines: Vec<String>) -> i32 {
    let mut result: i32 = 0;

    // keys: spelled numbers
    // values: equivalent numeric characters
    let word_values: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for line in lines {
        
        // keys: indices in string
        // values: numeric characters at key index
        let mut numeric_numbers: HashMap<usize, char> = HashMap::new();
        let mut i = 0;
        for c in line.chars() {
            if c.is_numeric(){
                numeric_numbers.insert(i, c);
            }
            i += 1;
        }

        // keys: indices in string
        // values: numeric characters at key index
        let mut written_numbers: HashMap<usize, char> = HashMap::new();
        for (written, value) in word_values.iter(){
            let indices = line.match_indices(written);
                for index in indices {
                    written_numbers.insert(index.0, *value);
                }
        }

        let numeric_indices: Vec<usize> = numeric_numbers.clone().into_keys().collect();
        let written_indices: Vec<usize> = written_numbers.clone().into_keys().collect();

        let leftmost = {
            let numeric_min = numeric_indices.clone().into_iter().min().unwrap();
            let written_min = written_indices.clone().into_iter().min().unwrap_or(line.len());
            if numeric_min <= written_min {
                *numeric_numbers.get(&numeric_min).unwrap()
            }
            else {
                *written_numbers.get(&written_min).unwrap()
            }
        };

        let rightmost = {
            let numeric_max = numeric_indices.into_iter().max().unwrap();
            let written_max = written_indices.into_iter().max().unwrap_or(0);
            if numeric_max >= written_max {
                *numeric_numbers.get(&numeric_max).unwrap()
            }
            else {
                *written_numbers.get(&written_max).unwrap()
            }
        };

        let leftmost_int: i32 = (leftmost.to_string()).parse::<i32>().unwrap();
        let rightmost_int: i32 = (rightmost.to_string()).parse::<i32>().unwrap();
        result += (leftmost_int * 10) + rightmost_int;
    }
    return result;
}