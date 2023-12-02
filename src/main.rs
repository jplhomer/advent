pub fn main() {
    puzzle_01_pt1();
    puzzle_01_pt2();
    puzzle_02();
}

pub fn puzzle_01_pt1() {
    // Load input.txt
    let input = include_str!("../fixtures/input_01.txt");

    // Split input into lines
    let lines = input.lines();

    let mut sum: i32 = 0;

    lines.for_each(|line| {
        let string_with_numbers: String = line.parse().unwrap();
        let characters = string_with_numbers.chars().filter(|c| c.is_numeric());
        let first_and_last_numbers_concatenated: String = characters
            .clone()
            .take(1)
            .chain(characters.clone().rev().take(1))
            .collect();

        sum += first_and_last_numbers_concatenated.parse::<i32>().unwrap();
    });

    println!("Puzzle 01, Part 1: {}", sum);
}

pub fn puzzle_01_pt2() {
    let lines = include_str!("../fixtures/input_01.txt").lines();

    let mut sum: i32 = 0;

    lines.for_each(|line| {
        sum += find_concatenated_first_and_last_numbers_from_string(line)
            .parse::<i32>()
            .unwrap();
    });

    println!("Puzzle 01, Part 2: {}", sum);
}

fn find_concatenated_first_and_last_numbers_from_string(string: &str) -> String {
    let first_number = find_first_number_in_string(string, false);
    let last_number = find_first_number_in_string(string, true);
    format!("{}{}", first_number, last_number)
}

fn find_first_number_in_string(string: &str, reverse: bool) -> String {
    // If we're reversing, reverse the string
    let string = if reverse {
        string.chars().rev().collect::<String>()
    } else {
        string.to_string()
    };

    let spelled_out_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // If we're reversing, reverse the spelled out numbers
    let spelled_out_numbers = if reverse {
        spelled_out_numbers
            .iter()
            .map(|r| r.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
    } else {
        spelled_out_numbers.map(|r| r.to_string()).to_vec()
    };

    // Scan from the left to find either a spelled out number or a numeric number
    let mut index = 0;
    let mut number = String::new();
    while index < string.len() {
        let char = string.chars().nth(index).unwrap();

        // It's a digit; use it.
        if char.is_numeric() {
            number.push(char);
            break;
        }

        for number_word in spelled_out_numbers.iter() {
            // It matches a spelled out number; use it.
            if string[index..].starts_with(number_word) {
                number.push_str(&format!(
                    "{}",
                    spelled_out_numbers
                        .iter()
                        .position(|r| r == number_word)
                        .unwrap()
                        + 1
                ));
                break;
            }
        }

        if number.len() > 0 {
            break;
        }

        // Otherwise, keep scanning
        index += 1;
    }

    number
}

#[test]
fn it_returns_concatenated_numbers_from_string() {
    assert_eq!(
        find_concatenated_first_and_last_numbers_from_string("1234"),
        "14"
    );

    assert_eq!(
        find_concatenated_first_and_last_numbers_from_string("91212129"),
        "99"
    );

    assert_eq!(
        find_concatenated_first_and_last_numbers_from_string("1212"),
        "12"
    );

    assert_eq!(
        find_concatenated_first_and_last_numbers_from_string("two1nine"),
        "29",
    );

    assert_eq!(
        find_concatenated_first_and_last_numbers_from_string("7pqrstsixteen"),
        "76",
    );
}

pub fn puzzle_02() {
    let input = include_str!("../fixtures/input_02.txt");

    let set = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = input
        .lines()
        .map(|line| parse_game(line.to_string()))
        .collect::<Vec<Game>>();

    let mut possible_games: Vec<&Game> = Vec::new();
    let mut sum_of_minumum_powers = 0;

    for game in games.iter() {
        sum_of_minumum_powers += game.minimum_power;
        if game_is_possible_for_set(&set, game) {
            possible_games.push(game.clone());
        }
    }

    let sum_of_game_ids = possible_games.iter().fold(0, |sum, game| sum + game.id);

    println!(
        "Puzzle 02: {} (total games), {} (sum), {} (sum of minimum powers)",
        possible_games.len(),
        sum_of_game_ids,
        sum_of_minumum_powers
    );
}

struct Game {
    id: i32,
    cube_sets: Vec<CubeSet>,
    minimum_power: i32,
}

struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_game(game: String) -> Game {
    let mut cube_sets: Vec<CubeSet> = Vec::new();

    // Where game is a in the format of "Game 1: 1 red, 5 blue; 3 red, 1 green; 2 red, 1 green, 1 blue"
    let id = game
        .split(":")
        .nth(0)
        .unwrap()
        .replace("Game ", "")
        .parse::<i32>()
        .unwrap();

    let game = game.split(":").nth(1).unwrap().to_string();

    let mut max_cube_set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    game.split(";").for_each(|set| {
        let mut cube_set = CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        };

        set.split(",").for_each(|cube| {
            let cube = cube.trim();
            let number = cube.split(" ").nth(0).unwrap().parse::<i32>().unwrap();
            let color = cube.split(" ").nth(1).unwrap();

            match color {
                "red" => {
                    cube_set.red += number;
                    if cube_set.red > max_cube_set.red {
                        max_cube_set.red = cube_set.red;
                    }
                }
                "green" => {
                    cube_set.green += number;
                    if cube_set.green > max_cube_set.green {
                        max_cube_set.green = cube_set.green;
                    }
                }
                "blue" => {
                    cube_set.blue += number;
                    if cube_set.blue > max_cube_set.blue {
                        max_cube_set.blue = cube_set.blue;
                    }
                }
                _ => panic!("Unknown cube color: {}", cube),
            }
        });

        cube_sets.push(cube_set);
    });

    let minimum_power = max_cube_set.red * max_cube_set.green * max_cube_set.blue;

    Game {
        id,
        cube_sets,
        minimum_power,
    }
}

#[test]
fn it_parses_games() {
    let game = "Game 1: 1 red, 5 blue; 3 red, 1 green; 2 red, 1 green, 1 blue".to_string();
    let game = parse_game(game);

    assert_eq!(game.id, 1);
    assert_eq!(game.cube_sets.len(), 3);
    assert_eq!(game.minimum_power, 15);

    assert_eq!(game.cube_sets[0].red, 1);
    assert_eq!(game.cube_sets[0].green, 0);
    assert_eq!(game.cube_sets[0].blue, 5);

    assert_eq!(game.cube_sets[1].red, 3);
    assert_eq!(game.cube_sets[1].green, 1);
    assert_eq!(game.cube_sets[1].blue, 0);

    assert_eq!(game.cube_sets[2].red, 2);
    assert_eq!(game.cube_sets[2].green, 1);
    assert_eq!(game.cube_sets[2].blue, 1);
}

fn game_is_possible_for_set(set: &CubeSet, game: &Game) -> bool {
    for game_set in game.cube_sets.iter() {
        if game_set.green > set.green || game_set.red > set.red || game_set.blue > set.blue {
            return false;
        }
    }

    true
}

#[test]
fn it_knows_whether_game_is_possible_for_set() {
    let game = "Game 1: 1 red, 5 blue; 3 red, 1 green; 2 red, 1 green, 1 blue".to_string();
    let game = parse_game(game);

    let set = CubeSet {
        red: 3,
        green: 1,
        blue: 5,
    };
    assert_eq!(game_is_possible_for_set(&set, &game), true);

    let set = CubeSet {
        red: 1,
        green: 1,
        blue: 2,
    };
    assert_eq!(game_is_possible_for_set(&set, &game), false);

    let set = CubeSet {
        red: 1,
        green: 2,
        blue: 1,
    };
    assert_eq!(game_is_possible_for_set(&set, &game), false);

    let set = CubeSet {
        red: 2,
        green: 1,
        blue: 1,
    };
    assert_eq!(game_is_possible_for_set(&set, &game), false);
}
