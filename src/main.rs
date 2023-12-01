pub fn main() {
    puzzle_01_pt1();
    puzzle_01_pt2();
}

pub fn puzzle_01_pt1() {
    // Load input.txt
    let input = include_str!("../fixtures/input.txt");

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
    let lines = include_str!("../fixtures/input.txt").lines();

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
