use std::env;

mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <day>", args[0])
    }

    const YEAR: i32 = 2023; // always 2023 for now
    let day: i32 = args[1].parse().expect("Invalid day");
    assert!(day <= 31);

    run_solution(YEAR, day);
}

fn run_solution(year: i32, day: i32) {
    match (year, day) {
        (2023, 1) => solutions::y2023::d1::solve(),
        (2023, 2) => solutions::y2023::d2::solve(),
        (2023, 3) => solutions::y2023::d3::solve(),
        (2023, 4) => solutions::y2023::d4::solve(),
        (2023, 5) => solutions::y2023::d5::solve(),
        (2023, 6) => solutions::y2023::d6::solve(),
        (2023, 7) => solutions::y2023::d7::solve(),
        (2023, 8) => solutions::y2023::d8::solve(),
        (2023, 9) => solutions::y2023::d9::solve(),
        (2023, 10) => solutions::y2023::d10::solve(),
        (2023, 11) => solutions::y2023::d11::solve(),
        (2023, 12) => solutions::y2023::d12::solve(),
        (2023, 13) => solutions::y2023::d13::solve(),
        (2023, 14) => solutions::y2023::d14::solve(),
        (2023, 15) => solutions::y2023::d15::solve(),
        (2023, 16) => solutions::y2023::d16::solve(),
        (2023, 17) => solutions::y2023::d17::solve(),
        (2023, 18) => solutions::y2023::d18::solve(),
        (2023, 19) => solutions::y2023::d19::solve(),
        (2023, 20) => solutions::y2023::d20::solve(),
        (2023, 21) => solutions::y2023::d21::solve(),
        (2023, 22) => solutions::y2023::d22::solve(),
        (2023, 23) => solutions::y2023::d23::solve(),
        (2023, 24) => solutions::y2023::d24::solve(),
        _ => println!("No solution found for {}-12-{}", year, day),
    }
}
