use std::io::*;
use rand::seq::IteratorRandom;
use std::fs::File;
use std::collections::HashSet;

fn main() -> Result<()> {
    println!("Welcome to a game of hangman!");

    let mut word = get_word()?;
    word = word.trim().to_string();

    let mut guess: Vec<String> = word
        .chars()
        .map(|_| "_".to_string())
        .collect();

    let mut letters = HashSet::new();

    println!("{}", guess.join(" "));
    println!("");

    while guess.join("") != word {
        println!("Enter a letter! Already guessed: {:?}", letters);
        let mut character = String::new();
        stdin().read_line(&mut character)?;
        character = character.trim().to_string();
        let mut in_word = false;

        guess = word
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .zip(guess.iter())
            .map(|(w, g)| if w.to_string().eq(&character) {
                in_word = true;
                w.to_string()
            } else {
                g.to_string()
            })
            .collect::<Vec<String>>();
        println!("");
        println!("{} \n", guess.join(" "));

        if !in_word {
            letters.insert(character);
        }


        if letters.len() > 8 {
            println!("Sorry, you guessed wrong too many times :( The correct word was \"{}\"", word);
            return Ok(());
        }

    }
    println!("Yay, you did it! The word was \"{}\"", word);
    Ok(())
}

fn get_word() -> Result<String> {
    let f = File::open("src/words.txt").unwrap_or_else(|e| panic!("file not found: {}: {}", "src/words.txt", e));
    let f = BufReader::new(f);
    Ok(f
        .lines()
        .map(|l| l.expect("Couldn't read line"))
        .filter(|x| x.len() > 4)
        .choose(&mut rand::thread_rng())
        .unwrap())
}

