use std::str::FromStr;

const DATA: &str = include_str!("input.txt");

type Bucket = [u32; 3]; // red , green , blue
pub struct Game {
    id: u32,
    buckets: Vec<Bucket>,
}

// Ugliest FromStr implementation on earth
impl FromStr for Game {
    type Err = std::convert::Infallible; // Big Boohoo. Cant convert stuff!

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, rest) = line.split_once(":").unwrap();
        Ok(Self {
            id: id.split_once(' ').unwrap().1.parse().unwrap(),
            buckets: rest
                .split("; ") // Split
                .map(|part| {
                    // each part is parsed in deep
                    let mut bucket = [0, 0, 0]; // making an empty bucket to set it later and return for each Game created
                    for (n, some_color) in part
                        .split(',') // Split for each set of cubes
                        .map(|mini_part| mini_part.trim().split_once(' ').unwrap())
                    {
                        bucket[some_color.as_bytes()[0] as usize % 3] = n.parse().unwrap();
                        // making the bucket
                    }
                    bucket // return the bucket to set for Game
                })
                .collect(),
        })
    }
}

fn part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|game| {
            game.buckets
                .iter()
                .all(|r| r[0] <= 12 && r[1] <= 13 && r[2] <= 14)
        })
        .map(|g| g.id)
        .sum()
}

fn part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            (0..3)
                .map(|i| game.buckets.iter().map(|r| r[i]).max().unwrap()) // Best attempt to get the max
                .product::<u32>() // product here of the results of map
        })
        .sum() // Aggregation
}

fn main() {
    println!("Hello, world! Part1");
    let mut game_vec: Vec<Game> = vec![];
    for line in DATA.lines() {
        println!("line : {:?}", line);
        // Make the game using FromStr implementation
        game_vec.push(Game::from_str(line).unwrap());
    }

    // Use the built vec and pass it as a slice to methods
    let part1_sum = part1(game_vec.as_slice());
    println!("part1 sum: {:?}", part1_sum);

    // Use the built vec and pass it as a slice to methods
    let part2_sum = part2(game_vec.as_slice());
    println!("part2 sum: {:?}", part2_sum);
}
