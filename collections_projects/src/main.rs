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

    println!("Project-3: Store employees by department");
    store_departments();
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

fn store_departments() {
    let mut register: HashMap<String, Vec<String>> = HashMap::new();

    let s_1 = String::from("Add Sally to Engineering");
    let s_2 = String::from("Add Amir to Sales");
    let s_3 = String::from("Add Sankit to Engineering");
    add_employee(s_1, &mut register);
    add_employee(s_2, &mut register);
    add_employee(s_3, &mut register);

    println!("Current register: {:?}", register);

    let d_1 = String::from("Engineering");
    let d_2 = String::from("Sales");
    emp_by_department(d_1, &register);
    emp_by_department(d_2, &register);
}

fn add_employee(sentence : String, register: &mut HashMap<String, Vec<String>>) {
    let parts : Vec<&str> = sentence.split(" ").collect();
    let emp_name = &parts[1].to_string();
    let emp_department = parts[3].to_string();
    let _ = &register.entry(emp_department)
            .and_modify(|v| v.push((&emp_name).to_string()))
            .or_insert(vec![(&emp_name).to_string()]);
}

fn emp_by_department(department: String, register: &HashMap<String, Vec<String>>) {
    if let Some(emp_names) = &register.get(&department) {
        println!("{:?}", emp_names);
    }
}
