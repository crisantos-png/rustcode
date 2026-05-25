use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
	guessing_game()
}
fn guessing_game(){
	let mut rng = rand::rng();
	let secret_num = rng.random_range(1..=100);
	
	println!("Guess the number!");
	let mut guess = String::new();
	let mut counter = 0;

	loop {
		guess.clear();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to take input");
		let guess: i32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Enter a valid number");
				continue;
			},
		};

		counter += 1;

		match guess.cmp(&secret_num) {
			Ordering::Greater => println!("Too high"),
			Ordering::Less => println!("Too low"),
			Ordering::Equal => {
				println!("Correct");
				break;
			}

		}
		if counter > 9 {
			println!("No more trials");
			break;
		}
	}

	println!("The secret nummer was {}", secret_num);	
}
