pub fn solve(input: &String) {
    let mut sum = 0;

    for line in input.lines() {
        let digits: Vec<u32> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        sum += digits.first().unwrap() * 10 + digits.last().unwrap();
    }

    println!("Solution: {}", sum);
}
