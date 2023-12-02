use std::fs;

mod day_1;
mod day_2;
mod day_3;

fn read_input(day: usize) -> String {
    let file_path = format!("input/day_{}.txt", day);
    fs::read_to_string(file_path).expect("Didn't find goofy input!")
}

fn main() {
    let day = 3;
    let part = 1;

    let input = read_input(day);

    println!("\n-={{ DAY: {}, PART: {} }}=-\n", day, part);

    match day {
        1 => day_1::solve_part(part, &input),
        2 => day_2::solve_part(part, &input),
        3 => day_3::solve_part(part, &input),
        _ => println!("Day not found, goofy!"),
    }

    println!("");
}
