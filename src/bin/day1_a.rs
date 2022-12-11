fn main() {
    let input = include_str!("day1_input");
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
            cur += line.parse::<i32>().unwrap();
        }
    }

    println!("{}", max);
}
