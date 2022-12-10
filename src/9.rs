use std::cmp::*;
use std::collections::HashSet;

mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    simulate_tail_visits(&data, 2);
    simulate_tail_visits(&data, 10);
}

fn simulate_tail_visits(data: &Vec<Vec<String>>, number_of_points: usize) -> HashSet<(i32, i32)> {
    let mut p: Vec<(i32, i32)> = vec![(0, 0); number_of_points];
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    for line in data {
        let (d, n) = (line[0].clone(), line[1].parse::<i32>().unwrap());
        for _ in 0..n {
            let c = match d.as_str() {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => panic!("Unexpected direction!"),
            };
            p[0].0 += c.0;
            p[0].1 += c.1;

            for i in 0..p.len() - 1 {
                let deltas = (p[i].0 - p[i + 1].0, p[i].1 - p[i + 1].1);
                if deltas.0.abs() > 1 || deltas.1.abs() > 1 {
                    let dx = match deltas.0.cmp(&0) {
                        Ordering::Greater => 1,
                        Ordering::Equal => 0,
                        Ordering::Less => -1,
                    };
                    let dy = match deltas.1.cmp(&0) {
                        Ordering::Greater => 1,
                        Ordering::Equal => 0,
                        Ordering::Less => -1,
                    };
                    p[i + 1].0 += dx;
                    p[i + 1].1 += dy;
                }
            }
            seen.insert((p.last().unwrap().0 as i32, p.last().unwrap().1 as i32));
        }
    }

    println!(
        "Positions that tail {} visits at least once: {}",
        number_of_points - 1,
        seen.len()
    );
    seen
}
