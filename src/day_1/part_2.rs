pub fn solve(input: &String) {
    let mut sum = 0;

    for line in input.lines() {
        let line_1 = line.replace("one", "o1ne");
        let line_2 = line_1.replace("two", "t2wo");
        let line_3 = line_2.replace("three", "th3ree");
        let line_4 = line_3.replace("four", "fo4ur");
        let line_5 = line_4.replace("five", "fi5ve");
        let line_6 = line_5.replace("six", "si6x");
        let line_7 = line_6.replace("seven", "sev7en");
        let line_8 = line_7.replace("eight", "eigh8t");
        let line_9 = line_8.replace("nine", "ni9ne");

        let digits: Vec<u32> = line_9
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        sum += digits.first().unwrap() * 10 + digits.last().unwrap();
    }

    println!("{}", sum);
}
