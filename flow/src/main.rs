#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
    Dollar,
}

fn match_flow(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter for state: {:?}", state);
            25
        },
        _ => {
            println!("The denomination not found");
            0
        },
    }
}

fn main() {
    println!("value of Nickel: {}", match_flow(Coin::Nickel));
    println!(
        "value for Quarter from New York: {}",
        match_flow(Coin::Quarter(USState::NewYork))
    );
    println!("value for Dollar: {}", match_flow(Coin::Dollar));
}
