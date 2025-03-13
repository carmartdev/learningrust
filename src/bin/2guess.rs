use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    let num = rand::thread_rng().gen_range(1..=101);
    println!("Guess the number!");
    println!("What's your number?");
    let mut guess = String::new();
    loop{
	io::stdin().read_line(&mut guess)
	    .expect("Failed to read line");
	let guess:u32 = guess.trim().parse()
	    .expect("Type a proper number!");
	match guess.cmp(&num){
	    Ordering::Less => println!("Too Low"),
	    Ordering::Greater => println!("Too High"),
	    Ordering::Equal => {
		println!("Correct, You Won!!!");
		break;
	    }
	}
    }
}
