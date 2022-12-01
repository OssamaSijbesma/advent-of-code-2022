#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut max_sum:i32 = 0;
    let mut cur_sum:i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            cur_sum = 0;
        } else {
            cur_sum += line.parse::<i32>().unwrap();
        }

        if cur_sum > max_sum {
            max_sum = cur_sum;
        }
    }
    
    return max_sum;
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut big3:[i32;3] = [0; 3];
    let mut cur_sum:i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            cur_sum = 0;
        } else {
            cur_sum += line.parse::<i32>().unwrap();
        }

        let mut i = 0;
        while i < 3 {
            if cur_sum > big3[i] {
                let old_sum:i32 = big3[i];
                big3[i] = cur_sum;
                cur_sum = old_sum;
            }
            i += 1;
        } 
    }
    return big3.iter().sum();
}
