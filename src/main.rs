use rand::Rng;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader};

fn main() {
    println!("Training text file:\n");
    let mut file_name = String::new();

    io::stdin()
        .read_line(&mut file_name)
        .expect("Error reading file");
    file_name = file_name.trim().to_string();

    if file_name.is_empty() {
        file_name = String::from("training_data/adventure_time_transcripts_speech.txt");
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

    #[derive(Debug)]
    enum Token {
        Speaker(String),
        StopWord(String),
        Punctuation(char),
        Word(String),
    }
    use Token::*;

    //type WordMatrix = HashMap<String, HashMap<String, u32>>;
    //let mut matrix: HashMap<String, WordMatrix> = HashMap::new();
    let mut matrix: HashMap<String, HashMap<String, u32>> = HashMap::new();

    let token_length = 1;

    //Words that exist for sentence structure but don't
    //convey ideas

    let stop_words = [
        "and", "the", "is", "are", "to", "of", "a", "an", "in", "for", "on", "but", "that", "it",
        "as",
    ];
    //outer key is previous word, inner key is stop word
    let mut stop_phrase_matrix: HashMap<String, HashMap<String, u32>> = HashMap::new();

    let mut tokens = Vec::new();
    let mut expr = String::new();

    let mut end_word = |tokens: &mut Vec<Token>, expr: &mut String| {
        if stop_words.contains(&expr.as_str()) {
            tokens.push(StopWord(expr.clone()));
        } else {
            tokens.push(Word(expr.clone()));
        }
        *expr = String::new();
    };

    for c in text.chars() {
        match c {
            _ if c.is_alphabetic() => {
                expr.extend(c.to_lowercase());
            }
            _ if c.is_numeric() || c == '\'' => {}
            '.' | '?' | ',' | '!' | '\n' => {
                if matches!(tokens.last(), Some(Punctuation(_))) {
                    continue;
                }
                if !expr.is_empty() {
                    end_word(&mut tokens, &mut expr);
                }
                tokens.push(Punctuation(c));
            }
            _ => {
                if !expr.is_empty() {
                    end_word(&mut tokens, &mut expr);
                }
            }
        }
    }

    let tokens_itr = tokens.iter().peekable();
    for (i, tok) in tokens_itr.enumerate() {
        println!("{:?}", tok);
        if i == 1000 {
            break;
        }
    }

    return;

    /*
    let mut last_token = String::new();
    for _ in 0..token_length {
        last_token += &tokens_itr.next().unwrap().to_string();
    }

    let mut curr_token = String::new();

    let mut speaker = String::new();
    let mut iter = 0;

    while let Some(expr) = tokens_itr.next() {
        if stop_words.contains(&expr.as_str()) {
            //let mut sequence = expr.clone();
            //Uncommenting will make it keep collecting
            //stop words into a sequence

            while let Some(next_word) = tokens_itr.peek() {
                let mut sequence = String::new();

                if stop_words.contains(&next_word.as_str()) {
                    sequence += next_word;
                    tokens_itr.next();
                } else {
                    stop_phrase_matrix
                        .entry(expr.to_string())
                        .or_insert_with(HashMap::new)
                        .entry(sequence.to_string())
                        .and_modify(|count| *count += 1)
                        .or_insert(1);

                    break;
                }
            }
        } else {
            curr_token += &expr;

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
        }

        iter += 1;
    }

    println!("Matrix length: {}\n\n", matrix.len());

    let mut rng = rand::thread_rng();

    let mut token = matrix
        .keys()
        .nth(rng.gen_range(0..matrix.len() - 1))
        .unwrap()
        .to_string();

    //Generate text
    for _ in 0..(num_phrases / token_length) {
        //print!("{expr} ");

        let mut max_entry_cnt = 0;
        let mut next_token = String::new();

        if !matrix.contains_key(&token) {
            token = matrix
                .keys()
                .nth(rng.gen_range(0..matrix.len() - 1))
                .unwrap()
                .to_string();
            continue;
        }

        for (entry_string, count) in &matrix[&token] {
            if *count > max_entry_cnt {
                max_entry_cnt = *count;
                next_token = entry_string.to_string();
            }
        }

        let mut token_part_itr = next_token.split_whitespace();

        while let Some(expr) = token_part_itr.next() {
            print!("{expr} ");

            if stop_phrase_matrix.contains_key(expr) {
                //Find most common stop word that occurs
                //after given expression
                let mut max_stop_entry_cnt = 0;
                let mut stop_phrase = String::new();

                for (entry_string, count) in &stop_phrase_matrix[expr] {
                    if *count > max_entry_cnt {
                        max_stop_entry_cnt = *count;
                        stop_phrase = entry_string.to_string();
                    }
                }

                //More often than not, stop word occurs before next phrase
                if max_stop_entry_cnt * 2 > max_entry_cnt {
                    print!("{stop_phrase} ");
                }
            }
        }

        if matrix[&token].contains_key(&next_token) {
            *matrix
                .get_mut(&token)
                .unwrap()
                .get_mut(&next_token)
                .unwrap() = 0;
        }
        //matrix.get_mut(&token).unwrap().get_mut(&next_token).unwrap() = 0;
        token = next_token;
    }
    */

    println!();
}
