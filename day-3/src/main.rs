use std::{fs};

fn main() {
    let input = get_input();

    part_two(input);
}

fn get_input() -> Vec<String> {
    let file_path = "./src/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return Vec::from_iter(contents.split("\n").map(String::from));
}

fn part_one(input: Vec<String>) {
    let mut total_priorities = 0;

    for string in input {
        let (first, second) = string.split_at(string.chars().count() / 2);

        let mut last_found = '.';

        let mut rucksack_priorities: u32 = 0;

        for char in first.chars() {
            if second.contains(char) {
                if last_found.eq(&char) {
                    continue;
                }

                last_found = char;

                if char.is_uppercase() {
                    rucksack_priorities += 27 + (char as u32 - 'A' as u32);

                    continue;
                }

                rucksack_priorities += 1 + (char as u32 - 'a' as u32);
            }
        }

        total_priorities += rucksack_priorities;
    }

    println!("total priorities {total_priorities}");
}

fn part_two(input: Vec<String>) {
    let mut total_values = 0;

    for chunk in input.chunks(3) {
        match chunk {
            [first, second, third] => {
                total_values += get_badge_value_for_group(first, second, third)
                    .expect("Something went wrong");
            },
            _ => {
                panic!("Somethin went wrong with input values");
            }
        }
    }

    println!("total badge values are {total_values}")
}

fn get_badge_value_for_group(first: &String, second: &String, third: &String) -> Result<u32, &'static str> {
    for char in first.chars() {
        if !second.contains(char) || !third.contains(char) {
            continue;
        }

        if char.is_uppercase() {
            return Ok(27 + (char as u32 - 'A' as u32));
        }

        return Ok(1 + (char as u32 - 'a' as u32));
    }

    Err("Something went wrong with badge groups, no matching characters")
}