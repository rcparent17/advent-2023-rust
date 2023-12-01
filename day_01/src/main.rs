use std::io::BufRead;

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