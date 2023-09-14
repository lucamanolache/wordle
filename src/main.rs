use std::{
    collections::{HashMap, HashSet},
    fs, io,
    process::exit,
};

fn calc_probs(guesses: &Vec<&str>) -> (String, f32) {
    let mut bmap: HashMap<&str, HashMap<u32, u32>> = HashMap::new();
    let mut res_set: HashSet<u32> = HashSet::new();
    for guess in guesses.iter() {
        let mut gmap: HashMap<u32, u32> = HashMap::new();
        if guess.len() != 5 {
            continue;
        }
        for ans in guesses.iter() {
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
            gmap.insert(res, gmap.get(&res).unwrap_or(&0) + 1); // put this in our hashmap
            res_set.insert(res);
        }
        bmap.insert(guess, gmap);
    }
    let mut informations: HashMap<u32, f32> = HashMap::new();
    let totals = guesses.len() * guesses.len();
    for res in res_set {
        let mut total = 0;
        for prob_map in bmap.values() {
            total += prob_map.get(&res).unwrap_or(&0);
        }
        informations.insert(res, -(total as f32 / totals as f32).log2());
    }
    let (mut best, mut guess) = (0.0, "ERROR???");
    for (key, map) in bmap.iter() {
        // so total is just gonna be how many possible answers there are
        let mut expected_information = 0.0;
        for (key, times) in map {
            expected_information +=
                informations.get(key).unwrap_or(&0.0) * (*times as f32 / guesses.len() as f32);
        }
        if expected_information > best {
            best = expected_information;
            guess = key;
            // println!("switching guess to {} with entropy of {}", guess, best);
        }
    }
    (guess.to_owned(), best)
}

fn mask_answers(mask: u32, guess: &str, answers: &mut Vec<&str>) {
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

fn make_mask(inp: &str) -> u32 {
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

fn main() {
    // read words
    let fp = "allowed_words.txt";
    let contents = fs::read_to_string(fp).expect("Should have been able to read the file");

    let mut contents: Vec<&str> = contents.split('\n').collect();

    let mut buffer = String::new();
    for i in 0..6 {
        let (guess, entropy) = calc_probs(&contents);
        println!(
            "guess: {}. has a predicted information of {}. {} answers remain",
            guess,
            entropy,
            contents.len()
        );
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mask = make_mask(buffer.trim());
        mask_answers(mask, &guess.trim(), &mut contents);
    }
}
