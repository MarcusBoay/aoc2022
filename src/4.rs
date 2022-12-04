use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if env::args().len() != 2 {
        panic!("Usage: {} path/to/file", &args[0]);
    }

    let filepath = &args[1];
    let data = fs::read_to_string(filepath)
        .unwrap_or_else(|_| panic!("Unable to read file {}.", filepath.to_owned()));

    let mut total_contained_pairs = 0;
    let mut total_overlapping_pairs = 0;
    for line in data.split('\n').map(String::from).collect::<Vec<String>>() {
        // Final line.
        if line.is_empty() {
            break;
        }

        let pairs = line.split(',').map(String::from).collect::<Vec<String>>();
        let pair1 = pairs[0]
            .split('-')
            .map(|x| String::from(x).parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let pair2 = pairs[1]
            .split('-')
            .map(|x| String::from(x).parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        total_contained_pairs += is_contained(&pair1, &pair2);
        total_overlapping_pairs += is_overlapping(&pair1, &pair2);
    }
    println!("Total contained pairs: {total_contained_pairs}");
    println!("Total contained pairs: {total_overlapping_pairs}");
}

fn is_contained(pair1: &[i32], pair2: &[i32]) -> i32 {
    if pair1[0] == pair2[0]
        || pair1[1] == pair2[1]
        || (pair1[0] > pair2[0] && pair1[1] <= pair2[1])
        || (pair2[0] > pair1[0] && pair2[1] <= pair1[1])
    {
        return 1;
    }
    0
}

fn is_overlapping(pair1: &[i32], pair2: &[i32]) -> i32 {
    if (pair1[0] >= pair2[0] && pair1[0] <= pair2[1])
        || (pair2[0] >= pair1[0] && pair2[0] <= pair1[1])
    {
        return 1;
    }
    0
}
