mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    part1(&data);
    part2(&data);
}

fn part1(data: &Vec<Vec<i32>>) {
    let mut visible_map = vec![vec![false; data[0].len()]; data.len()];

    // right
    let mut cur_tree: i32;
    for i in 0..data.len() {
        cur_tree = -1;
        for j in 0..data[i].len() {
            if cur_tree < data[i][j] {
                cur_tree = data[i][j];
                visible_map[i][j] = true;
            }
        }
    }
    // left
    for i in 0..data.len() {
        cur_tree = -1;
        for j in (0..data[i].len()).rev().step_by(1) {
            if cur_tree < data[i][j] {
                cur_tree = data[i][j];
                visible_map[i][j] = true;
            }
        }
    }
    // down
    for i in 0..data[0].len() {
        cur_tree = -1;
        for j in 0..data.len() {
            if cur_tree < data[j][i] {
                cur_tree = data[j][i];
                visible_map[j][i] = true;
            }
        }
    }
    // up
    for i in 0..data[0].len() {
        cur_tree = -1;
        for j in (0..data.len()).rev().step_by(1) {
            if cur_tree < data[j][i] {
                cur_tree = data[j][i];
                visible_map[j][i] = true;
            }
        }
    }

    let mut visible_count = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if visible_map[i][j] {
                visible_count += 1;
            }
        }
    }
    println!("Visible trees: {visible_count}");
}

fn part2(data: &Vec<Vec<i32>>) {
    let mut cur_best = 0;

    for i in 1..data.len() - 1 {
        for j in 1..data[i].len() - 1 {
            let mut views = [0, 0, 0, 0];
            // right
            let mut cur_tree = -1;
            for k in j + 1..data[i].len() {
                if data[i][k] >= data[i][j] {
                    views[0] += 1;
                    break;
                } else if cur_tree < data[i][k] {
                    views[0] += 1;
                }
            }
            // left
            cur_tree = -1;
            for k in (0..j).rev().step_by(1) {
                if data[i][k] >= data[i][j] {
                    views[1] += 1;
                    break;
                } else if cur_tree < data[i][k] {
                    views[1] += 1;
                }
            }
            // down
            cur_tree = -1;
            for k in i + 1..data[i].len() {
                if data[k][j] >= data[i][j] {
                    views[2] += 1;
                    break;
                } else if cur_tree < data[i][k] {
                    views[2] += 1;
                }
            }
            // up
            cur_tree = -1;
            for k in (0..i).rev().step_by(1) {
                if data[k][j] >= data[i][j] {
                    views[3] += 1;
                    break;
                } else if cur_tree < data[i][k] {
                    views[3] += 1;
                }
            }
            cur_best = std::cmp::max(cur_best, views[0] * views[1] * views[2] * views[3]);
        }
    }
    println!("Highest scenic score: {cur_best}");
}
