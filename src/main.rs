use std::io;
use rand::Rng;


fn send_bitcoin() {
    let amount = rand::thread_rng().gen_range(1..=100);
    println!("Sending bitcoin amount: {}", amount);
}

fn recieve_bitcoin() {
    println!("Enter the amount of bitcoin you want to receive:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let amount: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount, assuming 0");
            0.0
        }
    };
    println!("Received bitcoin amount: {}", amount);
}

fn invalid_choice() {
    println!("Invalid choice");
}

fn console(){
    println!("Hi, lets jiggy with some bitcoin \n");

    println!("Do you want to send (s) or recieve bitcoin (r) \n?");

    let mut command = String::new();
    
    io::stdin().read_line(&mut command).expect("Failed to read line");

    if command.trim().eq("s") {
        send_bitcoin();
    } else if command.trim().eq("r") {
        recieve_bitcoin();
    } else {
        invalid_choice();
    }
}

fn main() {
    console();
}
