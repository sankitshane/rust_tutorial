use std::ops::Div;
use std::collections::HashMap;

fn main() {
    println!("This Rust Collections project contain 3 functions ");
    let mut input = [7, 4, 8, 2, 7, 9, 2, 5];
    let result = find_median_and_mode(&mut input);
    println!("Median is {} and Mode is {}", result.0, result.1);
}

fn find_median_and_mode(input: &mut [i32]) -> (i32, i32) {
    fn find_median(input: &mut [i32]) -> i32 {
        input.sort();
        let mid = input.len().div(2);
        input[mid]
    }

    fn find_mode(input: &mut [i32]) -> i32 {
        let mut map = HashMap::new();
        for num in input.iter() {
            let count = map.entry(*num).or_insert(0);
            *count += 1;
        }

        let mut mode = 0;
        let mut max_frequency = std::i32::MIN;
        for (&key, &value) in &map {
            if value > max_frequency {
                max_frequency = value;
                mode = key;
            }
        }

        mode
    }

    let median = find_median(input);
    let mode = find_mode(input);
    (median, mode)
}