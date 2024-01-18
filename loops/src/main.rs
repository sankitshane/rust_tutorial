use std::io;
use std::collections::HashMap;

fn main() {
    println!("Working on mini projects");

    // println!("Project 1");
    // println!("Convert fahrenheit_to_celsius");
    // let temp = fahrenheit_to_celsius();
    // println!("Temperture in Celsius: {temp} ");

    // println!("Project 2");
    // println!("Get the nth fibonacci number");
    // let res = fibonacci();
    // println!("The nth fibonacci number is {res}");

    println!("project 3");
    println!("Generate The Twelve Days of Christmas");
    christmas_song();
}

fn fahrenheit_to_celsius() -> f32 {
    println!("Enter temperature in Fahrenheit");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Unable to read input");
    let temperature: f32 = temp.trim().parse().expect("Not a valid number");

    (temperature - 32.0) * (5.0/9.0)
}

fn fibonacci() -> i32 {
    fn dp(n: i32) -> i32 {
        if (n == 1) || (n == 0) {
            return n;
        }

        dp(n-1) + dp(n-2)
    }

    println!("Enter a value for n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("unable to read input");
    let n = input.trim().parse().expect("Not a valid number");

    return dp(n);
}

fn christmas_song() {
    fn generate_phase(n: i32){
        // sending n from 0-2
        let number = [
            "first",
            "second",
            "third"
        ];

        let mut num_name = HashMap::from([
            ("Two", "turtle"),
            ("Three", "French"),
        ]);

        let mut part = Vec::new();
        part.push("On the {number[n]} day of Christmas".to_string());
        part.push("my true love gave to me".to_string());
        
        part.push("A partridge in a pear tree.".to_string());
    }
    
    for num in 0..3 {
        generate_phase(num);
    }
}
