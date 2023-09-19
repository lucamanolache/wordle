use rayon::prelude::*;
use std::collections::HashMap;

pub fn _calc_probs(guesses: &Vec<&str>, answers: &Vec<&str>) -> (String, f32) {
    // map of each guess to how many times it gets each key.
    // so bmap['soare'] = { 20002: 5, 10001: 3, ... }
    let bmap: HashMap<&str, HashMap<u32, u32>> = guesses
        .par_iter()
        .map(|guess| -> (&str, HashMap<u32, u32>) {
            let mut gmap: HashMap<u32, u32> = HashMap::new();
            let reses: Vec<u32> = answers
                .par_iter()
                .map(|x| -> u32 { gen_mask(guess, x) })
                .collect();
            for res in reses.iter() {
                // increase the amount of times we have seen this pattern
                gmap.insert(*res, gmap.get(&res).unwrap_or(&0) + 1);
            }
            (guess, gmap)
        })
        .collect();
    // information's stores how much information each pattern gives us. that is
    // calculated by - log prob of seeing that pattern
    let totals = guesses.len() * answers.len();
    let mut informations: HashMap<u32, f32> = HashMap::new();
    bmap.iter().for_each(|(_, m)| {
        m.iter().for_each(|(key, times)| {
            informations.insert(*key, *informations.get(key).unwrap_or(&0.0) + *times as f32);
        })
    });

    let informations: HashMap<u32, f32> = informations
        .par_iter()
        .map(|(k, v)| -> (u32, f32) { (*k, -(*v / totals as f32).log2()) })
        .collect();
    // now that we have the information of each pattern, calculate the expected
    // information of each word
    let bmap: Vec<(&str, f32)> = bmap
        .par_iter()
        .map(|(word, map)| -> (&str, f32) {
            let mut expected_information = 0.0;
            for (pattern, times) in map {
                // our expected information increases by the information of the
                // pattern * the chance we think of seeing that pattern
                expected_information += informations.get(pattern).unwrap_or(&0.0)
                    * (*times as f32 / answers.len() as f32);
            }
            (word, expected_information)
        })
        .collect();
    let (mut best, mut guess) = (0.0, "ERROR???");
    for (key, map) in bmap.iter() {
        // so total is just gonna be how many possible answers there are
        if *map > best {
            best = *map;
            guess = key;
        }
    }
    (guess.to_owned(), best)
}

pub fn calc_probs(guesses: &Vec<&str>, answers: &Vec<&str>) -> (String, f32) {
    // map of each guess to how many times it gets each key.
    // so bmap['soare'] = { 20002: 5, 10001: 3, ... }
    let bmap: HashMap<&str, HashMap<u32, u32>> = guesses
        .par_iter()
        .map(|guess| -> (&str, HashMap<u32, u32>) {
            let mut gmap: HashMap<u32, u32> = HashMap::new();
            let reses: Vec<u32> = answers
                .par_iter()
                .map(|x| -> u32 { gen_mask(guess, x) })
                .collect();
            for res in reses.iter() {
                // increase the amount of times we have seen this pattern
                gmap.insert(*res, gmap.get(&res).unwrap_or(&0) + 1);
            }
            (guess, gmap)
        })
        .collect();
    // calculate the expected information of each word
    let bmap: Vec<(&str, f32)> = bmap
        .par_iter()
        .map(|(word, map)| -> (&str, f32) {
            let mut expected_information = 0.0;
            for (_pattern, times) in map {
                // our expected information increases by the information of the
                // pattern * the chance we think of seeing that pattern
                let p = *times as f32 / answers.len() as f32;
                expected_information += p * -p.log2();
            }
            (word, expected_information)
        })
        .collect();
    let (mut best, mut guess) = (0.0, "ERROR???");
    for (key, map) in bmap.iter() {
        // so total is just gonna be how many possible answers there are
        if *map > best {
            best = *map;
            guess = key;
        }
    }
    (guess.to_owned(), best)
}

fn gen_mask(guess: &str, ans: &str) -> u32 {
    let mut res = 0;
    let mut chars = [0; 26];
    for c in ans.as_bytes() {
        chars[(c - 97) as usize] += 1;
    }
    for (i, (gc, ac)) in guess
        .as_bytes()
        .iter()
        .zip(ans.as_bytes().iter())
        .enumerate()
    {
        if gc == ac {
            // its green
            res += 2 * 3u32.pow(i as u32);
            chars[*gc as usize - 97] -= 1;
        } else {
            // lets see if its yellow
            if chars[*gc as usize - 97] > 0 {
                res += 1 * 3u32.pow(i as u32);
                chars[*gc as usize - 97] -= 1;
            }
        }
        // its gray so no need to do anything
    }
    res
}

pub fn mask_answers(mask: u32, guess: &str, answers: &mut Vec<&str>) {
    for i in (0..answers.len() - 1).rev() {
        let ans = answers[i];
        if ans.len() != 5 {
            continue;
        }
        let mut res = 0;
        let mut chars = [0; 26];
        for c in ans.as_bytes() {
            chars[(c - 97) as usize] += 1;
        }
        for (i, (gc, ac)) in guess
            .as_bytes()
            .iter()
            .zip(ans.as_bytes().iter())
            .enumerate()
        {
            if gc == ac {
                // its green
                res += 2 * 3u32.pow(i as u32);
                chars[*gc as usize - 97] -= 1;
            } else {
                // lets see if its yellow
                if chars[*gc as usize - 97] > 0 {
                    res += 1 * 3u32.pow(i as u32);
                    chars[*gc as usize - 97] -= 1;
                }
            }
            // its gray so no need to do anything
        }
        if res != mask {
            answers.remove(i);
        }
    }
}

fn sanity(mask: u32, guess: &str, ans: &str) -> u32 {
    let mut res = 0;
    let mut chars = [0; 26];
    for c in ans.as_bytes() {
        chars[(c - 97) as usize] += 1;
    }
    for (i, (gc, ac)) in guess
        .as_bytes()
        .iter()
        .zip(ans.as_bytes().iter())
        .enumerate()
    {
        if gc == ac {
            // its green
            print!("🟩 ");
            res += 2 * 3u32.pow(i as u32);
            chars[*gc as usize - 97] -= 1;
        } else {
            // lets see if its yellow
            if chars[*gc as usize - 97] > 0 {
                print!("🟨 ");
                res += 1 * 3u32.pow(i as u32);
                chars[*gc as usize - 97] -= 1;
            } else {
                print!("⬜ ");
            }
        }
        // its gray so no need to do anything
    }
    println!("got: {}, {}", res, mask);
    res
}

pub fn make_mask(inp: &str) -> u32 {
    println!("mask: {}", inp);
    let mut ret = 0;
    let mut i = 1;
    for c in inp.as_bytes().iter() {
        if *c == 10 {
            return ret;
        }
        let last = (c - 48) as u32;
        ret += last * i;
        i *= 3;

        if last == 0 {
            print!("⬜ ");
        } else if last == 1 {
            print!("🟨 ");
        } else {
            print!("🟩 ");
        }
    }
    println!("");
    ret
}

pub fn play_mask(guess: &str, ans: &str) -> u32 {
    let mut res = 0;
    let mut chars = [0; 26];
    for c in ans.as_bytes() {
        chars[(c - 97) as usize] += 1;
    }
    for (i, (gc, ac)) in guess
        .as_bytes()
        .iter()
        .zip(ans.as_bytes().iter())
        .enumerate()
    {
        if gc == ac {
            // its green
            res += 2 * 3u32.pow(i as u32);
            chars[*gc as usize - 97] -= 1;
        } else {
            // lets see if its yellow
            if chars[*gc as usize - 97] > 0 {
                res += 1 * 3u32.pow(i as u32);
                chars[*gc as usize - 97] -= 1;
            }
        }
        // its gray so no need to do anything
    }
    res
}
