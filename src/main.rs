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
    let output = gen_text(speakers_matrix, 1000);

    println!("{output}");
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
    while let Some(c) = text_itr.next() {
        match c {
            _ if c.is_ascii_alphabetic() => {
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
                let speaker = text_itr
                    .by_ref()
                    .take_while(|&c| c != ':')
                    .filter(|&c| c.is_alphabetic())
                    .collect::<String>()
                    .to_lowercase();

                tokens.push(Speaker(speaker));
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
        //println!("{}", speakers.len());
        let speaker_name = speakers
            .keys()
            .nth(rng.gen_range(0..speakers.len() - 1))
            .unwrap();

        let speaker = speakers.get(speaker_name).unwrap();
        let mut speaker_words = 0;

        const MIN_DATA_LEN: usize = 500;

        if speaker.words.len() < MIN_DATA_LEN {
            continue;
        }
        //String manipulation in rust sucks
        output += speaker_name.to_uppercase().as_str();
        output += ": ";
        let mut prev_word = speaker
            .words
            .keys()
            .nth(rng.gen_range(0..speaker.words.len() - 1))
            .unwrap();
        output += prev_word;
        output += " ";

        while speaker_words < SPEAKER_WORDS {
            //this is gonna be real slow lul

            let max_word = if let Some(next) = speaker.words.get(prev_word) {
                next.iter().max_by_key(|&(_, value)| value).unwrap()
            } else {
                //fix later lol
                break;
                /*speaker
                .words
                .values()
                .next()
                .unwrap()
                .iter()
                .nth(rng.gen_range(0..speaker.words.len() - 1))
                .unwrap()
                */
            };

            let max_stop_word = if let Some(next) = speaker.stop_words.get(prev_word) {
                next.iter().max_by_key(|&(_, value)| value).unwrap()
            } else {
                (&String::new(), &0)
            };

            let max_stop_word = if let Some(next) = speaker.stop_words.get(prev_word) {
                next.iter().max_by_key(|&(_, value)| value).unwrap()
            } else {
                (&String::new(), &0)
            };

            let max_punct = if let Some(next) = speaker.punct.get(prev_word) {
                next.iter().max_by_key(|&(_, value)| value).unwrap()
            } else {
                (&String::new(), &0)
            };

            if max_word.1 >= max_stop_word.1 {
                output += max_word.0;
            } else {
                output += max_stop_word.0;
            }

            output += " ";

            prev_word = max_word.0;
            word_count += 1;
            speaker_words += 1;
        }

        output += "\n";
    }

    output
}
