use std::fs;

pub fn solve() {
    println!("Solution for 2023-12-04 is not working as intended yet.");

    const FILE_PATH: &str = "./files/y2023/d04/example1.txt";

    let contents: String =
        fs::read_to_string(FILE_PATH).expect("Should have been able to read the file.");

    let mut matches_per_card: Vec<i32> = vec![];
    let mut total_card_score = 0;
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
        
        let mut n_matches = 0;
        let mut card_score = 0;
        for n in play_numbers {
            if winning_numbers.contains(&n) {
                n_matches += 1;
                if card_score == 0 { card_score = 1; } else { card_score *= 2; }
            }
        }
        total_card_score += card_score;
        matches_per_card.push(n_matches);
    }
    // part 1
    dbg!(total_card_score);

    // part 2 unfinished
    // ideas: take the iter of matches (y), zip with a list of "n_duplicates" (n, initialized as all ones)
    // then for each value l[x] = (y, n) add the current value of n to l[x+1..x+y]
    // ex.:
    // 4 2 2 1 0 0
    // ------------
    // 1 1 1 1 1 1
    // > 1 1 1 1
    // x > 2 2
    // x x > 4 4
    // x x x > 8
    // x x x x >
    // x x x x x >
    // 1+2+4+8+14+1 = 30

    println!("{:?}", matches_per_card);
    let total_cards_won = get_scratchcards(matches_per_card.iter());
    dbg!(total_cards_won);
}


fn get_scratchcards(mut list: std::slice::Iter<i32>) -> i32 {
    // not working as intended.
    // probably replace this completely with the function idea detailed in the part 2 ideas.
    if let Some(&n_cards) = list.next() {
        println!("{} -- {:?}", n_cards, list);

        let mut total_cards_won = 1;

        if n_cards <= 0 {
            println!("I didn't win any more cards! {}", total_cards_won);
            return total_cards_won;
        };
   
        println!("Recursing a {}...", n_cards);
        for _ in 1..=n_cards {
            total_cards_won += get_scratchcards(list.clone());
        }
        println!("...done. Pulled {} from a {}.", total_cards_won, n_cards);
        return total_cards_won;
    }
    println!("I'm out of cards!");
    return 0;
}