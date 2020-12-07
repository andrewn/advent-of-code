use std::fs;
use std::convert::TryInto;

pub fn run() {
    println!("Day 2.");
    let mut valid_passwords =Vec::new();
    let lines = get_input();
    let total_lines = lines.len();
    for line in lines {
        let policy = parse_to_policy(line.clone());
        let password = parse_password(line.clone());
        println!("password \"{}\"", password);
        let is_match = policy.matches(password.clone());
        println!("min: {} max: {} char {}", policy.min, policy.max, policy.char);
        println!("matches? {}", is_match);
        if is_match {
            valid_passwords.push(password.clone())
        }
    }

    println!("Total passwords: {}", total_lines);
    println!("Valid: {}", valid_passwords.len());
}


struct Policy {
    min: u32,
    max: u32,
    char: char,
}

impl Policy {
    fn matches(&self, password: String) -> bool {
        let num_chars = password.matches(self.char).count();

        println!("password \"{}\", char {}, numChars {}", password, self.char, num_chars);

        return num_chars >= self.min.try_into().unwrap() && num_chars <= self.max.try_into().unwrap()
    }
}

fn parse_to_policy(input: String) -> Policy {
    let parts: Vec<&str> = input.split(|c| c == '-' || c == ' ' || c == ':').collect();
    return Policy { min: parse_num(parts[0]), max: parse_num(parts[1]), char: parse_char(parts[2]) };
}

fn parse_password(input: String) -> String {
    let parts: Vec<&str> = input.split(|c| c == ':').collect();
    return parts[1].trim().to_string();
}

pub fn get_input() -> Vec<String> {
    let input_filename = "input_data/day_2.txt";
    println!("In file {}", input_filename);

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    return contents.lines().map(|x| String::from(x)).collect();
}

// Utils
fn parse_num(val: &str) -> u32 {
    let parsed = val.parse::<u32>();

    let num = match parsed {
        Ok(num) => num,
        Err(error) => panic!("Problem parsing the number: {:?}", error),
    };

    return num;
}

fn parse_char(val: &str) -> char {
    let parsed = val.parse::<char>();

    let num = match parsed {
        Ok(num) => num,
        Err(error) => panic!("Problem parsing the char: {:?}", error),
    };

    return num;
}
