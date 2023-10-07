use serde::Deserialize;
use std::fs;
use wordle::*;

#[derive(Debug, Deserialize)]
struct Line {
    probability: f32,
    word: String,
}

fn main() {
    // read words
    let mut rdr =
        csv::Reader::from_path("freqs.csv").expect("Should have been able to read the file");
    let contents: Vec<([u8; 5], f32)> = rdr
        .deserialize()
        .map(|res| -> ([u8; 5], f32) {
            let record: Line = res.unwrap();

            (
                record.word.as_bytes().try_into().unwrap(),
                record.probability,
            )
        })
        .collect();

    let mut buffer = String::new();
    let mut guesses = contents.clone();

    for i in 0..6 {
        let (guess, entropy) = calc_probs(&guesses, i);
        let disp = std::str::from_utf8(&guess).unwrap();

        println!(
            "You should guess {}, it has a predicted information of {}",
            disp, entropy
        );

        std::io::stdin().read_line(&mut buffer).unwrap();
        let mask = make_mask(buffer.trim());
        buffer.clear();

        mask_answers(mask, &guess, &mut guesses);
    }

    // let wins = Arc::new(Mutex::new([0; 7]));
    // contents.par_iter().for_each(|word| {
    //     let mut guesses = contents.clone();
    //     let mut answers = contents.clone();
    //     let wins = wins.clone();
    //     let guess = "soare";
    //     if guess == *word {
    //         let mut wins = wins.lock().unwrap();
    //         wins[0] += 1;
    //         return;
    //     }
    //     let mask = play_mask(guess, word);
    //     mask_answers(mask, guess.trim(), &mut guesses);
    //     mask_answers(mask, guess.trim(), &mut answers);
    //     for i in 1..6 {
    //         let (guess, _entropy) = calc_probs(&guesses, &answers);
    //         if guess == *word {
    //             let mut wins = wins.lock().unwrap();
    //             wins[i] += 1;
    //             return;
    //         }
    //         let mask = play_mask(&guess, word);
    //         mask_answers(mask, guess.trim(), &mut answers);
    //         mask_answers(mask, guess.trim(), &mut guesses);
    //     }
    //     let mut wins = wins.lock().unwrap();
    //     wins[6] += 1;
    // });
    // println!("{:?}", wins);
}
