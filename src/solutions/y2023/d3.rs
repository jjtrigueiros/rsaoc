use std::fs;

pub fn solve() {
    println!("Solution for 2023-12-03 is not working as intended yet.");
    const FILE_PATH: &str = "./files/y2023/d03/example1.txt";

    let contents: String =
        fs::read_to_string(FILE_PATH).expect("Should have been able to read the file.");

    
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        schematic.push(line.chars().collect::<Vec<char>>());
    }
 
    let mut total = 0;
    for (m, row) in schematic.iter().enumerate() {
        for (n, item) in row.iter().enumerate() {
            if item.is_digit(10) && row.get(n-1).map_or(false, |prev| !prev.is_digit(10)) {
                
                // check surroundings
                let mut subtotal = 0;
                let surr_m = [m-1, m, m+1];
                let surr_n = [n-1, n, n+1];
                for mm in surr_m {
                    for nn in surr_n {
                        subtotal += schematic
                            .get(mm)
                            .and_then(|row| row.get(nn))
                            .and_then(|val| val.to_digit(10))
                            .unwrap_or_default();
                    }
                }
                dbg!(subtotal);
                total += subtotal;
            }
        }
    }
    dbg!(total);
}
