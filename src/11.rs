mod utils;

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Vec<String>,
    test: usize,
    t: usize,
    f: usize,
}

impl Monkey {
    pub fn new() -> Self {
        Self {
            items: vec![],
            operation: vec![],
            test: 0,
            t: 0,
            f: 0,
        }
    }
}

fn main() {
    let data = utils::fast_get_file_data_as_string()
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();
    part1(&data);
}

fn part1(data: &Vec<String>) {
    let monkeys = get_monkeys(&data);
    simulate_rounds(monkeys);
}

fn get_monkeys(data: &Vec<String>) -> Vec<Monkey> {
    let mut i = 0;
    let mut monkey = Monkey::new();
    let mut monkeys: Vec<Monkey> = vec![];
    for line in data {
        match i {
            0 => {
                monkey.items = line
                    .split(", ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
            }
            1 => {
                monkey.operation = line
                    .split_ascii_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>();
            }
            2 => {
                monkey.test = line.parse::<usize>().unwrap();
            }
            3 => {
                monkey.t = line.parse::<usize>().unwrap();
            }
            4 => {
                monkey.f = line.parse::<usize>().unwrap();
            }
            _ => {
                i = -1;
                monkeys.push(monkey.clone());
                monkey = Monkey::new();
            }
        }
        i += 1;
    }
    monkeys
}

fn simulate_rounds(mut monkeys: Vec<Monkey>) {
    let mut inspections = vec![0; monkeys.len()];
    for r in 0..10000 {
        let monkeys_len = monkeys.len();
        for i in 0..monkeys_len {
            let m = monkeys[i].clone();
            let items_len = m.items.len();

            // We already know that this monkey will inspect all items.
            // Add pre-emptively.
            inspections[i] += items_len;
            let mut j = 0;
            while j < items_len {
                // Operate on item.
                let l = if m.operation[0] == "old" {
                    m.items[j]
                } else {
                    m.operation[0].parse::<usize>().unwrap()
                };
                let r = if m.operation[2] == "old" {
                    m.items[j]
                } else {
                    m.operation[2].parse::<usize>().unwrap()
                };
                let mut res = match m.operation[1].as_str() {
                    "*" => l * r,
                    "+" => l + r,
                    _ => panic!("unexpected operation"),
                };

                // Monkey bored 😔
                res /= 3;

                // Test.
                let is_div = (res % m.test) == 0;

                // Throw.
                if is_div {
                    monkeys[m.t].items.push(res);
                } else {
                    monkeys[m.f].items.push(res);
                }

                j += 1;
            }
            monkeys[i].items = vec![];
        }

        println!("Round {r} inspection count: {:?}", inspections);
    }

    inspections.sort();
    inspections.reverse();
    println!(
        "Value of monkey business: {}",
        inspections[0] * inspections[1]
    );
}
