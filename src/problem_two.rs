use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() as i32 - 1;
    let mut result = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid as usize] == target {
            result = Some(mid as usize);
            high = mid - 1; 
        } else if arr[mid as usize] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

pub fn main() {

    println!("2. Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.");
    println!("Enter the elements of the sorted array separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();

    println!("Enter the target number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target: i32 = input.trim().parse().expect("Not an integer!");

    match find_first_occurrence(&arr, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
