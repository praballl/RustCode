use std::io;

fn find_shortest_word(sentence: &str) -> Option<&str> {
    sentence.split_whitespace().min_by_key(|word| word.len())
}

pub fn main() {
    println!("3. Given a string of words, implement a function that returns the shortest word in the string.");
    println!("Enter a sentence:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match find_shortest_word(&input) {
        Some(shortest_word) => println!("Shortest word: {}", shortest_word),
        None => println!("No words found"),
    }
}
