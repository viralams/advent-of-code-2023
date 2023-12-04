use std::collections::HashSet;

// const DATA: &str = include_str!("test.txt");
const DATA: &str = include_str!("input.txt");

fn main() {
    let mut card_counts = vec![1u32];
    let mut count = 0;

    for (index, line) in DATA.lines().enumerate() {
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
        let common_count = my_numbers_set.intersection(&winning_numbers_set).count();
        let end = index + common_count + 1;

        if end > card_counts.len() {
            card_counts.resize(end, 1);
        }

        for i in index + 1..end {
            card_counts[i] += card_counts[index];
        }

        count += card_counts[index];
    }

    println!("{:?}", count);
}
