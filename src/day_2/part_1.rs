struct Game {
    red: Vec<u8>,
    blue: Vec<u8>,
    green: Vec<u8>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            red: Vec::new(),
            blue: Vec::new(),
            green: Vec::new(),
        }
    }
}

pub fn solve(input: &String) {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut seg: Vec<&str> = line.split([':', ';']).collect();
        let num: u32 = seg[0][5..].parse().unwrap();
        seg.remove(0);
        let mut game = Game::new();

        for field in &seg {
            for color in field.split(',') {
                let part: Vec<&str> = color.split_whitespace().collect();
                match part[1] {
                    "red" => game.red.push(part[0].parse().unwrap()),
                    "blue" => game.blue.push(part[0].parse().unwrap()),
                    "green" => game.green.push(part[0].parse().unwrap()),
                    _ => (),
                }
            }
        }

        if *game.red.iter().max().unwrap() <= 12
            && *game.green.iter().max().unwrap() <= 13
            && *game.blue.iter().max().unwrap() <= 14
        {
            sum += num
        }
    }
    println!("Solution: {}", sum);
}
