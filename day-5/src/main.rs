extern crate core;

use std::fs;

fn main() {
    let (layout, instructions) = get_data_from_input();

    let mut sorted_layout_part_one = get_sorted_layout_part_one(layout.clone(), instructions.clone());
    let mut sorted_layout_part_two = get_sorted_layout_part_two(layout.clone(), instructions.clone());

    display_top(sorted_layout_part_one.as_mut());
    display_top(sorted_layout_part_two.as_mut());
}

fn get_data_from_input() -> (Vec<Vec<char>>, Vec<String>) {
    let file_content = fs::read_to_string("src/input.txt")
        .expect("file ./src/input.txt should exist");

    let (layout, instructions) = file_content.split_once("\n\n")
        .expect("input file should have input with valid format");

    let mut parsed = layout.split('\n')
        .collect::<Vec<&str>>();

    parsed.reverse();

    let drained = parsed.drain(1..).collect::<Vec<&str>>();

    let mut layout_parsed = Vec::<Vec<char>>::new();

    for value in drained {
        let mut character_index = 1;

        while character_index <= value.len() {
            let char = value.chars().nth(character_index)
                .expect("Valid character expected");

            let column = (character_index - 1) / 4;

            character_index += 4;

            if layout_parsed.len() < column + 1 {
                layout_parsed.push(Vec::<char>::new());
            }

            if char.is_ascii_whitespace() {
                continue;
            }

            layout_parsed[column].push(char);
        }
    }

    let parsed_instructions = Vec::from_iter(instructions.split('\n').map(String::from));

    return (layout_parsed, parsed_instructions);
}

fn get_sorted_layout_part_one(mut layout: Vec<Vec<char>>, instructions: Vec<String>) -> Vec<Vec<char>> {
    let mut sorted_layout = layout.clone();

    for instruction in instructions.into_iter() {
        let parts: Vec<&str> = instruction.split(' ').collect();

        let quantity: usize = parts[1].parse().unwrap();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();

        for _ in 0..quantity {
            let letter = layout[from - 1].pop().expect("Should return a valid &str");
            layout[to - 1].push(letter);
        }
    }

    return sorted_layout;
}

fn get_sorted_layout_part_two(mut layout: Vec<Vec<char>>, instructions: Vec<String>) -> Vec<Vec<char>> {
    let mut sorted_layout = layout.clone();

    for instruction in instructions.into_iter() {
        let parts: Vec<&str> = instruction.split(' ').collect();

        let quantity: usize = parts[1].parse().unwrap();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();

        let mut temp_vec = Vec::<char>::new();

        for _ in 0..quantity {
            if sorted_layout[from - 1].is_empty() {
                continue;
            }

            let letter = sorted_layout[from - 1].pop().expect("Should return a valid &str");

            temp_vec.insert(0, letter);
        }

        sorted_layout[to - 1].extend(temp_vec);
    }

    return sorted_layout;
}

fn display_top(layout: &mut Vec<Vec<char>>) {
    let mut top = Vec::<char>::new();

    for mut column in layout {
        if column.is_empty() {
            continue;
        }

        let top_letter = column.pop().expect("Expect column to have at least one character");

        top.push(top_letter);
    }

    println!("top values: {:?}", top.into_iter().collect::<String>())
}