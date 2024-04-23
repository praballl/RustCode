use std::io;

fn reverse_string(input: &str) -> String {
    let mut reversed = String::new();
    for c in input.chars().rev() {
        reversed.push(c);
    }
    reversed
}

pub fn main() {
    println!("9. Reverse a string in Rust");
    println!("Enter a string to reverse:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let reversed = reverse_string(input.trim());

    println!("Original string: {}", input.trim());
    println!("Reversed string: {}", reversed);
}
