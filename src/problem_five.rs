use std::io;

fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        return (arr[mid_left] + arr[mid_right]) as f64 / 2.0;
    } else {
        let mid = len / 2;
        return arr[mid] as f64;
    }
}

pub fn main() {
    println!("5. Given a sorted array of integers, implement a function that returns the median of the array.");
    println!("Enter the sorted array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();

    println!("Median: {}", find_median(&arr));
}
