use std::collections::HashSet;

mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec();

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

fn part1(data: &Vec<String>) {
    // Get all priorities in the form of characters.
    let mut priorities: Vec<char> = vec![];
    for line in data {
        let first_compartment: HashSet<char> = HashSet::from_iter(line[0..line.len() / 2].chars());

        for c in line[line.len() / 2..].chars() {
            if first_compartment.contains(&c) {
                priorities.push(c);
                break;
            }
        }
    }

    get_priority_values(&priorities);
}

fn part2(data: &Vec<String>) {
    let mut groups: Vec<HashSet<char>> = vec![];
    let mut priorities: Vec<char> = vec![];
    for line in data {
        // Get set of three rucksacks.
        groups.push(HashSet::from_iter(line.chars()));

        if groups.len() == 3 {
            // Get priority.
            for c in groups[0].iter() {
                if groups[1].contains(c) && groups[2].contains(c) {
                    priorities.push(*c);
                    break;
                }
            }
            // Rusty version... Ugly...
            // let common = groups[0]
            //     .intersection(&groups[1])
            //     .map(|x| x.to_owned())
            //     .collect::<HashSet<char>>()
            //     .intersection(&groups[2])
            //     .map(|x| x.to_owned())
            //     .collect::<HashSet<char>>();
            // for c in common {
            //     priorities.push(c);
            // }

            groups.clear()
        }
    }

    get_priority_values(&priorities);
}
