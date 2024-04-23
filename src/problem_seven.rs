use std::io;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k - 1])
}

pub fn main() {
    println!("7. Implement a function that returns the kth smallest element in a given array.");
    println!("Enter the elements of the array separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");

    // Attempt to parse k_input into a usize
    let k: usize = match k_input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid input for k, please enter a valid number.");
            return;
        }
    };

    match kth_smallest(&arr, k) {
        Some(val) => println!("The {}th smallest element is: {}", k, val),
        None => println!("Invalid input!"),
    }
}
