use regex::Regex;
use std::io::BufRead;

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

const HANDFUL_REGEX_PATTERN: &str = r"\s*(?<number>\d+)\s(?<color>green|red|blue)\s*";
const GAME_ID_REGEX_PATTERN: &str = r"Game\s(?<game_id>\d+)";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("PASS INPUT FILE AS FIRST AND ONLY COMMAND LINE ARGUMENT");
        return;
    }

    let filename: &str = &args[1];
    let file = std::fs::File::open(filename).unwrap();
    let input_lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|x| x.trim().to_string())
        .collect();

    let part_1_result: u32 = part_1(input_lines.clone());
    println!("Part 1 result: {}", part_1_result);

    let part_2_result: u32 = part_2(input_lines.clone());
    println!("Part 2 result: {}", part_2_result);
}

/*********************************************************************************
* Determine which games would have been possible if the bag had been loaded with *
* only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the   *
* IDs of those games?                                                            *
*********************************************************************************/
fn part_1(lines: Vec<String>) -> u32 {
    let id_regex: Regex = Regex::new(GAME_ID_REGEX_PATTERN).unwrap();
    let handful_regex: Regex = Regex::new(HANDFUL_REGEX_PATTERN).unwrap();
    let mut solution: u32 = 0;
    for line in lines {
        let mut valid_game: bool = true;
        let game_id: u32 = id_regex
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let handful_data: &str = line.split(':').collect::<Vec<&str>>()[1];
        let handfuls: Vec<&str> = handful_data.split(';').collect();
        'handful_loop: for handful in handfuls {
            for cap in handful_regex.captures_iter(handful) {
                let num: u32 = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let color = cap.get(2).unwrap().as_str();
                valid_game = match color {
                    "red" => num <= RED_MAX,
                    "green" => num <= GREEN_MAX,
                    "blue" => num <= BLUE_MAX,
                    _ => false,
                };
                if !valid_game {
                    break 'handful_loop;
                }
            }
        }
        if valid_game {
            solution += game_id;
        }
    }
    solution
}

/*********************************************************************************
* For each game, find the smallest number of red, green, and blue cubes needed   *
* for the game to be possible. Multiply these together for each game and sum up  *
* all of these products.                                                         *
*********************************************************************************/
fn part_2(lines: Vec<String>) -> u32 {
    let handful_regex: Regex = Regex::new(HANDFUL_REGEX_PATTERN).unwrap();
    let mut solution: u32 = 0;
    for line in lines {
        let mut min_needed: Vec<u32> = vec![0, 0, 0]; // 0->red, 1->green, 2->blue
        let handful_data: &str = line.split(':').collect::<Vec<&str>>()[1];
        let handfuls: Vec<&str> = handful_data.split(';').collect();
        for handful in handfuls {
            for cap in handful_regex.captures_iter(handful) {
                let num: u32 = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let color = cap.get(2).unwrap().as_str();
                match color {
                    "red" => {
                        if num > min_needed[0] {min_needed[0] = num}
                    },
                    "green" => {
                        if num > min_needed[1] {min_needed[1] = num}
                    },
                    "blue" => {
                        if num > min_needed[2] {min_needed[2] = num}
                    },
                    _ => (),
                };
            }
        }
        solution += min_needed.iter().product::<u32>();
    }
    solution
}