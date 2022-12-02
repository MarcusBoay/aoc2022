use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if env::args().len() != 2 {
        panic!("Usage: {} path/to/file", &args[0]);
    }

    let filepath = &args[1];
    let data = fs::read_to_string(filepath)
        .unwrap_or_else(|_| panic!("Unable to read file {}.", filepath.to_owned()));

    // Collect food for each elf.
    // elves[0] = total calories of food carried by 1st elf.
    let mut elves: Vec<i64> = vec![];
    let mut cur_amt = 0;
    for line in data.split('\n') {
        println!("{line}");
        if line.is_empty() {
            println!("{cur_amt}");
            elves.push(cur_amt);
            cur_amt = 0;
        } else {
            cur_amt += line
                .parse::<i64>()
                .expect("Cannot parse line {line} as i64.");
        }
    }
    if cur_amt > 0 {
        elves.push(cur_amt);
    }

    // print_elves(&elves);
    // print_max_elf(&elves);
    print_top_three_elf(&elves);
}

fn print_elves(elves: &[i64]) {
    for (i, cal) in elves.iter().enumerate() {
        println!("Elf {} carries {cal} calories.", i + 1);
    }
}

fn print_max_elf(elves: &[i64]) {
    let mut cur_max = 0;
    let mut elf = 0;
    for (i, &cal) in elves.iter().enumerate() {
        if cal > cur_max {
            cur_max = std::cmp::max(cur_max, cal);
            elf = i + 1;
        }
    }
    println!("Elf {elf} carries the most at {cur_max} calories.");
}

fn print_top_three_elf(elves: &[i64]) {
    let mut elves = elves.to_owned().clone();
    elves.sort();
    elves.reverse();
    let mut total_calories = 0;
    for i in 0..3 {
        println!("Top {} elf is carrying {} calories.", i + 1, elves[i]);
        total_calories += elves[i];
    }
    println!("Total calories carried by the top 3 elves: {total_calories}");
}
