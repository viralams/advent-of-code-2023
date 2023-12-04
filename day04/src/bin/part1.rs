use std::collections::HashSet;

// const DATA: &str = include_str!("test.txt");
const DATA: &str = include_str!("input.txt");

fn main() {
    let mut total: u32 = 0;
    for line in DATA.lines() {
        // Split by  ": "
        let (_card_details, card_values) = line
            .split_once(": ")
            .expect("Cant find the delimiter to split here");
        // Split by  " | "
        let (winning_numbers_str, my_numbers_str) = card_values
            .split_once(" | ")
            .expect("Cant find the | delimiter to split here");
        // reach values and split by white spaces and place them into the set collector
        let winning_numbers_set: HashSet<String> = winning_numbers_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        // reach values and split by white spaces and place them into the set collector
        let my_numbers_set: HashSet<String> = my_numbers_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        // println!("{:?}", winning_numbers_set);
        // println!("{:?}", my_numbers_set);
        let common_count = my_numbers_set.intersection(&winning_numbers_set).count();
        println!("Common Count : {}", common_count);
        let my_points = get_points(common_count as u32);
        println!("My Points : {}", my_points);
        total = total + my_points;
    }

    println!("My total of winning: {}", total)
}

fn get_points(count: u32) -> u32 {
    return match count {
        0 => 0,
        1 => 1,
        _ => {
            let base: u32 = 2;
            base.pow(count - 1)
        }
    };
}
