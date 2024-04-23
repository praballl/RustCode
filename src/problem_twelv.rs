use std::io;

fn max_subarray_sum(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        panic!("Array is empty!");
    }

    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in arr.iter().skip(1) {
        current_sum = current_sum.max(current_sum + num);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

pub fn main() {
    println!("Find the maximum subarray sum in Rust");
    println!("Enter the elements of the array separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
