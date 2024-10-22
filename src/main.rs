use rand::Rng;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Hash)]
enum Token {
    Speaker(String),
    StopWord(String),
    Punctuation(char),
    Word(String),
}
use Token::*;

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

    let tokens = lex(&text);
    let mut speakers_matrix = train(&tokens);
    //let output = gen_text(speakers_matrix, 1000);

    for (k, _) in speakers_matrix.iter() {
        println!("{}", k);
    }
    println!("Size: {}", speakers_matrix.len());

    return;

    let tokens_itr = tokens.iter().peekable();
    for (i, tok) in tokens_itr.enumerate() {
        println!("{:?}", tok);
        if i == 1000 {
            break;
        }
    }

    let mut matrix: HashMap<String, HashMap<String, u32>> = HashMap::new();
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
    */

    /*
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

fn lex(text: &str) -> Vec<Token> {
    //type WordMatrix = HashMap<String, HashMap<String, u32>>;
    //let mut matrix: HashMap<String, WordMatrix> = HashMap::new();

    let token_length = 1;

    //Words that exist for sentence structure but don't
    //convey ideas

    let stop_words = [
        "and", "the", "is", "are", "to", "of", "a", "an", "in", "for", "on", "but", "that", "it",
        "as",
    ];
    //outer key is previous word, inner key is stop word

    let mut tokens = Vec::new();
    let mut expr = String::new();
    //combine stop words to form phrases

    let mut end_word = |tokens: &mut Vec<Token>, expr: &mut String| {
        if expr.is_empty() {
            return;
        }

        if stop_words.contains(&expr.as_str()) {
            match tokens.last_mut().unwrap() {
                StopWord(sw) => *sw += &(" ".to_owned() + expr),

                _ => {
                    tokens.push(StopWord(expr.clone()));
                }
            }
        } else {
            tokens.push(Word(expr.clone()));
        }
        *expr = String::new();
    };

    let mut text_itr = text.chars();
    for c in text_itr {
        match c {
            _ if c.is_alphabetic() => {
                expr.extend(c.to_lowercase());
            }
            _ if c.is_numeric() || c == '\'' => {}
            '.' | '?' | ',' | '!' => {
                if matches!(tokens.last(), Some(Punctuation(_))) {
                    continue;
                }
                end_word(&mut tokens, &mut expr);
                tokens.push(Punctuation(c));
            }
            '\n' => {
                end_word(&mut tokens, &mut expr);
                let mut speaker = String::new();
            }
            _ => {
                end_word(&mut tokens, &mut expr);
            }
        }
    }

    tokens
}

type MATRIX = HashMap<String, HashMap<String, u32>>;
//Map<Speaker, Map<String, Count>>>

//enum Token {
//    Speaker(String),
//    StopWord(String),
//    Punctuation(char),
//    Word(String),
//}

//HashMap<Speaker, (word_matrix, stop_word_matrix, punct_matrix)>
struct Speaker {
    pub words: MATRIX,
    pub stop_words: MATRIX,
    pub punct: MATRIX,
}

impl Speaker {
    fn new() -> Speaker {
        Speaker {
            words: HashMap::new(),
            stop_words: HashMap::new(),
            punct: HashMap::new(),
        }
    }
}

fn train(tokens: &Vec<Token>) -> HashMap<String, Speaker> {
    let mut speakers: HashMap<String, Speaker> = HashMap::new();

    let mut tokens_itr = tokens.iter();
    let mut prev_word = String::new();
    while let Some(tok) = tokens_itr.next() {
        match tok {
            Word(w) => {
                prev_word = w.clone();
                break;
            }
            _ => {}
        }
    }

    let mut speaker = "";

    for token in tokens.iter() {
        let mut speaker_data = speakers
            .entry(speaker.to_lowercase())
            .or_insert_with(Speaker::new);
        match token {
            Word(word) => {
                speaker_data
                    .words
                    .entry(prev_word.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(word.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                prev_word = word.clone();
            }
            StopWord(word) => {
                speaker_data
                    .stop_words
                    .entry(prev_word.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(word.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            Punctuation(c) => {
                speaker_data
                    .punct
                    .entry(prev_word.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(c.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            Speaker(new_speaker) => {
                speaker = new_speaker;
            }
        }
    }

    speakers
}

fn gen_text(speakers: HashMap<String, Speaker>, output_len: u64) -> String {
    const SPEAKER_WORDS: u64 = 10;
    let mut rng = rand::thread_rng();

    let mut output = String::new();
    let mut word_count = 0;

    while word_count < output_len {
        let speaker_name = speakers
            .keys()
            .nth(rng.gen_range(0..speakers.len() - 1))
            .unwrap();

        //String manipulation in rust sucks
        output += speaker_name.to_uppercase().as_str();
        output += ": ";

        let speaker = speakers.get(speaker_name).unwrap();
        let mut sentences = 0;

        let mut prev_word = speaker
            .words
            .keys()
            .nth(rng.gen_range(0..speaker.words.len() - 1))
            .unwrap();
        output += prev_word;

        while sentences < SPEAKER_WORDS {
            //this is gonna be real slow lul
            let max_word = speaker
                .words
                .get(prev_word)
                .unwrap()
                .iter()
                .max_by_key(|&(_, value)| value)
                .unwrap();
            let max_stop_word = speaker
                .stop_words
                .get(prev_word)
                .unwrap()
                .iter()
                .max_by_key(|&(_, value)| value)
                .unwrap();
            let max_punct = speaker
                .punct
                .get(prev_word)
                .unwrap()
                .iter()
                .max_by_key(|&(_, value)| value)
                .unwrap();

            if max_word.1 >= max_stop_word.1 {
                output += max_word.0;
            } else {
                output += max_stop_word.0;
            }

            prev_word = max_word.0;
            word_count += 1;
        }
    }

    output
}
