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
    let mut char_arr: Vec<i32> = vec![0; 26];
    loop {
        if j - i < max_no {
            char_arr[to_num(data[j])] += 1;
            j += 1;
        } else {
            let mut found = true;
            for n in &char_arr {
                if n > &1 {
                    found = false;
                }
            }
            if found {
                println!("Found after character {}", j);
                break;
            }
            char_arr[to_num(data[i])] -= 1;
            char_arr[to_num(data[j])] += 1;
            i += 1;
            j += 1;
        }
    }
}

fn to_num(c: char) -> usize {
    (c.to_digit(36).unwrap() - 10) as usize
}
