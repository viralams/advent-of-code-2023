use std::collections::BinaryHeap;

// const DATA: &str = include_str!("test.txt");
const DATA: &str = include_str!("input.txt");

fn main() {
    let my_sum: usize = DATA
        .lines()
        .map(|line| {
            // Easy for comparison instead of structs
            line.replace("A", "Z")
                .replace("K", "Y")
                .replace("Q", "X")
                .replace("J", "0") // Replacing J with a lower value for char
                .replace("T", "V")
        }) // My awesome way of splitting the str provided
        // I dont want to use structs
        .map(|line| (line[0..5].to_string(), line[6..].parse::<usize>().unwrap()))
        // Count the most frequently occurring card
        .map(|(hand, bid)| {
            (
                hand.chars()
                    .map(|c| (hand.chars().filter(|c1| c == *c1).count(), c))
                    .max()
                    .unwrap(),
                hand,
                bid,
            )
        })
        // Card frequency calculated
        .map(|(max1, hand, bid)| {
            (
                max1,
                (hand
                    .chars()
                    .map(|c| {
                        (
                            hand.chars()
                                .filter(|c1| ((c == *c1) && (c != max1.1)))
                                .count(),
                            c,
                        )
                    })
                    .max()
                    .unwrap()),
                hand,
                bid,
            )
        })
        // Copy hand to hand2, replace the joker with the most frequently occurring other card.
        // What if the jokers are the most frequent condition
        .map(|(max1, max2, hand, bid)| {
            (
                match max1.1 {
                    '0' => hand.replace("0", &max2.1.to_string()),
                    c => hand.replace("0", &c.to_string()),
                },
                hand,
                bid,
            )
        })
        // Now recalculate the most frequently occurring card using hand2
        .map(|(hand2, hand, bid)| {
            (
                hand2
                    .chars()
                    .map(|c| (hand2.chars().filter(|c1| c == *c1).count(), c))
                    .max()
                    .unwrap(),
                hand,
                hand2,
                bid,
            )
        })
        .map(|(max1, hand, hand2, bid)| {
            (
                max1.0 * 10
                    + hand2
                        .chars()
                        .map(|c| {
                            hand2
                                .chars()
                                .filter(|c1| ((c == *c1) && (c != max1.1)))
                                .count()
                        })
                        .max()
                        .unwrap(),
                hand,
                bid,
            )
        })
        .collect::<BinaryHeap<_>>() // Heaps are cool
        .into_sorted_vec()
        .iter()
        .enumerate()
        .map(|(index, (_score, _hand, bid))| (index + 1) * bid)
        .sum();

    println!("Calculated sum for part 2: {my_sum}");
}
