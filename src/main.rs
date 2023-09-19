use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::{fs};
use wordle::*;

fn main() {
    // read words
    let fp = "allowed_words.txt";
    let contents = fs::read_to_string(fp).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split('\n').collect();

    let wins = Arc::new(Mutex::new([0; 7]));

    let mut buffer = String::new();

    let mut guesses = contents.clone();
    let mut answers = contents.clone();
    for _ in 0..6 {
        let (guess, entropy) = calc_probs(&guesses, &answers);
        println!(
            "You should guess {}, it has a predicted information of {}",
            guess, entropy
        );

        std::io::stdin().read_line(&mut buffer).unwrap();
        let mask = make_mask(buffer.trim());
        buffer.clear();

        mask_answers(mask, guess.trim(), &mut guesses);
        mask_answers(mask, guess.trim(), &mut answers);
    }

    contents.par_iter().for_each(|word| {
        let mut guesses = contents.clone();
        let mut answers = contents.clone();
        let wins = wins.clone();
        let guess = "soare";
        if guess == *word {
            let mut wins = wins.lock().unwrap();
            wins[0] += 1;
            return;
        }
        let mask = play_mask(guess, word);
        mask_answers(mask, guess.trim(), &mut guesses);
        mask_answers(mask, guess.trim(), &mut answers);
        for i in 1..6 {
            let (guess, _entropy) = calc_probs(&guesses, &answers);
            if guess == *word {
                let mut wins = wins.lock().unwrap();
                wins[i] += 1;
                return;
            }
            let mask = play_mask(&guess, word);
            mask_answers(mask, guess.trim(), &mut answers);
            mask_answers(mask, guess.trim(), &mut guesses);
        }
        let mut wins = wins.lock().unwrap();
        wins[6] += 1;
    });
    println!("{:?}", wins);
}
