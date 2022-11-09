use std::{fs::File, io::*};
use rand::seq::IteratorRandom;
use unicode_segmentation::UnicodeSegmentation;

fn main(){
    // variables
    let mut strikes = 0;
    let allowed_strikes = 8;
    let mut previous_guesses: Vec<char> = Vec::new();
    let generated_word: Vec<char> = generate_word().unwrap(); // word in list of character form
    let size = generated_word.len();
    let mut guessed_word:Vec<char> = "_".repeat(size).chars().collect(); // guess state

    // game commencing
    println!("Let's play hangman!");
    println!("Here's your word: ({} letters)", size);

    while strikes < allowed_strikes {
        // player guess
        let mut player_input: String = String::new();
        let state: String = guessed_word.iter().collect(); // shows the guessed word progress in string form
        println!("You have {} out of {} strikes.", strikes, allowed_strikes);
        println!("{}    Guesses: {:?}", state, previous_guesses);
        println!("Enter your one letter guess:");

        while player_input.graphemes(true).count() != 1 {

            stdin().read_line(&mut player_input).expect("Could not read player input.");
            player_input = player_input.trim().to_string();
            if player_input.graphemes(true).count() != 1 { // same thing as len() but better
                println!("Please enter one character.");
                player_input = String::new();
            }
            else {
                let chars: Vec<char> = player_input.chars().collect();
                let char: char = chars[0];
                if  previous_guesses.contains(&char){
                    println!("You have already guessed this.");
                    player_input = String::new(); // should work as you are creating new string before exiting the scope
                }
                else {
                    previous_guesses.push(char);
                }
            }
        }
        clear_console();

        // handling of guess
        let player_input: Vec<char> = player_input.chars().collect();
        let player_input: char = player_input[0];
        let word: String = generated_word.iter().collect(); 
        //
        if generated_word.contains(&player_input) { // guess was right
            for (i, v) in generated_word.iter().enumerate() {
                if player_input.to_string() == v.to_string() {
                    guessed_word[i] = *v;
                }
            }
        }
        else { // guess was wrong
            strikes += 1;
            println!("Oops!");
            if strikes == allowed_strikes {
                println!("You lost! The word was {}.", word);
            }
        }
        if guessed_word == generated_word {
            println!("You got it! The word was {}.", word);
            break
        }
    }
}

fn generate_word() -> std::result::Result<Vec<char>, Error>{
    let file = File::open("dict.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let word = lines.choose(&mut rand::thread_rng()).expect("File had no lines").unwrap();
    let word: Vec<char> = word.chars().collect();
    Ok::<Vec<char>, Error>(word)
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H"); 
}