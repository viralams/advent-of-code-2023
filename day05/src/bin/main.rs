use std::ops::Range;
use std::str::Split;

// const DATA: &str = include_str!("test.txt");
const DATA: &str = include_str!("input.txt");

// Making a struct with all the ranges for each
#[derive(Debug)]
struct SeedData {
    // tuple ranges to capture range of things in one shot
    seed_to_soil: Vec<(Range<u64>, Range<u64>)>,
    soil_to_fertilizer: Vec<(Range<u64>, Range<u64>)>,
    fertilizer_to_water: Vec<(Range<u64>, Range<u64>)>,
    water_to_light: Vec<(Range<u64>, Range<u64>)>,
    light_to_temperature: Vec<(Range<u64>, Range<u64>)>,
    temperature_to_humidity: Vec<(Range<u64>, Range<u64>)>,
    humidity_to_location: Vec<(Range<u64>, Range<u64>)>,
}

// Implement SeedData using ranges to make the mappings from given seed
impl SeedData {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = do_the_mapping(seed, &self.seed_to_soil);
        let fertilizer = do_the_mapping(soil, &self.soil_to_fertilizer);
        let water = do_the_mapping(fertilizer, &self.fertilizer_to_water);
        let light = do_the_mapping(water, &self.water_to_light);
        let temperature = do_the_mapping(light, &self.light_to_temperature);
        let humidity = do_the_mapping(temperature, &self.temperature_to_humidity);
        do_the_mapping(humidity, &self.humidity_to_location)
    }
}
// source to destination ranges are being iterated and we make the entire mapping
fn do_the_mapping(input: u64, map: &[(Range<u64>, Range<u64>)]) -> u64 {
    for (dest, source) in map {
        if source.contains(&input) {
            let shift = input - source.start;
            return dest.start + shift;
        }
    }
    input
}

fn extract_info(input: &str) -> (Vec<u64>, SeedData) {
    let mut parts = input.split("\n\n");

    let seeds = parts.next().unwrap()[7..]
        .split(' ')
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();

    let seed_to_soil = extract_and_collect(&mut parts);
    let soil_to_fertilizer = extract_and_collect(&mut parts);
    let fertilizer_to_water = extract_and_collect(&mut parts);
    let water_to_light = extract_and_collect(&mut parts);
    let light_to_temperature = extract_and_collect(&mut parts);
    let temperature_to_humidity = extract_and_collect(&mut parts);
    let humidity_to_location = extract_and_collect(&mut parts);

    (
        seeds,
        SeedData {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    )
}

fn extract_and_collect(parts: &mut Split<&str>) -> Vec<(Range<u64>, Range<u64>)> {
    parts
        .next()
        .unwrap()
        .split_once('\n')
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let mut nums = line.split(' ');
            let dest = nums.next().unwrap().parse::<u64>().unwrap();
            let source = nums.next().unwrap().parse::<u64>().unwrap();
            let range = nums.next().unwrap().parse::<u64>().unwrap();

            (dest..(dest + range), source..(source + range))
        })
        .collect()
}

fn main() {
    let (seeds, data) = extract_info(DATA);
    // println!("{:?}", seeds);
    // println!("{:?}", data);

    let min_stuff = seeds
        .clone()
        .into_iter()
        .map(|seed| data.seed_to_location(seed))
        .min()
        .unwrap();
    println!("Part1: {}", min_stuff);

    let another_min_stuff = seeds
        .clone() // Slowest thing I can think of instead of creating separate files and isolating the process
        .chunks_exact(2)
        .inspect(|_| println!("Im here now"))
        .flat_map(|range| {
            let &[start, length] = range else {
                unreachable!()
            }; // This makes it slower

            start..(start + length)
        })
        .into_iter()
        .map(|seed| data.seed_to_location(seed))
        .min()
        .unwrap(); // sloooooooow
    println!("Part2: {}", another_min_stuff);
}
