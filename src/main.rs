
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use rand::Rng;


fn main() {

    println!("Training text file:\n");
    let mut file_name = String::new();

    if file_name.trim().is_empty() {
        file_name = String::from("shakespeare_plays.txt");
    }

    io::stdin().read_line(&mut file_name).expect("Error reading file");
    println!("Opening file {file_name}");

    println!("Number of phrases to output:\n");
    let mut num_phrases = String::new();
    io::stdin().read_line(&mut num_phrases);

    if num_phrases.trim().is_empty() {
        num_phrases = String::from("100");
    }

    let num_phrases: u64 = num_phrases.trim().parse().unwrap();


    let file = File::open(file_name.trim()).unwrap();
    let reader = BufReader::new(file);

    let mut matrix: HashMap<String, HashMap<String, u32>> = HashMap::new();

    //Train the matrix
    for line in reader.lines() {

        let mut expressions_itr = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .into_iter();

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
        print!("{}{}", expr,
            if !expr.chars().last().unwrap().is_ascii_punctuation() { " " } else { "\n" }
        );

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