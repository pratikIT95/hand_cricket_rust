use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to the guessing game!"); 
    let secret_number = rand::thread_rng().gen_range(1,101);
    loop{
        println!("Guess a number!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("failed to read line");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations!");
                break;
            }
        }
    }
    
}
