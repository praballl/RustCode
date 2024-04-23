use std::io;

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);

    merged
}

pub fn main() {
    let mut input = String::new();
    
    println!("11. Merge two sorted arrays in Rust.");
    println!("Enter the elements of the first sorted array separated by spaces:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr1: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    
    input.clear();
    
    println!("Enter the elements of the second sorted array separated by spaces:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr2: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);
}
