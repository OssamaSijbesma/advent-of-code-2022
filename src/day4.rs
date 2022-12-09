#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let mut score: i32 = 0;

    for line in input.lines() {
        let chunks: Vec<i32> = line.split(&['-', ',']).map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if chunks[0] <= chunks[2] && chunks[1] >= chunks[3] || chunks[2] <= chunks[0] && chunks[3] >= chunks[1] {
            //println!("{:?}", chunks);
            score += 1;
        }
    }
    
    return score;
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let mut score: i32 = 0;

    for line in input.lines() {
        let chunks: Vec<i32> = line.split(&['-', ',']).map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if chunks[2] >= chunks[0] && chunks[2] <= chunks[1] 
        || chunks[3] >= chunks[0] && chunks[3] <= chunks[1] 
        || chunks[2] <= chunks[0] && chunks[3] >= chunks[1] {
            //println!("{:?}", chunks);
            score += 1;
        }
    }
    
    return score;
}