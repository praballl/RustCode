use std::io;

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let mut prefix = String::new();
    let first_str = &strings[0];

    'outer: for (i, char) in first_str.chars().enumerate() {
        for string in strings.iter().skip(1) {
            if let Some(c) = string.chars().nth(i) {
                if c != char {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(char);
    }
    
    if prefix.is_empty() {
        "No prefix".to_string()
    } else {
        prefix
    }
}

pub fn main() {
    println!("6. Implement a function that finds the longest common prefix of a given set of strings.");
    println!("Enter strings separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let strings: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

    println!("Longest Common Prefix: {}", longest_common_prefix(&strings));
}

