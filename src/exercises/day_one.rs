use std::fs;

pub fn run() {
    println!("Day 1.");
    let input_filename = "input_data/day_1.txt";
    println!("In file {}", input_filename);

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    println!("contents: {}", contents);

    let lines: Vec<&str> = contents.split_whitespace().collect();

    println!("there are {} lines in the input", lines.len());

    for a in lines.iter() {
        for b in lines.iter() {
            let total = parse_num(a) + parse_num(b);
            if total == 2020 {
                println!("{} + {} = {}", a, b, total);
                println!("Answer: {}", parse_num(a) * parse_num(b));
            }
        }
    }
}

// Utils
fn parse_num(val: &&str) -> u32 {
    let parsed = val.parse::<u32>();

    let num = match parsed {
        Ok(num) => num,
        Err(error) => panic!("Problem parsing the number: {:?}", error),
    };

    return num;
}
