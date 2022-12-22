#[derive(Debug)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: u32,           // Divisible by
    true_target: usize,  // Who to throw to if test returns true
    false_target: usize, // Who to throw to if test returns false
}

fn create_monkey_set(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .filter_map(|string| {
            if let Some((_, rest)) = string.split_once('\n') {
                Some(rest)
            } else {
                None
            }
        })
        .filter_map(|string| {
            if let Some((starting_items_line, rest)) = string.split_once('\n') {
                let starting_items = starting_items_line
                    .trim()
                    .trim_start_matches("Starting items: ")
                    .split(", ")
                    .filter_map(|number| number.parse::<u32>().ok())
                    .collect::<Vec<_>>();
                Some((starting_items, rest))
            } else {
                None
            }
        })
        .filter_map(|(starting_items, string)| {
            if let Some((operation_line, rest)) = string.split_once('\n') {
                let (operator, operand) = operation_line
                    .trim_start_matches("  Operation: new = old ")
                    .split_once(' ')
                    .unwrap();
                let operation = match operator {
                    "+" => Operation::Add(operand.parse().ok().unwrap()),
                    "*" => {
                        if let Ok(operand) = operand.parse() {
                            Operation::Multiply(operand)
                        } else {
                            Operation::Square
                        }
                    }
                    _ => unreachable!(),
                };
                Some((starting_items, operation, rest))
            } else {
                None
            }
        })
        .filter_map(|(items, operation, string)| {
            if let [test, true_target, false_target] = string
                .split('\n')
                .filter(|line| !line.is_empty())
                .flat_map(|line| {
                    line.split(' ')
                        .filter_map(|string| string.parse().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()[..]
            {
                Some(Monkey {
                    items,
                    operation,
                    test: test as u32,
                    true_target,
                    false_target,
                })
            } else {
                None
            }
        })
        .collect()
}

fn evaluate_rounds(mut monkeys: Vec<Monkey>, rounds: usize) -> usize {
    let mut inspection_counts: [usize; 8] = [0; 8];
    for _ in 1..=rounds {
        for monkey_index in 0..monkeys.len() {
            let inspected_items = monkeys[monkey_index]
                .items
                .iter()
                .map(|item| {
                    let new_item = match monkeys[monkey_index].operation {
                        Operation::Add(value) => item + value,
                        Operation::Multiply(value) => item * value,
                        Operation::Square => item.pow(2),
                    };

                    new_item / 3
                })
                .collect::<Vec<_>>();
            monkeys[monkey_index].items.clear();
            inspection_counts[monkey_index] += inspected_items.len();

            // Throw items
            for item in inspected_items {
                let target = if item % monkeys[monkey_index].test == 0 {
                    monkeys[monkey_index].true_target
                } else {
                    monkeys[monkey_index].false_target
                };
                monkeys[target].items.push(item);
            }
        }
    }
    inspection_counts.sort_unstable();
    if let Some([a, b]) = inspection_counts.chunks(2).next_back() {
        a * b
    } else {
        0
    }
}

fn part_1(input: &str) -> usize {
    let monkeys = create_monkey_set(input);
    evaluate_rounds(monkeys, 20)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 11 Part 1: {}", part_1_result);
}
