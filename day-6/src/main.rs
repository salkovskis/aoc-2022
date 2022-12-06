use std::collections::HashSet;
use std::fs;

fn find_buffer_end(input: &String, length_of_character_group: usize) -> usize {
    let mut end_position_of_first_buffer = 0;

    let mut previous_characters = Vec::<char>::new();

    for (index, character) in input.chars().enumerate() {
        if (previous_characters).len() < length_of_character_group - 1 {
            previous_characters.push(character);

            continue;
        }

        let mut temp_part_one = Vec::<char>::new();

        temp_part_one.push(character);
        temp_part_one.append(previous_characters.clone().as_mut());

        let all_unique = temp_part_one.iter()
            .collect::<HashSet<_>>().len() == length_of_character_group;

        if all_unique {
            end_position_of_first_buffer = index;

            break;
        }

        previous_characters.remove(0);
        previous_characters.push(character);
    }

    return end_position_of_first_buffer + 1;
}

fn main() {
    let file_path = "./src/input.txt";

    let input = fs::read_to_string(file_path)
        .expect(format!("Expect file {} to exist", file_path).as_str());

    let length_of_first_buffer = 4;
    let location_part_one = find_buffer_end(&input, length_of_first_buffer);

    println!(
        "ending position of first buffer of length {} is {}",
        length_of_first_buffer,
        location_part_one
    );

    let length_of_second_buffer = 14;
    let location_part_twos = find_buffer_end(&input, length_of_second_buffer);

    println!(
        "ending position of first buffer of length {} is {}",
        length_of_second_buffer,
        location_part_twos
    );
}
