use std::io;

fn main() {
    println!("Working on mini projects");

    println!("Project 1");
    println!("Convert fahrenheit_to_celsius");
    let temp = fahrenheit_to_celsius();
    println!("Temperture in Celsius: {temp} ");

    println!("Project2");
    println!("Get the nth fibonacci number");
    let res = fibonacci();
    println!("The nth fibonacci number is {res}");
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
