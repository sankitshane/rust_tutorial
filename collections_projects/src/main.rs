use std::ops::Div;
use std::collections::HashMap;

fn main() {
    println!("This Rust Collections project contain 3 functions ");

    println!("Project-1: Find Median and Mode");
    let mut input = [7, 4, 8, 2, 7, 9, 2, 5];
    let result = find_median_and_mode(&mut input);
    println!("Median is {} and Mode is {}", result.0, result.1);

    println!("Project-2: Pig Latin converter");
    let input_1 = String::from("apple");
    let input_2 = String::from("first");
    let result_1 = generate_pig_latin(&input_1);
    let result_2 = generate_pig_latin(&input_2);
    println!(
        "The result for project are {input_1}: {result_1} & {input_2}: {result_2}"
    );
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

fn generate_pig_latin(input: &String) -> String {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let mut chars = input.chars();
    if let Some(first_char) = chars.next() {
        if vowels.contains(&first_char) {
            format!("{}-hay", input)
        } else {
            format!("{}-{}ay", &input[1..], first_char)
        }
    } else {
        println!("Input is None");
        "".to_string()
    }

    
}
