use std::{io, u32};

fn main() {
    
    let mut feelings = String::new();
    
    println!("Hello, good sir or madam!");
    println!("How are you? ");
    
    io::stdin()
        .read_line(&mut feelings)
        .expect("Failed to read line");
    
    println!("I'd like to ask you a rather personal question.");
    
    let age: u32 = loop {
        let mut input = String::new();
        println!("How old are you? ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("A NUMBER YOU FOOL");
                continue;
            }
        };
    };

    match age {
        0..=12 => println!("My my, sir or madam, you are a young one."),
        13..=25 => println!("All right good sir or madam, you are just getting started."),
        26..=37 => println!("A bit seasoned aren't we, sir or madam."),
        38 => println!("Well you're quite the old fart now aren't you, Eric!!"),
        39..=70 => println!("Being middle aged is something you should be proud of."),
        71..=120 => println!("You're only as old as you feel sir or madam"),
        121..=u32::MAX => println!("You must have achieved a state of ascension and are looking back on us with either bewilderment or pride")
    }
    
    println!("Good day to you now!  I'm off!")
}
