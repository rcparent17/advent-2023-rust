use regex::Regex;
use std::io::BufRead;

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

const HANDFUL_REGEX_PATTERN: &str = r"\s*(?<number>\d+)\s(?<color>green|red|blue)\s*";
const GAME_ID_REGEX_PATTERN: &str = r"Game\s(?<game_id>\d+)";

pub struct CubeGame {
    pub id: u32,
    pub handfuls: [(u32, u32, u32)], // list of tuples: (# r seen, # g seen, # b seen) per handful
}

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

    let part_1_result = part_1(input_lines.clone());
    println!("Part 1 result: {}", part_1_result);

    // let part_2_result: i32 = part_2(input_lines.clone());
    // println!("Part 2 result: {}", part_2_result);
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
        //println!("{}", line);
        let handful_data: &str = line.split(':').collect::<Vec<&str>>()[1];
        let handfuls: Vec<&str> = handful_data.split(';').collect();
        for handful in handfuls {
            for cap in handful_regex.captures_iter(handful) {
                let num: u32 = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let color = cap.get(2).unwrap().as_str();
                //rintln!("{} {}", num, color);
                valid_game = match color {
                    "red" => num <= RED_MAX,
                    "green" => num <= GREEN_MAX,
                    "blue" => num <= BLUE_MAX,
                    _ => false,
                };
                println!("{} {} {}", num, color, valid_game);
                if !valid_game {
                    break;
                }
            }
            if !valid_game {
                break;
            }
        }
        println!("{}", valid_game);
        if valid_game {
            println!("{}", game_id);
            solution += game_id;
        }
    }
    solution
}
