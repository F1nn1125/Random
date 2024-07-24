use rand::Rng;
use std::io;

fn main() {
    println!("What is your question? ");

    let possible_answers: [&str; 20] = ["It is certain", "It is decidedly so", "Without a doubt", "Yes definitely", "You may rely on it", "As I see it, yes", "Most likely", "Outlook good", "Yes Signs point to yes", "Reply hazy", "Try again", "Ask again later", "Better not tell you that", "Cannot predict now", "Concentrate and ask again", "Dont count on it", "My reply is no", "My sources say no", "Outlook not so good", "Very doubtful"];
    
    let answer_index: usize = rand::thread_rng().gen_range(0..possible_answers.len());
    let answer: &str = possible_answers[answer_index];

    let mut question: String = String::new();
    io::stdin()
        .read_line(&mut question)
        .expect("Failed to read line");

    println!("{answer}");

    println!("\nPress any key to exit");

    io::stdin()
        .read_line(&mut question)
        .expect("Failed to read line");
}