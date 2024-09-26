use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::{thread, time};

fn main() {

    let humorous_insults: [&str; 14] = [
        "You are about one bit short of a byte.",
        "You are impossible to underestimate.",
        "I thought I made the instructions idiot proof,",
        "Your logic is like a lawn full of weeds.",
        "Spacial reasoning must not be your strong suit.",
        "Can we normalize telling you that you aren’t so wonderful?",
        "Playing with you is like stepping on a leaf in autumn and hearing no crunch - disappointment.",
        "We are not going appease that empty brain of yours.",
        "A fool is the same all year round, and we celebrate you on April 1st.",
        "I’m jealous of how you can be so dumb.",
        "You make me increase the amount of caffeine I take daily.",
        "Do you see the light at the end of the tunnel? Your presence keeps covering it up.",
        "If I were to jump down from your IQ, it would equate to hopping.",
        "You do realize we tolerate you."
    ];

    println!("This game is called, GUESS THE NUMBER!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess. (positive integers only)");
    loop {
        let mut guess = String::new();
        let insult_number = rand::thread_rng().gen_range(0..=13);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                thread::sleep(time::Duration::from_millis(1500));
                println!("C'MON, DON'T YOU KNOW WHAT A POSITIVE INTEGER IS, YOU MORON!!");
                thread::sleep(time::Duration::from_millis(2000));
                println!("Please try again...");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        thread::sleep(time::Duration::from_millis(1000));
        
        println!("Calculating...");
        thread::sleep(time::Duration::from_millis(3000));
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                thread::sleep(time::Duration::from_millis(1000));
                println!("Took you long enough.");
                println!("Press ENTER so I can avoid playing with you anymore!");
                io::stdin().read_line(&mut String::new()).unwrap();
                return;
            }
        }
        thread::sleep(time::Duration::from_millis(1000));

        println!("{}", humorous_insults[insult_number]);
        thread::sleep(time::Duration::from_millis(3000));

        println!("Please try again...");
    }

}
