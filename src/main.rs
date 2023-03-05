use std::io;

fn binary_search(arr: &[i32], key: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] == key {
            return Some(mid);
        } else if arr[mid] < key {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

fn main() {
    println!("Please enter a sorted array of integers (e.g. 1 3 5 7):");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("Please enter an integer to search for:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let key = input.trim().parse::<i32>().expect("Invalid input");

    match binary_search(&arr, key) {
        Some(index) => println!("Found {} at index {}", key, index),
        None => println!("{} not found", key),
    }
}
