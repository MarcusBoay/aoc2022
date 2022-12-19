use std::collections::{HashSet, VecDeque};

mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|x| x.split(',').map(String::from).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    let mut coords: HashSet<(i32, i32, i32)> = HashSet::new();
    for line in data {
        let coord = (
            line[0].parse::<i32>().unwrap(),
            line[1].parse::<i32>().unwrap(),
            line[2].parse::<i32>().unwrap(),
        );
        coords.insert(coord);
    }
    get_surface_area(&coords);
    get_outer_surface_area(&coords);
}

fn get_surface_area(coords: &HashSet<(i32, i32, i32)>) {
    let mut surface_area = 0;
    for coord in coords {
        surface_area += !coords.contains(&(coord.0 - 1, coord.1, coord.2)) as i32;
        surface_area += !coords.contains(&(coord.0 + 1, coord.1, coord.2)) as i32;
        surface_area += !coords.contains(&(coord.0, coord.1 - 1, coord.2)) as i32;
        surface_area += !coords.contains(&(coord.0, coord.1 + 1, coord.2)) as i32;
        surface_area += !coords.contains(&(coord.0, coord.1, coord.2 - 1)) as i32;
        surface_area += !coords.contains(&(coord.0, coord.1, coord.2 + 1)) as i32;
    }
    println!("Total surface area: {surface_area}");
}

fn get_bounds(coords: &HashSet<(i32, i32, i32)>) -> (i32, i32, i32, i32, i32, i32) {
    let (mut x_min, mut y_min, mut z_min) = (i32::MAX, i32::MAX, i32::MAX);
    let (mut x_max, mut y_max, mut z_max) = (0, 0, 0);
    for coord in coords {
        use std::cmp::{max, min};
        x_min = min(x_min, coord.0);
        x_max = max(x_max, coord.0);
        y_min = min(y_min, coord.1);
        y_max = max(y_max, coord.1);
        z_min = min(z_min, coord.2);
        z_max = max(z_max, coord.2);
    }
    println!("{x_min}-{x_max} {y_min}-{y_max} {z_min}-{z_max}");
    (x_min, x_max, y_min, y_max, z_min, z_max)
}

fn get_outer_surface_area(coords: &HashSet<(i32, i32, i32)>) {
    let (x_min, x_max, y_min, y_max, z_min, z_max) = get_bounds(coords);
    let mut mat: Vec<Vec<Vec<char>>> =
        vec![
            vec![vec![' '; (z_max - z_min + 5) as usize]; (y_max - y_min + 5) as usize];
            (x_max - x_min + 5) as usize
        ];
    for (x, y, z) in coords {
        mat[*x as usize + 2][*y as usize + 2][*z as usize + 2] = 'x';
    }

    let mut surface_area = 0;

    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    q.push_back((0, 0, 0));
    mat[0][0][0] = 'o';
    let mut q_size = q.len();
    while q_size > 0 {
        let p_i = q.pop_back().unwrap();
        let p = (p_i.0 as usize, p_i.1 as usize, p_i.2 as usize);

        if p.0 > 0 {
            if mat[p.0 - 1][p.1][p.2] == ' ' {
                q.push_back((p_i.0 - 1, p_i.1, p_i.2));
                mat[p.0 - 1][p.1][p.2] = 'o';
            } else if mat[p.0 - 1][p.1][p.2] == 'x' {
                surface_area += 1;
            }
        }
        if p.0 < mat.len() - 1 {
            if mat[p.0 + 1][p.1][p.2] == ' ' {
                q.push_back((p_i.0 + 1, p_i.1, p_i.2));
                mat[p.0 + 1][p.1][p.2] = 'o';
            } else if mat[p.0 + 1][p.1][p.2] == 'x' {
                surface_area += 1;
            }
        }
        if p.1 > 0 {
            if mat[p.0][p.1 - 1][p.2] == ' ' {
                q.push_back((p_i.0, p_i.1 - 1, p_i.2));
                mat[p.0][p.1 - 1][p.2] = 'o';
            } else if mat[p.0][p.1 - 1][p.2] == 'x' {
                surface_area += 1;
            }
        }
        if p.1 < mat[0].len() - 1 {
            if mat[p.0][p.1 + 1][p.2] == ' ' {
                q.push_back((p_i.0, p_i.1 + 1, p_i.2));
                mat[p.0][p.1 + 1][p.2] = 'o';
            } else if mat[p.0][p.1 + 1][p.2] == 'x' {
                surface_area += 1;
            }
        }
        if p.2 > 0 {
            if mat[p.0][p.1][p.2 - 1] == ' ' {
                q.push_back((p_i.0, p_i.1, p_i.2 - 1));
                mat[p.0][p.1][p.2 - 1] = 'o';
            } else if mat[p.0][p.1][p.2 - 1] == 'x' {
                surface_area += 1;
            }
        }
        if p.2 < mat[0][0].len() - 1 {
            if mat[p.0][p.1][p.2 + 1] == ' ' {
                q.push_back((p_i.0, p_i.1, p_i.2 + 1));
                mat[p.0][p.1][p.2 + 1] = 'o';
            } else if mat[p.0][p.1][p.2 + 1] == 'x' {
                surface_area += 1;
            }
        }

        q_size = q.len();
    }
    println!("Total outer surface area: {surface_area}");
}
