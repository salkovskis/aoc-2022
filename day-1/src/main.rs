use std::{fs};

fn read_input() -> Vec<String> {
    let file_path = "./src/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return Vec::from_iter(contents.split("\n").map(String::from));
}

fn main() {
    let list_of_calories = read_input();

    let mut list_of_elf_calories = parse_and_sort_list(list_of_calories);

    let most_calories = list_of_elf_calories.get(0).unwrap();
    println!("most calories: {most_calories}");

    let top_three = most_calories + list_of_elf_calories.get(1).unwrap() + list_of_elf_calories.get(2).unwrap();

    println!("combined of top three: {top_three}");
}

fn parse_and_sort_list(list_of_calories: Vec<String>) -> Vec<i32> {

    let mut list_of_elf_calories = Vec::<i32>::new();

    let mut calories_total = 0;

    for item in &list_of_calories {
        if item.eq("") {
            list_of_elf_calories.push(calories_total);

            calories_total = 0;

            continue;
        }

        calories_total += item.parse::<i32>().unwrap();
    }

    list_of_elf_calories.sort();
    list_of_elf_calories.reverse();

    return list_of_elf_calories;
}

fn get_present_for(name: String) -> Option<String> {
    if name.eq("Marta") {
        return Some("Buča".to_string())
    }

    if name.eq("Miķelis") {
        return Some("Mjauuu".to_string())
    }

   readline::readline(">")

    None
}