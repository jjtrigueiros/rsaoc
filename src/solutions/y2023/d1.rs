use std::fs;

pub fn solve() {
    const FILE_PATH: &str = "./files/y2023/d01/input.txt";
    const PART_TWO: bool = true;

    let contents: String =
        fs::read_to_string(FILE_PATH).expect("Should have been able to read the file.");

    let mut total: i32 = 0;
    for line in contents.lines() {
        if PART_TWO {
            total += get_first_and_last(&dumb_replace(&line));
        } else {
            total += get_first_and_last(&line);
        }
    }

    dbg!(total);
}

fn get_first_and_last(s: &str) -> i32 {
    let integers: Vec<i32> = s
        .chars()
        .filter(|&c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    if integers.len() == 0 {
        return 0;
    }

    [integers[0], integers[integers.len() - 1]]
        .iter()
        .map(|&digit| digit.to_string())
        .collect::<String>()
        .parse()
        .unwrap_or(0)
}

// NOTE - Results for part 2 can overlap (ex.: eightwone => 821).

fn dumb_replace(string_value: &str) -> String {
    String::from(string_value)
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}
