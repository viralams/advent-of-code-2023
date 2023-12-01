const TEST_DATA: &str = include_str!("input1.txt");

fn main() {
    let mut total = 0;
    let mut lines = TEST_DATA.lines();

    for line in lines {
        let chars = line.chars();
        let mut first_num: Option<char> = None;
        let mut last_num: Option<char> = None;
        let mut line_vec = vec![];

        for char in chars {
            if char.is_numeric() {
                if first_num == None {
                    first_num = Some(char);
                }
                last_num = Some(char);
            }
        }
        line_vec.push(first_num.unwrap());
        line_vec.push(last_num.unwrap());
        let line_val = line_vec.into_iter().collect::<String>().parse::<i32>().unwrap();
        total = total+ line_val;
    }
    println!("Total is : {:?}", total);
}