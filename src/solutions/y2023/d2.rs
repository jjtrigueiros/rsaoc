use std::fs;

pub fn solve() {
    const FILE_PATH: &str = "./files/y2023/d02/input.txt";

    let contents: String =
        fs::read_to_string(FILE_PATH).expect("Should have been able to read the file.");
    
    let mut solution_part1 = 0;
    let mut solution_part2 = 0;

    for line in contents.lines() {
        if line.is_empty() { continue; }

        let game_id: i32 = line.split(": ").next().unwrap()
            .chars()
            .skip_while(|c| !c.is_digit(10))
            .take_while(|c| c.is_digit(10))
            .collect::<String>().parse().unwrap();
        
        let rounds = line.split(": ").last().unwrap().split("; ");

        let mut max_red_value = 0;
        let mut max_green_value = 0;
        let mut max_blue_value = 0;

        
        for round in rounds {
            let color_values = round.split(", ");
            for color_value in color_values {
                let color_number: i32 = color_value.chars()
                    .take_while(|c| c.is_digit(10)).collect::<String>().parse().unwrap();
                if color_value.contains("red") {
                    max_red_value = max_red_value.max(color_number)
                } else if color_value.contains("blue") {
                    max_blue_value = max_blue_value.max(color_number)
                } else if color_value.contains("green") {
                    max_green_value = max_green_value.max(color_number)
                }
            }
            
        }

        // calculate if game is valid for part 1 criteria (12 red, 13 green, 14 blue in bag)
        if (max_red_value <= 12) & (max_green_value <= 13) & (max_blue_value <= 14) {
            solution_part1 += game_id;
        }

        // calculate power of game for part 2 (minimum number of dice in bag = max number found in game)
        solution_part2 = solution_part2 + max_red_value * max_green_value * max_blue_value;
    }
    dbg!(solution_part1);
    dbg!(solution_part2);
}
