fn main() {
    read_input_txt();
}

fn read_input_txt() {
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];

    let input: String = std::fs::read_to_string("input.txt").unwrap();

    let trimmed_input = input.trim();

    for itx in trimmed_input.lines().into_iter() {
        let parts: Vec<&str> = itx.split_whitespace().collect();

        if parts.len() != 2 {
            panic!("Each Line must contain 2 nums");
        }

        let left_val = parts[0]
            .parse::<i64>()
            .expect("Failed to parse the left value as an integer");

        let right_val = parts[1]
            .parse::<i64>()
            .expect("Failed to parse the right value as an integer");

        left.push(left_val);
        right.push(right_val);
    }

    assert!(
        left.len() == right.len(),
        "RIGHT AND LEFT HAVE DIFFERENT LENGTH DO NOT MATCH"
    );

    left.sort();
    right.sort();

    let mut total_distance = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        // absolute value in case of negatives
        let distance = (r - l).abs();

        println!("Pair: ({}, {}), Distance: {}", l, r, distance);

        total_distance += distance;
    }

    println!("Total Distance: {}", total_distance);
}
