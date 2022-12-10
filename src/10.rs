mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    print_crt(&data);
}

fn print_crt(data: &Vec<Vec<String>>) {
    let mut i = 1;
    let mut j = 0;
    let mut stall = false;
    let mut reg = 1;
    let mut signal_str = 0;
    let mut crt: Vec<Vec<char>> = vec![];
    let mut crt_line: Vec<char> = vec![];
    while j < data.len() {
        let line = &data[j];

        // Get signal strength.
        let k = i - 20;
        if k >= 0 && k % 40 == 0 {
            signal_str += reg * i;
        }

        // CRT line.
        let k = (i - 1) % 40;
        if (-1..=40).contains(&reg) {
            if k == reg - 1 || k == reg || k == reg + 1 {
                crt_line.push('#');
            } else {
                crt_line.push('.');
            }
        }
        if i % 40 == 0 {
            crt.push(crt_line.clone());
            crt_line.clear();
        }

        // Execute instruction wrt. cycle.
        if stall {
            stall = false;
            reg += line[1].parse::<i32>().unwrap();
            j += 1;
        } else if line[0] == "noop" {
            // noop
            j += 1;
        } else if line[0] == "addx" {
            stall = true;
        }

        i += 1;
    }
    println!("Signal strength: {}", signal_str);
    for line in crt {
        println!("{:?}", line.iter().collect::<String>());
    }
}
