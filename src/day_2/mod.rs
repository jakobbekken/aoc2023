mod part_1;
mod part_2;

pub fn solve_part(part: usize, input: &String) {
    match part {
        1 => part_1::solve(input),
        2 => part_2::solve(input),
        _ => println!("Invalid part number!"),
    }
}
