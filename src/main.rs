
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use rand::Rng;


fn main() {

    println!("Training text file:\n");
    let mut file_name = String::new();

    io::stdin().read_line(&mut file_name).expect("Error reading file");
    file_name = file_name.trim().to_string();

    if file_name.is_empty() {
        file_name = String::from("adventure_time_transcripts_speech.txt");
    }

    println!("Opening file {file_name}");

    println!("Number of phrases to output:\n");
    let mut num_phrases = String::new();
    io::stdin().read_line(&mut num_phrases);
    num_phrases = num_phrases.trim().to_string();

    if num_phrases.is_empty() {
        num_phrases = String::from("1000");
    }

    let num_phrases: u64 = num_phrases.parse().unwrap();


    let text = fs::read_to_string(file_name).unwrap();

    enum Token {
        StopWord,
        Word(String),
    }

    let mut matrix: HashMap<String, HashMap<String, u32>> = HashMap::new();

    let token_length = 3;

    //Words that exist for sentence structure but don't
    //convey ideas

    let stop_words = [
        "fjkdaf",//"and", "the", "is", "are", "to", "of", "a", "an", "in", "for", "on", "but", "that", "it", "as"
    ];
    //outer key is previous word, inner key is stop word
    let mut stop_phrase_matrix: HashMap<String, HashMap<String, u32>> = HashMap::new();

    //Train the matrix
    let mut expressions_itr =
        text.split_whitespace()
        .map(|s| s.to_lowercase()
            .chars()
            .filter(|c| !c.is_ascii_punctuation() && !c.is_whitespace())
            .collect::<String>());


    let mut last_token = String::new();
    for _ in 0..token_length {
        last_token += expressions_itr.next().unwrap().as_str();
    }

    let mut curr_token = String::new();

    let mut iter = 0;

    while let Some(expr) = expressions_itr.next() {
        /*
        if stop_words.contains(&expr.as_str()) {
            //let mut sequence = expr.clone();
            //Uncommenting will make it keep collecting
            //stop words into a sequence
            while let Some(next_word) = expressions_itr.next() {

                if stop_words.contains(&next_word.as_str()) {
                    sequence = sequence + next_word.as_str();
                } else {
                    stop_phrase_matrix
                        .entry(last_expr.to_string())
                        .or_insert_with(HashMap::new)
                        .entry(sequence.to_string())
                        .and_modify(|count| *count += 1)
                        .or_insert(1);

                    last_expr = expr;
                    break;
                }
            }
        } else {*/
            curr_token += expr.as_str();

            if iter % token_length == 0 {
                matrix
                    .entry(last_token.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(curr_token.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                    last_token = curr_token.to_string();
                    curr_token.clear();

            } else {
                curr_token += " ";
            }
        //}
        iter += 1;
    }


    println!("Matrix length: {}\n\n", matrix.len());

    let mut rng = rand::thread_rng();

    let mut expr = matrix.keys().nth(rng.gen_range(0..matrix.len()-1)).unwrap().to_string();

    //Generate text
    for _ in 0..num_phrases {
        print!("{expr} ");

        let mut max_entry_cnt = 0;
        let mut next_expr = String::new();

        if !matrix.contains_key(&expr) {
            expr = matrix.keys().nth(rng.gen_range(0..matrix.len()-1)).unwrap().to_string();
            continue;
        }

        for (entry_string, count) in &matrix[&expr] {
            if *count > max_entry_cnt {
                max_entry_cnt = *count;
                next_expr = entry_string.to_string();
            }
        }



        if stop_phrase_matrix.contains_key(&expr) {
            //Find most common stop word that occurs
            //after given expression
            let mut max_stop_entry_cnt = 0;
            let mut stop_phrase = String::new();

            for (entry_string, count) in &stop_phrase_matrix[&expr] {
                if *count > max_entry_cnt {
                    max_stop_entry_cnt = *count;
                    stop_phrase = entry_string.to_string();
                }
            }

            //More often than not, stop word occurs before next phrase
            if max_stop_entry_cnt*2 > max_entry_cnt {
                print!("{stop_phrase} ");
            }
        }

        *matrix.get_mut(&expr).unwrap().get_mut(&next_expr).unwrap() = 0;
        expr = next_expr;
    }
}