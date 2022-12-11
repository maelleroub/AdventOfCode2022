fn main() {
    let input = include_str!("day1_input");
    let lines : Vec<&str> = input.lines().collect();
    let mut calories: Vec<i32> = Vec::new();
    let mut cur = 0;

    for line in lines {
        if line.is_empty()
        {
            calories.push(cur);
            cur = 0;
        }
        else {
            cur += line.parse::<i32>().unwrap();
        }
    }

    calories.sort_by(|a, b| b.cmp(a));
    println!("{}", calories[0] + calories[1] + calories[2]);
}
