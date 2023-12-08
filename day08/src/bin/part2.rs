use num::integer::lcm;
use std::collections::BTreeMap;

// const DATA: &str = include_str!("test2.txt");
// const DATA: &str = include_str!("another_test.txt");
const DATA: &str = include_str!("input.txt");

fn main() {
    let instructions = DATA.lines().nth(0).unwrap().chars();
    // println!("instructions : {instructions:?}");

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

    let start_locations: Vec<_> = my_map
        .keys()
        .filter(|n| n.ends_with("A"))
        .map(|n| n.to_owned())
        .collect();
    println!("Starting locations {start_locations:?}");

    let mut steps = Vec::new();

    for curr in start_locations {
        let mut curr_steps: u64 = 0;
        let mut curr_loc = curr;
        for choice in instructions.clone().cycle() {
            let location = my_map.get(curr_loc).unwrap();
            curr_steps += 1;
            match choice {
                'L' => curr_loc = location.0.as_str(),
                'R' => curr_loc = location.1.as_str(),
                _ => panic!("invalid direction {curr}"),
            }

            if curr_loc.ends_with("Z") {
                break;
            }
        }
        steps.push(curr_steps);
    }

    let final_lcm = calculate_lcm(steps.as_slice());

    println!("Steps Part 2: {final_lcm}")
}

fn calculate_lcm(numbers: &[u64]) -> u64 {
    let mut result = 1;

    for &num in numbers {
        result = lcm(result, num);
    }

    result
}
