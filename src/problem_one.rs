pub fn check_palindrome() {
    let mut input = String::new();
    println!("1.
    A function that checks whether a given string is a palindrome or not.");
    println!("Enter a string: ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut chars: Vec<char> = input.chars().collect();

    if is_palindrome(&chars) {
        println!("{} is a palindrome", input);
    } else {
        println!("{} is not a palindrome", input);
    }
}

fn is_palindrome(input: &Vec<char>) -> bool {

    let mut i = 0;

    while i < (input.len() / 2) {

        if input[i] != input[input.len() - 1 - i] { return false; }

        i += 1;

    }

    true
}