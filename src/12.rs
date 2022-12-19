use std::collections::{HashSet, VecDeque};

mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    for (i, line) in data.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'S' {
                start_pos = (i, j);
            } else if c == &'E' {
                end_pos = (i, j);
            }
        }
    }
    find_shortest_path(data.clone(), start_pos);
    let mut new_data = data.clone();
    for (i, line) in data.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'S' {
                new_data[start_pos.0][start_pos.1] = 'a';
            } else if c == &'E' {
                new_data[end_pos.0][end_pos.1] = 'z';
            }
        }
    }
    find_shortest_path_2(new_data.clone(), end_pos);
}

fn find_shortest_path(data: Vec<Vec<char>>, start_pos: (usize, usize)) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    q.push_back(start_pos);
    seen.insert(start_pos);
    let mut moves = 0;
    loop {
        let mut q_size = q.len();
        while q_size > 0 {
            let p = q.pop_front().unwrap();
            if data[p.0][p.1] == 'E' {
                println!("End found: ({}, {}). Number of moves: {}", p.0, p.1, moves);
                return;
            }

            if p.0 > 0
                && !seen.contains(&(p.0 - 1, p.1))
                && (p == start_pos
                    || (data[p.0 - 1][p.1] == 'E' && data[p.0][p.1] == 'z')
                    || (data[p.0 - 1][p.1] != 'E'
                        && data[p.0][p.1].to_digit(36).unwrap() as i32
                            - data[p.0 - 1][p.1].to_digit(36).unwrap() as i32
                            >= -1))
            {
                q.push_back((p.0 - 1, p.1));
                seen.insert((p.0 - 1, p.1));
            }
            if p.0 < data.len() - 1
                && !seen.contains(&(p.0 + 1, p.1))
                && (p == start_pos
                    || (data[p.0 + 1][p.1] == 'E' && data[p.0][p.1] == 'z')
                    || (data[p.0 + 1][p.1] != 'E'
                        && data[p.0][p.1].to_digit(36).unwrap() as i32
                            - data[p.0 + 1][p.1].to_digit(36).unwrap() as i32
                            >= -1))
            {
                q.push_back((p.0 + 1, p.1));
                seen.insert((p.0 + 1, p.1));
            }
            if p.1 > 0
                && !seen.contains(&(p.0, p.1 - 1))
                && (p == start_pos
                    || (data[p.0][p.1 - 1] == 'E' && data[p.0][p.1] == 'z')
                    || (data[p.0][p.1 - 1] != 'E'
                        && data[p.0][p.1].to_digit(36).unwrap() as i32
                            - data[p.0][p.1 - 1].to_digit(36).unwrap() as i32
                            >= -1))
            {
                q.push_back((p.0, p.1 - 1));
                seen.insert((p.0, p.1 - 1));
            }
            if p.1 < data[0].len() - 1
                && !seen.contains(&(p.0, p.1 + 1))
                && (p == start_pos
                    || (data[p.0][p.1 + 1] == 'E' && data[p.0][p.1] == 'z')
                    || (data[p.0][p.1 + 1] != 'E'
                        && data[p.0][p.1].to_digit(36).unwrap() as i32
                            - data[p.0][p.1 + 1].to_digit(36).unwrap() as i32
                            >= -1))
            {
                q.push_back((p.0, p.1 + 1));
                seen.insert((p.0, p.1 + 1));
            }

            q_size -= 1;
        }
        moves += 1;
    }
}

fn find_shortest_path_2(data: Vec<Vec<char>>, start_pos: (usize, usize)) -> usize {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    q.push_back(start_pos);
    seen.insert(start_pos);
    let mut moves = 0;
    loop {
        // println!("move: {moves}");
        let mut q_size = q.len();
        while q_size > 0 {
            let p = q.pop_front().unwrap();
            // println!("cur: ({}, {}) {}", p.0, p.1, data[p.0][p.1]);
            if data[p.0][p.1] == 'a' {
                println!("End found: ({}, {}). Number of moves: {}", p.0, p.1, moves);
                return moves;
            }

            if p.0 > 0
                && !seen.contains(&(p.0 - 1, p.1))
                && (data[p.0][p.1].to_digit(36).unwrap() as i32
                    - data[p.0 - 1][p.1].to_digit(36).unwrap() as i32
                    <= 1)
            {
                q.push_back((p.0 - 1, p.1));
                seen.insert((p.0 - 1, p.1));
            }
            if p.0 < data.len() - 1
                && !seen.contains(&(p.0 + 1, p.1))
                && (data[p.0][p.1].to_digit(36).unwrap() as i32
                    - data[p.0 + 1][p.1].to_digit(36).unwrap() as i32
                    <= 1)
            {
                q.push_back((p.0 + 1, p.1));
                seen.insert((p.0 + 1, p.1));
            }
            if p.1 > 0
                && !seen.contains(&(p.0, p.1 - 1))
                && (data[p.0][p.1].to_digit(36).unwrap() as i32
                    - data[p.0][p.1 - 1].to_digit(36).unwrap() as i32
                    <= 1)
            {
                q.push_back((p.0, p.1 - 1));
                seen.insert((p.0, p.1 - 1));
            }
            if p.1 < data[0].len() - 1
                && !seen.contains(&(p.0, p.1 + 1))
                && (data[p.0][p.1].to_digit(36).unwrap() as i32
                    - data[p.0][p.1 + 1].to_digit(36).unwrap() as i32
                    <= 1)
            {
                q.push_back((p.0, p.1 + 1));
                seen.insert((p.0, p.1 + 1));
            }

            q_size -= 1;
        }
        moves += 1;
    }
}
