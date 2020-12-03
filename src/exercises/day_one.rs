use std::fs;

pub fn run() {
    println!("Day 1.");

    exercise_1();
    exercise_2();
}

fn exercise_1() {
    println!("Exercise 1");

    let lines = get_input();

    println!("there are {} lines in the input", lines.len());

    for a in lines.iter() {
        for b in lines.iter() {
            let total = a + b;
            if total == 2020 {
                println!("{} + {} = {}", a, b, total);
                println!("Answer: {}", a + b);
            }
        }
    }
}

fn exercise_2() {
    println!("Exercise 2");

    let inputs = get_input();

    for a in inputs.iter() {
        for b in inputs.iter() {
            for c in inputs.iter() {
                let total = a + b + c;

                if total == 2020 {
                    println!("{} + {} + {} b = {} => {}", a, b, c, total, a * b * c);
                }
            }
        }
    }
}

pub fn get_input() -> Vec<u32> {
    let input_filename = "input_data/day_1.txt";
    println!("In file {}", input_filename);

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    // println!("contents: {}", contents);

    let lines: Vec<&str> = contents.split_whitespace().collect();
    return lines.iter().map(|x| parse_num(x)).collect();
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
