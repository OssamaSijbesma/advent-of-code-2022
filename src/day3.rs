#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let mut score: i32 = 0;

    for line in input.lines() {
        let chunks: (&str, &str) = line.split_at(line.chars().count()/2);
        // println!("{:?}", chunks);
        for c in chunks.0.chars() {
            if chunks.1.contains(c) {
                // println!("{}", c);
                let ascii_char = c as i32;
                let correction = if ascii_char >= 97 { 96 } else { 38 };
                score += (ascii_char - correction) as i32;
                break;
            }
        }
    }
    
    return score;
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let mut score: i32 = 0;

    for chunk in input.lines().collect::<Vec<&str>>().chunks(3) {
        // println!("{:?}", chunk);
        for c in chunk[0].chars() {
            if chunk[1].contains(c) && chunk[2].contains(c) {
                // println!("{}", c);
                let ascii_char = c as i32;
                let correction = if ascii_char >= 97 { 96 } else { 38 };
                score += (ascii_char - correction) as i32;
                break;
            }
        }
    }
    
    return score;
}