use std::io;

fn main() {
    println!("Working on mini projects");

    println!("Convert fahrenheit_to_celsius");
    let temp = fahrenheit_to_celsius();
    println!("Temperture in Celsius: {temp} ");
}

fn fahrenheit_to_celsius() -> f32 {
    println!("Convert Fahrenheit to Celsius");
    println!("Enter temperature in Fahrenheit");

    let mut temperature: f32 = 0.0;
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Unable to read input");
    temperature = temp.trim().parse().expect("Not a valid number");

    (temperature - 32.0) * (5.0/9.0)

}