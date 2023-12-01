use std::collections::HashMap;

const TEST_DATA: &str = include_str!("input1.txt");


fn main() {
    println!("{}", TEST_DATA);
    let mut total = 0;
    let mut lines = TEST_DATA.lines();
    for line in lines {
        let line_val = process_line(&str::to_string(line));
        println!("Line val: {:?}", line_val);
        total = total + line_val;
    }
    println!("Total is : {:?}", total);
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });
    let first = it.next().expect("first should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
        .parse::<u32>()
        .expect("rendered value should be a valid number in the valid range")
}

