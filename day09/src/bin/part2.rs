use std::collections::HashSet;

// const DATA: &str = include_str!("test.txt");
const DATA: &str = include_str!("input.txt");

fn main() {
    let mut total_diff = 0;

    for line in DATA.lines() {
        // Parse into vec
        let my_vec: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("You shall not Paaarse!!"))
            .collect();
        // println!("My Vec is : {my_vec:?}");

        // Making a grid of vecs
        let mut grid = vec![my_vec.clone()];
        while grid.last().unwrap().iter().collect::<HashSet<_>>().len() > 1 {
            let next_line: Vec<i32> = grid
                .last()
                .unwrap()
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            grid.push(next_line);
        }
        // println!("My Grid is : {grid:?}");

        // Get diff of elements and mutate it
        let mut difference = *grid.last().unwrap().first().unwrap();
        for i in (0..grid.len() - 1).rev() {
            difference = *grid[i].first().unwrap() - difference;
        }
        total_diff += difference;
        println!("Calculated diff for this line : {difference:?}");
    }

    println!("Total Difference is (Part2) : {total_diff:?}");
}
