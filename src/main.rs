use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn receive_validate_input() ->u32{
    println!("Enter your number:");
    let mut player_intended_input = String::new();
    io::stdin().read_line(&mut player_intended_input)
    .expect("Failed to read line");
    let player_intended_input:u32 = match player_intended_input.trim().parse(){
        Ok(num) => num,
        Err(_) => return 0,
    };
    if player_intended_input < 1 || player_intended_input > 6{
        println!("Wrong input! You have to enter between 1 and 6!");
        return 0;
    }
    return player_intended_input;
}

fn player_batting(player_runs : &mut u32, computer_runs : &mut u32){
    loop{
        let player_intended_run = receive_validate_input();
        if player_intended_run == 0{
            continue;
        }
        let computer_number = rand::thread_rng().gen_range(1,7);
        println!("Computer entered : {}",computer_number);
        match player_intended_run.cmp(&computer_number){
            Ordering::Equal => {
                println!("That's out! Computer needs {} to win", *player_runs+1);
                break;
            }
            _ => {
                *player_runs+=player_intended_run;
                println!("Runs : {}", *player_runs);
            }
        }
    }
}
fn player_bowling(player_runs : &mut u32, computer_runs : &mut u32){
    loop{
        let player_intended_ball = receive_validate_input();
        let computer_intended_run = rand::thread_rng().gen_range(1,7);
        println!("Computer entered : {}",computer_intended_run);

        match computer_intended_run.cmp(&player_intended_ball){
            Ordering::Equal => {
                println!("That's out! Computer score : {} You win by {} runs", computer_runs, *player_runs-*computer_runs+1);
                break;
            }
            _ => {
                *computer_runs += computer_intended_run;
                if *computer_runs > *player_runs + 1{
                    println!("Computer wins! Better luck next time!");
                    break;
                }
                else{
                    println!("Computer Runs : {}, Computer needs {} runs to win", *computer_runs, *player_runs-*computer_runs+1);
                }
            }
        }
    }
}
fn main() {
    println!("Welcome to the Hand Cricket game!");

    println!("You get to bat first! Your innings starts now!");

    let mut player_runs:u32 = 0;
    let mut computer_runs:u32 = 0;
    player_batting(&mut player_runs,&mut computer_runs);
    println!("=======End of Innings=====");
    println!("You get to bowl now!");
    println!("Computer needs {} to win", player_runs+1);
    player_bowling(&mut player_runs, &mut computer_runs);
}
