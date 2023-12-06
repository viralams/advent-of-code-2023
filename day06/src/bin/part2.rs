fn main() {
    // let data: &str = include_str!("test.txt");
    let data: &str = include_str!("input.txt");
    let mut data_lines = data.lines();

    let times: i64 = data_lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse()
        .expect("Unable to parse times");
    let times = vec![times];

    let distances: i64 = data_lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .trim()
        .replace(" ", "")
        .parse()
        .expect("Unable to parse distances");
    let distances = vec![distances];

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

    println!("Part2: Number of ways = {output}");
}
