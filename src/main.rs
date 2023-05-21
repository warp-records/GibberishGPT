
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use rand::Rng;


fn main() {

    println!("Training text file:\n");
    let mut file_name = String::new();

    io::stdin().read_line(&mut file_name).expect("Error reading file");
    file_name = file_name.trim().to_string();

    if file_name.is_empty() {
        file_name = String::from("shakespeare_plays.txt");
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


    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut matrix: HashMap<String, HashMap<String, u32>> = HashMap::new();

    //Words that exist for sentence structure but don't
    //convey ideas

    let stop_words = [
        "and", "the", "is", "are", "to", "of", "a", "an", "in", "for", "on", "but", "that", "it", "as"
    ];
    //Train the matrix
    for line in reader.lines() {

        let mut expressions_itr = line.unwrap();
        let mut expressions_itr =
            expressions_itr.split_whitespace()
            .map(|s| s.to_lowercase()
                .chars()
                .filter(|c| !c.is_ascii_punctuation() && !c.is_whitespace())
                .collect::<String>())
            .filter(|s| !stop_words.contains(&s.as_str()));

        let mut last_expr = match expressions_itr.next() {
            Some(expr) => expr.to_string(),
            None => { continue; },
        };

        for expr in expressions_itr {

            matrix
                .entry(last_expr.to_string())
                .or_insert_with(HashMap::new)
                .entry(expr.to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);

            last_expr = expr.to_string();
        }
    }


    //pretened there is other code here that populates the matrix

    println!("{}", matrix.len());

    let mut rng = rand::thread_rng();
    let mut expr = matrix.keys().nth(rng.gen_range(0..matrix.len()-1)).unwrap().to_string();

    //Generate text
    for _ in 0..num_phrases {
        print!("{expr} ");

        let mut max_entry_cnt = 0;
        let mut next_expr = String::new();

        for (entry_string, count) in &matrix[&expr] {
            if *count > max_entry_cnt {
                max_entry_cnt = *count;
                next_expr = entry_string.to_string();
            }
        }

        *matrix.get_mut(&expr).unwrap().get_mut(&next_expr).unwrap() = 0;
        expr = next_expr;
    }
}