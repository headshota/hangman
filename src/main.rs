use std::io;
use std::collections::HashMap;

fn main() {
    println!("=== Welcome to hangman! ===");
    println!("Please enter a secret word:");

    let mut secret_word = String::new();

    io::stdin()
        .read_line(&mut secret_word)
        .expect("Failed to read line");

    secret_word = secret_word.trim().to_owned();

    while (secret_word.len() < 1) || !secret_word.chars().all(char::is_alphabetic) {
        println!("Secret word can only contain letters and can't be empty");

        secret_word = String::new();
        io::stdin()
            .read_line(&mut secret_word)
            .expect("Failed to read line");
        secret_word = secret_word.trim().to_owned();
    }

    let mut guesses = 6;

    let secret_chars = secret_word.chars();

    let mut secret_chars_index: HashMap<char, Vec<usize>> = HashMap::new();

    secret_chars.enumerate().for_each(|(i, c)|{
        secret_chars_index.entry(c).and_modify(|val| val.push(i)).or_insert(vec![i]);
    });


    let cnt = secret_word.len();
    let mut masked_guess_word = (0..cnt).map(|_| "_").collect::<String>();

    while guesses > 0 {
        display_guess(&masked_guess_word);
        println!();
        println!("Please guess the letter, {} attempt(s):", guesses);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.trim().to_owned();

        while (guess.len() != 1) || !guess.chars().next().unwrap().is_alphabetic() {
            println!("Guess can only be 1 letter character!");

            guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            guess = guess.trim().to_owned();
        }

        let guessed_char = guess.trim().chars().next().unwrap();

        guess_letter(guessed_char, &mut masked_guess_word, &secret_chars_index, &mut guesses);

        if !masked_guess_word.contains("_") {
            println!("You've guessed all letters, you've won!");
            return;
        }
    }

    println!("You've ran out of guesses, you are hanged!");

}

fn guess_letter(guess: char, masked_guess_word: &mut String, secret_chars_index: &HashMap<char, Vec<usize>>, guesses: &mut i32){
    match secret_chars_index.get(&guess) {
        Some(indices) => {
            indices.iter().for_each(|i|{
                masked_guess_word.replace_range(*i..i+1, &format!("{}", guess));
            })
        },
        None => {
            *guesses -= 1;
        }

    }
}

fn display_guess(masked_guess_word: &String) {
    let formatted_mask: String = masked_guess_word.chars().map(|c| format!("{} ", c)).collect();
    println!("{}", formatted_mask);
}

