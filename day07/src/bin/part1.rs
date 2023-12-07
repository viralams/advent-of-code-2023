use std::collections::BinaryHeap;

// const DATA: &str = include_str!("test.txt");
const DATA: &str = include_str!("input.txt");

fn main() {
    let updated_hands = DATA
        .lines()
        .map(|line| {
            // Easy for comparison instead of structs
            line.replace("A", "Z")
                .replace("K", "Y")
                .replace("Q", "X")
                .replace("J", "W")
                .replace("T", "V")
        }) // My awesome way of splitting the str provided
        .map(|line| (line[0..5].to_string(), line[6..].parse::<usize>().unwrap()));

    let counted_hands = updated_hands.map(|(hand, bid)| {
        (
            // Frequency counted here
            hand.chars()
                .map(|c| (hand.chars().filter(|c1| c == *c1).count(), c))
                .max()
                .unwrap(),
            hand,
            bid,
        )
    });

    let map_to_score_tup = counted_hands.map(|(max1, hand, bid)| {
        (
            // Ugly way of capturing a random score based on frequency and the char value 100 times the number of cards and then count the card values to make a score
            (max1.0 * 100
                + hand
                    .chars()
                    .map(|c| {
                        hand.chars()
                            .filter(|c1| ((c == *c1) && (c != max1.1)))
                            .count()
                    })
                    .max()
                    .unwrap()),
            hand,
            bid,
        )
    });

    let make_heap_and_sum: usize = map_to_score_tup
        .collect::<BinaryHeap<_>>() // Heaps so I dont have to sort
        .into_sorted_vec() // Capturing it as a vec
        .iter()
        .enumerate()
        .map(|(index, (_score, _hand, bid))| (index + 1) * bid) // we need the index so we can calculate the score
        .sum();

    println!("Calculated sum for part 1: {make_heap_and_sum}");
}
