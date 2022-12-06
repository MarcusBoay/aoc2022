use std::collections::HashSet;

mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_string()
        .chars()
        .collect::<Vec<char>>();

    find_marker(&data, 4);
    find_marker(&data, 14);
}

fn find_marker(data: &Vec<char>, max_no: usize) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut ss: Vec<i32> = vec![0; 27];
    loop {
        if j - i < max_no {
            ss[(data[j].to_digit(36).unwrap() - 9) as usize] += 1;
            j += 1;
        } else {
            let mut found = true;
            for n in &ss {
                if n > &1 {
                    found = false;
                }
            }
            if found {
                println!("Found after character {}", j);
                break;
            }
            ss[(data[i].to_digit(36).unwrap() - 9) as usize] -= 1;
            i += 1;
            ss[(data[j].to_digit(36).unwrap() - 9) as usize] += 1;
            j += 1;
        }
    }
}
