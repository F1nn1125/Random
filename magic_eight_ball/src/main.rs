use rand::Rng;
use std::io;

fn main() {
    let possible_answers: [&str; 3] = ["yes", "no", "maybe"];
    
    let answer_index: usize = rand::thread_rng().gen_range(0..possible_answers.len());
    let answer: &str = possible_answers[answer_index];

    let mut question: String = String::new();
    io::stdin()
        .read_line(&mut question)
        .expect("Failed to read line");

    println!("{answer}");

    io::stdin()
        .read_line(&mut question)
        .expect("Failed to read line");
}