use std::io ; 
use std::cmp::Ordering; 
use rand::Rng;
use colored::*;

fn main() {

println!("Guess the Number Game");

let secret_number  = rand::thread_rng().gen_range(1, 101);

// println!(" The secret num is {}",secret_number);
loop{println!(" Please input your guess");

let mut guess = String::new();

io::stdin().read_line( &mut guess).expect("Failed to read line");

let guess :u32= match guess.trim().parse(){
  Ok(num)=> num, // if parsing successful return number as u32 type
  Err(_)=> continue
};

println!("You guessed : {}",
guess);



match guess.cmp(&secret_number){
Ordering::Less =>println!("{}","Too small".red()),
Ordering::Greater=> println!("{}","Too Big".red()),
Ordering::Equal=> {
  println!("{}","You win".green());
break;
}
};
}

}
