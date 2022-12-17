fn part_a(input: &str) -> u32 {
    let lines : Vec<&str> = input.lines().collect();
    let mut max = 0;
    let mut cur = 0;

    for line in lines {
        if line.is_empty() {
            if cur > max {
                max = cur;
            }
            cur = 0;
        }
        else {
            cur += line.parse::<u32>().unwrap();
        }
    }
    max
}

fn part_b(input: &str) -> u32 {
    let lines : Vec<&str> = input.lines().collect();
    let mut calories: Vec<u32> = Vec::new();
    let mut cur = 0;

    for line in lines {
        if line.is_empty()
        {
            calories.push(cur);
            cur = 0;
        }
        else {
            cur += line.parse::<u32>().unwrap();
        }
    }

    calories.sort_by(|a, b| b.cmp(a));
    calories[0] + calories[1] + calories[2]
}

fn main() {
    let input = include_str!("day1_input");
    println!("Part A: {}", part_a(input));
    println!("Part B: {}", part_b(input));
}
