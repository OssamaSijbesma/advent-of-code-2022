pub fn transform_to_numbers(line: &str) -> Vec<i32> {
    return line.split_whitespace().into_iter().map(|l| {
            if l == "A" || l == "X" { return 1; } 
            else if l == "B" || l == "Y" { return 2; }
            return 3
        }).rev().collect();
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut score: i32 = 0;

    for line in input.lines() {
        let chunks: Vec<i32> = transform_to_numbers(line);

        if chunks[0] == chunks[1] {
            score += 3;
        } else if (chunks[1] + 1) % 3 == chunks[0] % 3 {
            score += 6;
        }

        score += chunks[0]
    }
    
    return score;
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut score: i32 = 0;

    for line in input.lines() {
        let chunks: Vec<i32> = transform_to_numbers(line);

        if chunks[0] == 2 {
            score += 3;
            score += chunks[1];
        } else if chunks[0] == 3 {
            score += 6;
            score += if (chunks[1] + 1) % 3 == 0 { 3 } else { (chunks[1] + 1) % 3 };
        } else { 
            score += if (chunks[1] - 1) % 3 == 0 { 3 } else { (chunks[1] - 1) % 3 };
        }
    }
    
    return score;
}
