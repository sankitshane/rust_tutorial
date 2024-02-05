use std::io;

fn main() {
    println!("Working on mini projects");
    println!("\n");

    println!("Project 1: Convert fahrenheit_to_celsius");
    println!("\n");
    let temp = fahrenheit_to_celsius();
    println!("Temperture in Celsius: {temp} ");
    println!("\n");

    println!("Project 2: Get the nth fibonacci number");
    println!("\n");
    let res = fibonacci();
    println!("The nth fibonacci number is {res}");
    println!("\n");

    println!("project 3: Generate The Twelve Days of Christmas");
    println!("\n");
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
    fn first_line(n: usize){
        // sending n from 0-2
        let number = [
            "first",
            "second",
            "third"
        ];

        println!("On the {} day of Christmas", number[n]);
        println!("my true love gave to me");
    }
    
    fn middle_line(n: usize) {
        let sentence = [
            "Two turtle doves",
            "Three French hens",
            "Four calling birds",
            "Five golden rings"
        ];

        for i in (0..n).rev() {
            println!("{}", sentence[i]);
        }


    }

    fn last_line() {
        println!("A partridge in a pear tree.");
    }

    for num in 0..3 {
        first_line(num);
        middle_line(num);
        last_line();
        println!("\n");
    }
}
