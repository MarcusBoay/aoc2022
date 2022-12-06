mod utils;

enum MoveType {
    OneByOne,
    WholeGroup,
}

fn main() {
    let data = utils::fast_get_file_data_as_vec();

    solve(data.clone(), MoveType::OneByOne);
    solve(data, MoveType::WholeGroup);
}

fn solve(data: Vec<String>, move_type: MoveType) {
    // Get stack of boxes
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut cont_i = 0;
    for (i, line) in data.iter().enumerate() {
        if line.is_empty() {
            cont_i = i + 1;
            break;
        }

        let mut cur_stack: Vec<char> = vec![];
        for c in line.chars() {
            cur_stack.push(c);
        }
        stacks.push(cur_stack);
    }

    // Process actions
    for i in cont_i..data.len() {
        let line = data[i]
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut popped_stack: Vec<char> = vec![];
        for _ in 0..line[0] {
            let popped = stacks[line[1] - 1].pop().unwrap();
            popped_stack.push(popped);
        }
        match move_type {
            MoveType::WholeGroup => popped_stack.reverse(),
            _ => (),
        }
        for val in popped_stack {
            stacks[line[2] - 1].push(val);
        }
    }

    // Get top of stacks
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
