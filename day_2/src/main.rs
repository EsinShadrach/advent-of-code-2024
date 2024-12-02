fn main() {
    println!("Hello, world!");
    read_input();
}

fn read_input() {
    let mut _safe_count = 0;
    let mut _unsafe_count = 0;
    let file_input = "input.txt";

    let file = std::fs::read_to_string(file_input).unwrap();
    let _lines: Vec<&str> = file.lines().collect();

    for line in _lines.iter() {
        let items: Vec<i8> = line
            .split_whitespace()
            .map(|element| {
                return element.parse::<i8>().expect("Failed to parse int");
            })
            .collect();

        if is_safe_report(&items) || can_be_safe(&items) {
            _safe_count += 1;
        } else {
            _unsafe_count += 1;
        }
    }

    println!("Safe: {}", _safe_count);
}

/// Determines if a report is safe by checking both conditions:
/// 1. The levels are either all increasing or all decreasing.
/// 2. Any two adjacent levels differ by at least 1 and at most 3.
fn is_safe_report(data: &Vec<i8>) -> bool {
    let mut all_increasing = true;
    let mut all_decreasing = true;

    for i in 0..data.len() - 1 {
        let prev = data[i];
        let next = data[i + 1];
        let diff = next - prev;

        if diff.abs() < 1 || diff.abs() > 3 {
            println!("Unsafe: {:?}, Invalid difference: {}", data, diff);
            return false;
        }

        if diff < 0 {
            all_increasing = false;
        } else if diff > 0 {
            all_decreasing = false;
        }
    }

    if all_increasing || all_decreasing {
        println!("Safe: {:?}", data);
        return true;
    }

    println!("Unsafe: {:?}, Neither Increasing or decreasing", data);
    return false;
}

/// Can be marked as safe if any value is removed
fn can_be_safe(data: &Vec<i8>) -> bool {
    for i in 0..data.len() {
        let mut temp = data.clone();
        temp.remove(i);

        if is_safe_report(&temp) {
            return true;
        }
    }
    return false;
}
