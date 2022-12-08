mod utils;

struct Solution {
    map: Vec<Vec<i32>>,
    n: usize,
    m: usize,
}

impl Solution {
    pub fn new() -> Self {
        let map = utils::fast_get_file_data_as_vec()
            .iter()
            .map(|x| {
                x.chars()
                    .map(|y| y.to_digit(10).unwrap() as i32)
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        Self {
            map: map.clone(),
            n: map.len(),
            m: map[0].len(),
        }
    }

    pub fn get_visible_trees(&self) {
        let map = self.map.clone();
        let n = self.n;
        let m = self.m;
        let mut visible_map = vec![vec![false; m]; n];

        // right
        let mut c: i32;
        for i in 0..n {
            c = -1;
            for j in 0..m {
                if c < map[i][j] {
                    c = map[i][j];
                    visible_map[i][j] = true;
                }
            }
        }
        // left
        for i in 0..n {
            c = -1;
            for j in (0..m).rev() {
                if c < map[i][j] {
                    c = map[i][j];
                    visible_map[i][j] = true;
                }
            }
        }
        // down
        for i in 0..m {
            c = -1;
            for j in 0..n {
                if c < map[j][i] {
                    c = map[j][i];
                    visible_map[j][i] = true;
                }
            }
        }
        // up
        for i in 0..m {
            c = -1;
            for j in (0..n).rev() {
                if c < map[j][i] {
                    c = map[j][i];
                    visible_map[j][i] = true;
                }
            }
        }

        let mut count = 0;
        for i in 0..n {
            for j in 0..m {
                if visible_map[i][j] {
                    count += 1;
                }
            }
        }
        println!("Visible trees: {count}");
    }

    pub fn get_highest_scenic_score(&self) {
        let map = self.map.clone();
        let n = self.n;
        let m = self.m;
        let mut cur_best = 0;

        for i in 1..n - 1 {
            for j in 1..m - 1 {
                let mut views = [0, 0, 0, 0];
                // right
                let mut c = -1;
                for k in j + 1..m {
                    if map[i][k] >= map[i][j] {
                        views[0] += 1;
                        break;
                    } else if c < map[i][k] {
                        views[0] += 1;
                    }
                }
                // left
                c = -1;
                for k in (0..j).rev() {
                    if map[i][k] >= map[i][j] {
                        views[1] += 1;
                        break;
                    } else if c < map[i][k] {
                        views[1] += 1;
                    }
                }
                // down
                c = -1;
                for k in i + 1..n {
                    if map[k][j] >= map[i][j] {
                        views[2] += 1;
                        break;
                    } else if c < map[i][k] {
                        views[2] += 1;
                    }
                }
                // up
                c = -1;
                for k in (0..i).rev() {
                    if map[k][j] >= map[i][j] {
                        views[3] += 1;
                        break;
                    } else if c < map[i][k] {
                        views[3] += 1;
                    }
                }
                cur_best = std::cmp::max(cur_best, views.iter().product());
            }
        }
        println!("Highest scenic score: {cur_best}");
    }
}

fn main() {
    let soln = Solution::new();
    soln.get_visible_trees();
    soln.get_highest_scenic_score();
}
