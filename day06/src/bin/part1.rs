fn main() {
    // let data: &str = include_str!("test.txt");
    let data: &str = include_str!("input.txt");
    let mut data_lines = data.lines();

    let times: Vec<i64> = data_lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let distances: Vec<i64> = data_lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{:?}", times);
    println!("{:?}", distances);

    let mut output = 1;
    for (time, high_score) in times.iter().zip(distances.iter()) {
        let mut count = 0;
        for hold_time in 1..*time {
            let distance_traveled = (time - hold_time) * hold_time;
            if distance_traveled > *high_score {
                count += 1;
            }
        }
        output *= count;
    }
    println!("Part 1: Number of ways(product) =  {output}");
}
