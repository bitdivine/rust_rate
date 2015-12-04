extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Rate, wie alt ich bin!");
	let alter = rand::thread_rng().gen_range(1,140);
	loop {
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.ok()
			.expect("Was war das?");
		let guess:u32 = match guess.trim().parse() {
				Ok(num)		=> num,
				Err(_)		=> continue,
			};
		match guess.cmp(&alter){
			Ordering::Less		=> println!("Nee, älter!"),
			Ordering::Greater	=> println!("Nee, ich bin doch nicht so alt!"),
			Ordering::Equal		=> {
					println!("Naja, geraten!");
					break;
				}
		};
	}
}
