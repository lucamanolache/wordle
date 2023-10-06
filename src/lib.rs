use argminmax::ArgMinMax;
use rayon::prelude::*;

pub fn calc_probs(words: &Vec<[u8; 5]>) -> ([u8; 5], f32) {
    // map of each guess to how many times it gets each key.
    // so bmap['soare'] = { 20002: 5, 10001: 3, ... }
    let len = words.len() as f32;
    let information: Vec<f32> = words
        .par_iter()
        .map(|guess| -> f32 {
            let mut mask_counts = [0.0; 243];
            let masks: Vec<usize> = words
                .par_iter() // big array so use parallism here
                .map(|other_word| -> usize { gen_mask(guess, other_word) })
                .collect();
            for mask in masks {
                // increase the amount of times we have seen this pattern
                mask_counts[mask] += 1.0;
            }
            // get the information from this
            mask_counts
                .iter() // array of 243 so no need to go parallel
                .filter_map(|p| -> Option<f32> {
                    if *p == 0.0 {
                        // log 0 is NaN, so we ignore 0s
                        None
                    } else {
                        // turn a probability into information
                        let p = p / len;
                        Some(p * -p.log2())
                    }
                })
                .sum()
        })
        .collect();
    // find what element has the best information
    let (_, argmax) = information.argminmax();
    (words[argmax].clone(), information[argmax])
}

#[inline]
fn gen_mask(guess: &[u8; 5], ans: &[u8; 5]) -> usize {
    let mut res = 0;
    let mut chars = [0; 26];
    for c in ans {
        chars[(c - 97) as usize] += 1;
    }
    let mut pow = 1;
    for i in 0..5 {
        let (gc, ac) = (guess[i], ans[i]);
        if gc == ac {
            // its green
            res += 2 * pow;
            chars[gc as usize - 97] -= 1;
        } else if chars[gc as usize - 97] > 0 {
            // it's yellow
            res += pow;
            chars[gc as usize - 97] -= 1;
        }
        // its gray so no need to do anything
        pow *= 3;
    }
    res
}

pub fn mask_answers(mask: u32, guess: &[u8; 5], answers: &mut Vec<[u8; 5]>) {
    for i in (0..answers.len() - 1).rev() {
        let ans = answers[i];
        if ans.len() != 5 {
            continue;
        }
        let mut res = 0;
        let mut chars = [0; 26];
        for c in ans {
            chars[(c - 97) as usize] += 1;
        }
        for (i, (gc, ac)) in guess.iter().zip(ans.iter()).enumerate() {
            if gc == ac {
                // its green
                res += 2 * 3u32.pow(i as u32);
                chars[*gc as usize - 97] -= 1;
            } else {
                // lets see if its yellow
                if chars[*gc as usize - 97] > 0 {
                    res += 3u32.pow(i as u32);
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
    println!();
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
                res += 3u32.pow(i as u32);
                chars[*gc as usize - 97] -= 1;
            }
        }
        // its gray so no need to do anything
    }
    res
}
