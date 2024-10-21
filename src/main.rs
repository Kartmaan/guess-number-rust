/// This program makes the user guess a random number.
///  
/// As a first Rust project, the goal here is to exploit as many 
/// different concepts as possible (borrowing, loops, enum, 
/// pattern matching, Result, etc.) even if it means sometimes 
/// making the code a little more convoluted.

use std::io; // User input
use rand::Rng; // Random Number Generator

/// The variant of this enum represent all possible discrete 
/// values of numeric comparaisons between the user-inserted
/// number and the generated random number
enum GuessResult {
    Greater,
    Less,
    Equal,
}

/// Checks if the choosen level is valid and returns a Result type
fn lvl_checker(lvl:&u8) -> Result<u8, String> {
    if *lvl >= 1 && *lvl <= 4 {
        Ok(*lvl)
    } else {
        Err(String::from("Invalid lvl : must be between 1 and 4"))
    }
}

/// Compares the user-guessed value (guessed_num) to the one 
/// randomly generated by the program (rnd_num) and returns a 
/// variant of the GuessResult enum based on the result of this
/// comparison
/// 
/// # 'cmp' method
/// The cmp method compares two integers and returns an 
/// enumerator (Ordering type) which can be either : 
/// Less, Greater or Equal 
/// 
/// ## Example :
/// let num1 = 15;
/// let num2 = 22;
/// let compare = num1.cmp(&num2)
/// 'let compare' now contains : std::cmp::Ordering::Less
fn equality_match(guessed_num: i32, rnd_num: i32) -> GuessResult {
    match guessed_num.cmp(&rnd_num) {
        std::cmp::Ordering::Less => GuessResult::Greater,
        std::cmp::Ordering::Greater => GuessResult::Less,
        std::cmp::Ordering::Equal => GuessResult::Equal,
    }
}

fn main() {
    // Level choice input (String)
    let mut lvl_input: String = String::new();

    // Upper bound of the rng range
    #[allow(unused_assignments)]
    let mut num_max: i32 = 500;

    // Level choice loop
    'lvl_loop : loop {
        lvl_input.clear(); // User input is cleared
        println!("Please choose a level (1-4) :");

        // Input reader
        io::stdin()
            .read_line(&mut lvl_input)
            .expect("Failed to read");

        // Input conversion to u8
        let lvl: u8;
        match lvl_input.trim().parse::<u8>() {
            Ok(val) => lvl = val,
            Err(_) => {
                println!("Not a valid number");
                continue 'lvl_loop;
            }
        }

        // Checks the return of the 'lvl_checker' function that 
        // is supposed to return a Result type. The choice of 
        // level impacts the value of the upper bound of the 
        // random number generation 'num_max'
        match lvl_checker(&lvl) {
            Ok(choice) => {
                println!("Level {}", choice);
                match lvl {
                    1 => num_max = 500,
                    2 => num_max = 1000,
                    3 => num_max = 5000,
                    4 => num_max = 10000,
                    _ => num_max = 500,
                }
                break 'lvl_loop; // Loop exit
            }
            Err(_) => continue 'lvl_loop, // Loop resume
        } 
    }

    // Number of player attempts
    let mut attempts: i32 = 0;

    // Random number generation
    // Integer between 1 (include) and num_max (include)
    let rnd_num: i32 = rand::thread_rng().gen_range(1..=num_max);

    // The 'in_guess' loop will continue while true 
    let mut in_guess: bool = true;

    // Void String vide to retrieve the user input
    let mut input: String = String::new();

    // LOOP START
    'in_guess_loop : while in_guess {
        input.clear(); // User input is cleared
        println!("Try to guess the number between 0 et {} :", num_max);

        // Reading the user input
        // read_line() : all the line will be read until the 
        // player press Enter
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read");

        // Convert String in to i32
        // trim() : sdelete all spaces
        // parse() : conversion according to the type of assigned variable
        let num_guessed: i32;
        match input.trim().parse::<i32>() {
            Ok(val) => num_guessed = val,
            Err(_) => {
                println!("Please type a valid number");
                continue 'in_guess_loop;
            }
        }
        
        // Submits the number entered by the player and the random
        // number generated to the 'equality_match' function for 
        // comparison. The function returns a variant of GuessResult
        let guess:GuessResult = equality_match(num_guessed, rnd_num);

        // Instructions to execute according to the comparison result
        match guess {
            // It's less
            GuessResult::Less => {
                println!("It's less !");
                attempts += 1;
            }

            // It's greater
            GuessResult::Greater => {
                println!("It's greater !");
                attempts += 1;
            }

            // It's equal
            // The 'in_guess' flag turns to false so the loop can ends
            GuessResult::Equal => {
                println!("Well done !");
                attempts += 1;
                in_guess = false;
            },
        } // match guess
        } // While

    println!("You succeeded in {} attempts", attempts);
}