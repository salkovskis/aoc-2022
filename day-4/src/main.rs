use std::fs;

fn read_input() -> Vec<String> {
    let file_path = "./src/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect(" Should have been able to read the file");

    return Vec::from_iter(contents.split("\n").map(String::from));
}

fn main() {
    let input = read_input();

    let mut par_one_count = 0;
    let mut par_two_count = 0;

    for line in input {
        let groups = line.split(',')
            .map(|item| item.split('-'))
            .map(|item| {
                let values = item.map(String::from)
                    .map(|a| a.to_string().parse().unwrap())
                    .collect::<Vec<i32>>();

                return (values.first().unwrap().clone(), values.last().unwrap().clone());
            })
            .collect::<Vec<(i32, i32)>>();

        let (first_start, first_end) = groups.first().unwrap();
        let (second_start, second_end) = groups.last().unwrap();

        let is_full_overlap = check_for_full_overlap(first_start, first_end, second_start, second_end);
        let is_partial_overlap = check_for_partial_overlap(first_start, first_end, second_start, second_end);

        if is_full_overlap {
            par_one_count += 1;
        }

        if is_full_overlap || is_partial_overlap {
            par_two_count += 1;
        }
    }

    println!("part 1. count {}, part 2. count {}", par_one_count, par_two_count);
}


fn check_for_full_overlap(first_start: &i32, first_end: &i32, second_start: &i32, second_end: &i32) -> bool {
    if first_start <= second_start && first_end >= second_end {
        return true;
    }

    if first_start >= second_start && first_end <= second_end {
        return true;
    }

    return false;
}

fn check_for_partial_overlap(first_start: &i32, first_end: &i32, second_start: &i32, second_end: &i32) -> bool {
    if first_start >= second_start && first_start <= second_end {
        return true;
    }

    if first_end >= second_start && first_end <= second_end {
        return true;
    }

    return false;
}