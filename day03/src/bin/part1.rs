use itertools::Itertools;
use std::collections::BTreeMap;

const DATA: &str = include_str!("input.txt");
// const DATA: &str = include_str!("test.txt");
#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}

fn main() {
    // Make a map with co-ordinates and corresponding values
    let my_btree_map = DATA
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, character)| {
                (
                    (y as i32, x as i32), // This is our co-ordinate in the matrix
                    // We match on what the character is and map it to the appropriate Value enum type
                    match character {
                        '.' => Value::Empty,
                        c if c.is_ascii_digit() => {
                            Value::Number(c.to_digit(10).expect("Should be a valid number!"))
                        }
                        c => Value::Symbol(c),
                    },
                )
            })
        })
        .collect::<BTreeMap<(i32, i32), Value>>(); // Collecting it as a map of (usize, usize): position and the corresponding Value
                                                   // println!("{:?}", my_btree_map);

    // Make the numbers in co-ordinates as one
    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in my_btree_map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last =
                                    numbers.iter_mut().last().expect("value should exist here");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![((*x, *y), *num)]);
                            }
                        }
                        None => unimplemented!("God help if we get here"),
                    }
                }
                None => {
                    numbers.push(vec![((*x, *y), *num)]);
                }
            }
        }
    }

    // println!("{:?}", numbers);

    // map what is around with static positions
    let mut total = 0;
    for num_list in numbers {
        // (x,y)
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let num_positions: Vec<(i32, i32)> = num_list.iter().map(|((y, x), _)| (*x, *y)).collect();
        let pos_to_check: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions.iter().map(|outer_pos| {
                    // outer_pos.x + pos.x, .y + .y
                    (outer_pos.0 + pos.1, outer_pos.1 + pos.0)
                })
            })
            .unique() // Had to research and add itertools to make my life easy
            .filter(|num| !num_positions.contains(num))
            .collect();

        let is_part_number = pos_to_check.iter().any(|pos| {
            let value = my_btree_map.get(pos);
            #[allow(clippy::match_like_matches_macro)]
            if let Some(Value::Symbol(_)) = value {
                true
            } else {
                false
            }
        });

        if is_part_number {
            total += num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        }
    }

    println!("Part 1 total: {:?}", total);
}
