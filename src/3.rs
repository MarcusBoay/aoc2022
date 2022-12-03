use std::{collections::HashSet, env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if env::args().len() != 2 {
        panic!("Usage: {} path/to/file", &args[0]);
    }

    let filepath = &args[1];
    let data = fs::read_to_string(filepath)
        .unwrap_or_else(|_| panic!("Unable to read file {}.", filepath.to_owned()));

    part1(&data);
    part2(&data);
}

/// Converts all characters to priority values.
fn get_priority_values(priorities: &Vec<char>) {
    let mut total_priorities = 0;
    for c in priorities {
        total_priorities += if c.is_ascii_lowercase() {
            c.to_digit(36).unwrap() - 9
        } else {
            c.to_digit(36).unwrap() - 9 + 26
        };
    }
    println!("Total priorities: {total_priorities}");
}

fn part1(data: &str) {
    // Get all priorities in the form of characters.
    let mut priorities: Vec<char> = vec![];
    for line in data.split('\n').map(String::from).collect::<Vec<String>>() {
        let mut first_compartment: HashSet<char> = HashSet::new();
        for (i, char) in line.chars().enumerate() {
            if i < line.bytes().len() / 2 {
                first_compartment.insert(char);
            } else if first_compartment.contains(&char) {
                priorities.push(char);
                break;
            }
        }
    }

    get_priority_values(&priorities);
}

fn part2(data: &str) {
    let mut groups: Vec<HashSet<char>> = vec![];
    let mut priorities: Vec<char> = vec![];
    for (i, line) in data
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>()
        .iter()
        .enumerate()
    {
        if groups.len() == 3 {
            // Get priority.
            for c in groups[0].iter() {
                if groups[1].contains(c) && groups[2].contains(c) {
                    priorities.push(*c);
                    break;
                }
            }

            // Reset groups.
            groups = vec![];
        }

        // Last line.
        if line.is_empty() {
            break;
        }

        // Get set of three rucksacks.
        groups.push(HashSet::new());
        for char in line.chars() {
            groups[i % 3].insert(char);
        }
    }

    get_priority_values(&priorities);
}
