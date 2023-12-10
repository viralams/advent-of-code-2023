use std::collections::{HashMap, HashSet};

// const DATA: &str = include_str!("test.txt");
const DATA: &str = include_str!("input.txt");

type Direction = (i32, i32);
type GridPosition = (i32, i32);
type Pipe = [Direction; 2]; // Takes in possible directions

#[derive(Debug)]
struct Grid {
    starting_position: GridPosition,
    pipes: HashMap<GridPosition, Pipe>,
    bottom_right: GridPosition, // Had to add this as a part of the struct to capture the end positions and limits of the grid
}

fn main() {
    let mut my_grid = Grid {
        // Beginning grid set up
        starting_position: (0, 0),
        pipes: HashMap::new(),
        bottom_right: (
            DATA.lines().next().unwrap().chars().count() as i32 - 1,
            DATA.lines().count() as i32 - 1,
        ),
    };

    DATA.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, tile)| match tile {
            'S' => my_grid.starting_position = (x as i32, y as i32),
            '.' => (),
            _ => {
                my_grid.pipes.insert(
                    (x as i32, y as i32),
                    match tile {
                        // Mapping possible directions to each pipe in the grid
                        '|' => [(0, -1), (0, 1)],
                        '-' => [(-1, 0), (1, 0)],
                        'L' => [(0, -1), (1, 0)],
                        'J' => [(0, -1), (-1, 0)],
                        '7' => [(-1, 0), (0, 1)],
                        'F' => [(1, 0), (0, 1)],
                        _ => unreachable!(),
                    },
                );
            }
        })
    });

    println!("{:?}", my_grid);

    let mut current_position = my_grid.starting_position;
    let mut main_lewp = HashSet::from([current_position]);

    'main_loop: loop {
        if let Some(pipe) = my_grid
            .pipes
            .get(&current_position)
            .or(Some(&starting_pipe(&my_grid)))
        {
            for direction in pipe {
                let adjacent_pipe_position = (
                    current_position.0 + direction.0,
                    current_position.1 + direction.1,
                );

                if !main_lewp.contains(&adjacent_pipe_position) {
                    current_position = adjacent_pipe_position;
                    main_lewp.insert(current_position); // Add position to the HashSet
                    continue 'main_loop;
                }
            }

            break 'main_loop;
        }
    }

    println!(
        "Farthest point is the half the length {:?}",
        main_lewp.len() / 2
    );

    let mut inside_count = 0;
    let starting_pipe = starting_pipe(&my_grid);

    for y in 0..=my_grid.bottom_right.1 {
        let mut outside = true;

        for x in 0..=my_grid.bottom_right.0 {
            if let Some(position) = main_lewp.get(&(x, y)) {
                let directions = my_grid.pipes.get(position).unwrap_or(&starting_pipe);

                if directions.contains(&(0, 1)) {
                    outside = !outside;
                }
            } else if !outside {
                inside_count += 1;
            }
        }
    }

    println!("Inside count : {inside_count:?}");
}

fn starting_pipe(grid: &Grid) -> Pipe {
    let starting_position = grid.starting_position;
    let mut starting_directions = Vec::with_capacity(2);

    // All possible directions from the starting position
    for (x, y) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let pipe_position = (starting_position.0 + x, starting_position.1 + y);

        if let Some(pipe) = grid.pipes.get(&pipe_position) {
            for direction in pipe.iter() {
                if (pipe_position.0 + direction.0, pipe_position.1 + direction.1)
                    == starting_position
                {
                    starting_directions.push((-direction.0, -direction.1));
                    break;
                }
            }

            if starting_directions.len() == 2 {
                break;
            }
        }
    }

    starting_directions.try_into().unwrap() // Please dont break here
}
