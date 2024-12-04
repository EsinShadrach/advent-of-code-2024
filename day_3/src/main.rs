use regex::Regex;

fn main() {
    println!("Hello, world!");
    read_input_txt();
}

fn read_input_txt() {
    const INPUT_FILE: &str = "input.txt";

    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    // match mul(digit-X, digit-Y)
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;

    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            let multiplication = x * y;

            println!("Found mul({}, {}), result: {}", x, y, multiplication);
            result += x * y;
        }
    }

    println!("Total result: {}", result);
}
