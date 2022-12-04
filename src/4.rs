use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if env::args().len() != 2 {
        panic!("Usage: {} path/to/file", &args[0]);
    }

    let filepath = &args[1];
    let data = fs::read_to_string(filepath)
        .unwrap_or_else(|_| panic!("Unable to read file {}.", filepath.to_owned()));

    let mut contained_pairs = 0;
    let mut overlapping_pairs = 0;
    for line in data.trim().split('\n').collect::<Vec<&str>>() {
        let pairs = line
            .split(',')
            .map(|x| {
                x.split('-')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        contained_pairs += is_contained(&pairs[0], &pairs[1]);
        overlapping_pairs += is_overlapping(&pairs[0], &pairs[1]);
    }
    println!("Total contained pairs: {contained_pairs}");
    println!("Total overlapping pairs: {overlapping_pairs}");
}

fn is_contained(pair1: &[i32], pair2: &[i32]) -> i32 {
    (pair1[0] == pair2[0]
        || pair1[1] == pair2[1]
        || (pair1[0] > pair2[0] && pair1[1] <= pair2[1])
        || (pair2[0] > pair1[0] && pair2[1] <= pair1[1])) as i32
}

fn is_overlapping(pair1: &[i32], pair2: &[i32]) -> i32 {
    ((pair1[0] >= pair2[0] && pair1[0] <= pair2[1])
        || (pair2[0] >= pair1[0] && pair2[0] <= pair1[1])) as i32
}
