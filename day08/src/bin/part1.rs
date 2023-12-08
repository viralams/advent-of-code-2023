use std::collections::BTreeMap;

// const DATA: &str = include_str!("test.txt");
// const DATA: &str = include_str!("another_test.txt");
const DATA: &str = include_str!("input.txt");

fn main() {
    let instructions = DATA.lines().nth(0).unwrap().chars();
    println!("instructions : {instructions:?}");

    let mut my_map = BTreeMap::new();
    for line in DATA.lines().skip(2) {
        let line_split = line.split_once("=").expect("Boom here");
        let mut tuple_split = line_split.1.trim_right_matches(')')[2..]
            .split(',')
            .map(|s| s.trim().to_string());

        my_map.insert(
            line_split.0.trim(),
            (tuple_split.next().unwrap(), tuple_split.next().unwrap()),
        );
    }

    let mut steps: i128 = 0;
    let mut current = "AAA";
    for dir in instructions.cycle() {
        println!("{dir} in step: {steps}");
        let location = my_map.get(current).unwrap();
        println!("{location:?} location currently");
        match dir {
            'L' => current = location.0.as_str(),
            'R' => current = location.1.as_str(),
            _ => break,
        }

        steps += 1;

        if current == "ZZZ" {
            break;
        }
    }

    println!("Total taken steps Part 1: {steps:?}")
}
