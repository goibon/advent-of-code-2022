enum Operation {
    Addx(i32),
    Noop,
}

fn cycle(cycle: u32, x_register: i32) -> Option<i32> {
    if cycle >= 60 && (cycle - 20) % 40 == 0 {
        Some(x_register * cycle as i32)
    } else if cycle == 20 {
        Some(x_register * 20)
    } else {
        None
    }
}

fn part_1(input: &str) -> i32 {
    let mut current_cycle: u32 = 0;
    let mut x_register = 1;
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if line.starts_with("addx") {
                let (_, value) = line.split_once(' ').unwrap();
                let value = value.parse().unwrap();
                Operation::Addx(value)
            } else {
                Operation::Noop
            }
        })
        .map(|operation| match operation {
            Operation::Addx(value) => {
                let mut cycle_results = 0;
                current_cycle += 1;
                cycle_results += cycle(current_cycle, x_register).unwrap_or(0);
                current_cycle += 1;
                cycle_results += cycle(current_cycle, x_register).unwrap_or(0);
                x_register += value;
                cycle_results
            }
            Operation::Noop => {
                current_cycle += 1;
                cycle(current_cycle, x_register).unwrap_or(0)
            }
        })
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 10 Part 1: {}", part_1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_return_sum_of_signal_strengths() {
        let input = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop\n\n";

        assert_eq!(part_1(input), 13140);
    }
}
