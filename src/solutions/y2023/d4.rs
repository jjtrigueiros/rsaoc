use std::fs;

pub fn solve() {
    const FILE_PATH: &str = "./files/y2023/d04/input.txt";

    let contents: String =
        fs::read_to_string(FILE_PATH).expect("Should have been able to read the file.");

    let mut total_score = 0;
    for line in contents.lines() {

        let mut split = line.split(":");

        let game_id: i32 = split
            .next()
            .unwrap()
            .chars()
            .skip_while(|c| !c.is_digit(10))
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        dbg!(game_id);

        let mut body = split
            .next()
            .unwrap()
            .split(" | ");

        let winning_numbers: Vec<i32> = body
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|item| item.parse().ok())
            .collect();

        let play_numbers: Vec<i32> = body
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|item| item.parse().ok())
            .collect();
        
        let mut card_score = 0;
        for n in play_numbers {
            if winning_numbers.contains(&n) {
                if card_score == 0 { card_score = 1; } else { card_score *= 2; }
            }
        }
        total_score += card_score;
    }
    dbg!(total_score);
}
